package com.lembryo.tauri.plugin.torchlight

import android.app.Activity
import android.content.Context
import android.hardware.camera2.CameraAccessException
import android.hardware.camera2.CameraCharacteristics
import android.hardware.camera2.CameraManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.webkit.WebView
import androidx.appcompat.app.AppCompatActivity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class TorchOptions {
    var enabled: Boolean = false
    /** Optional brightness in the 0.0..1.0 range. */
    var level: Double? = null
}

@InvokeArg
class ToggleOptions {
    /** Optional brightness applied when the toggle turns the torch on. */
    var level: Double? = null
}

/** Error codes mirrored by the Rust `Error::kind()` and the TypeScript `TorchErrorKind`. */
private object ErrorCode {
    const val UNAVAILABLE = "unavailable"
    const val CAMERA_ACCESS = "cameraAccess"
    const val TORCH_FAILED = "torchFailed"
}

@TauriPlugin
class TorchlightPlugin(private val activity: Activity) : Plugin(activity) {

    // The application context is used deliberately: CameraManager's torch
    // callbacks are registered into a process-wide singleton, so holding the
    // Activity here would outlive it.
    private val cameraManager: CameraManager by lazy {
        activity.applicationContext.getSystemService(Context.CAMERA_SERVICE) as CameraManager
    }

    /** Id of a camera that actually has a flash unit (the rear one when there is a choice), or null. */
    private val torchCameraId: String? by lazy { findTorchCameraId() }

    /** Guards the read-modify-write in [toggle] so two concurrent toggles cannot both see the same state. */
    private val torchLock = Any()

    @Volatile
    private var torchEnabled: Boolean = false

    /**
     * Whether the torch is usable right now.
     *
     * Distinct from [torchCameraId] being non-null, which is a static hardware
     * fact: another app holding the camera, or thermal duress, makes the torch
     * temporarily unusable without removing the hardware.
     */
    @Volatile
    private var torchUsable: Boolean = false

    /** Whether the system has reported the torch state at least once. */
    @Volatile
    private var stateSeeded: Boolean = false

    @Volatile
    private var callbackRegistered: Boolean = false

    private val torchCallback = object : CameraManager.TorchCallback() {
        override fun onTorchModeChanged(cameraId: String, enabled: Boolean) {
            if (cameraId != torchCameraId) return
            torchEnabled = enabled
            torchUsable = true
            stateSeeded = true
            emitTorchModeChanged(enabled = enabled, available = true)
        }

        override fun onTorchModeUnavailable(cameraId: String) {
            if (cameraId != torchCameraId) return
            torchEnabled = false
            torchUsable = false
            stateSeeded = true
            emitTorchModeChanged(enabled = false, available = false)
        }
    }

    override fun load(webView: WebView) {
        super.load(webView)
        // Keep our cached state in sync with the system: the torch can be turned
        // off by another app, by thermal limits, or by the OS at any time.
        //
        // Registering also delivers the current mode once, which is what seeds
        // the cache so `isEnabled()` is right on a cold start with the torch
        // already lit. The event that seeding emits is discarded — `load` runs
        // before the webview can register a listener, and Tauri's `trigger`
        // drops a payload when none exists.
        registerTorchCallback()
    }

    override fun onDestroy(activity: AppCompatActivity) {
        // registerTorchCallback registers into a process-scoped singleton that
        // holds this plugin (and therefore the Activity) alive. Without this the
        // Activity leaks on every configuration change, and a re-created plugin
        // would register a second callback that emits duplicate events into a
        // destroyed webview.
        unregisterTorchCallback()
        super.onDestroy(activity)
    }

    override fun onDestroy() {
        unregisterTorchCallback()
        super.onDestroy()
    }

    @Synchronized
    private fun registerTorchCallback() {
        if (callbackRegistered || torchCameraId == null) return
        cameraManager.registerTorchCallback(torchCallback, Handler(Looper.getMainLooper()))
        callbackRegistered = true
    }

    @Synchronized
    private fun unregisterTorchCallback() {
        if (!callbackRegistered) return
        cameraManager.unregisterTorchCallback(torchCallback)
        callbackRegistered = false
    }

    private fun emitTorchModeChanged(enabled: Boolean, available: Boolean) {
        val payload = JSObject()
        payload.put("enabled", enabled)
        payload.put("available", available)
        trigger("torchModeChanged", payload)
    }

    /**
     * Prefers a back-facing camera with a flash unit, falling back to any camera
     * that has one. Relying on `cameraIdList` ordering alone can pick a
     * flash-capable front or auxiliary camera on multi-camera devices.
     */
    private fun findTorchCameraId(): String? {
        return try {
            val withFlash = cameraManager.cameraIdList.filter { id ->
                cameraManager.getCameraCharacteristics(id)
                    .get(CameraCharacteristics.FLASH_INFO_AVAILABLE) == true
            }
            withFlash.firstOrNull { id ->
                cameraManager.getCameraCharacteristics(id)
                    .get(CameraCharacteristics.LENS_FACING) == CameraCharacteristics.LENS_FACING_BACK
            } ?: withFlash.firstOrNull()
        } catch (e: CameraAccessException) {
            null
        } catch (e: IllegalArgumentException) {
            null
        }
    }

    /** Device maximum torch strength, or 1 when the device has no brightness control. */
    private fun maxStrengthLevel(cameraId: String): Int {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) return 1
        return try {
            cameraManager.getCameraCharacteristics(cameraId)
                .get(CameraCharacteristics.FLASH_INFO_STRENGTH_MAXIMUM_LEVEL) ?: 1
        } catch (e: CameraAccessException) {
            1
        } catch (e: IllegalArgumentException) {
            1
        }
    }

    /**
     * Applies the requested state. Must be called with [torchLock] held by
     * callers that also read [torchEnabled].
     */
    private fun applyTorch(cameraId: String, enabled: Boolean, level: Double?) {
        if (enabled && level != null && Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            val maxLevel = maxStrengthLevel(cameraId)
            if (TorchMath.supportsBrightness(maxLevel)) {
                cameraManager.turnOnTorchWithStrengthLevel(
                    cameraId,
                    TorchMath.strengthFromLevel(level, maxLevel),
                )
            } else {
                cameraManager.setTorchMode(cameraId, true)
            }
        } else {
            cameraManager.setTorchMode(cameraId, enabled)
        }

        // The TorchCallback is the authority for external changes, but it is
        // delivered asynchronously on the main looper. Updating the cache here
        // keeps `isEnabled()` honest immediately after a successful call.
        torchEnabled = enabled
        stateSeeded = true
    }

    @Command
    fun torch(invoke: Invoke) {
        val args = invoke.parseArgs(TorchOptions::class.java)
        val cameraId = torchCameraId
        if (cameraId == null) {
            invoke.reject("Torch is not available on this device", ErrorCode.UNAVAILABLE)
            return
        }

        val result = runCatching {
            synchronized(torchLock) { applyTorch(cameraId, args.enabled, args.level) }
        }
        result.fold(
            onSuccess = { invoke.resolve() },
            onFailure = { invoke.rejectTorchFailure(it) },
        )
    }

    @Command
    fun toggle(invoke: Invoke) {
        val args = invoke.parseArgs(ToggleOptions::class.java)
        val cameraId = torchCameraId
        if (cameraId == null) {
            invoke.reject("Torch is not available on this device", ErrorCode.UNAVAILABLE)
            return
        }

        val result = runCatching {
            synchronized(torchLock) {
                val next = !torchEnabled
                applyTorch(cameraId, next, if (next) args.level else null)
                next
            }
        }
        result.fold(
            onSuccess = { next ->
                val payload = JSObject()
                payload.put("enabled", next)
                invoke.resolve(payload)
            },
            onFailure = { invoke.rejectTorchFailure(it) },
        )
    }

    @Command
    fun isAvailable(invoke: Invoke) {
        val result = JSObject()
        result.put("available", torchCameraId != null)
        invoke.resolve(result)
    }

    @Command
    fun isEnabled(invoke: Invoke) {
        val result = JSObject()
        result.put("enabled", torchEnabled)
        invoke.resolve(result)
    }

    @Command
    fun capabilities(invoke: Invoke) {
        val cameraId = torchCameraId
        val result = JSObject()
        if (cameraId == null) {
            result.put("available", false)
            result.put("usable", false)
            result.put("brightnessSupported", false)
            result.put("levelSteps", null)
            invoke.resolve(result)
            return
        }

        val maxLevel = maxStrengthLevel(cameraId)
        val brightnessSupported = TorchMath.supportsBrightness(maxLevel)
        result.put("available", true)
        // Before the first callback arrives we have no evidence the torch is
        // unusable, so report the hardware answer rather than a pessimistic one.
        result.put("usable", if (stateSeeded) torchUsable else true)
        result.put("brightnessSupported", brightnessSupported)
        result.put("levelSteps", if (brightnessSupported) maxLevel else null)
        invoke.resolve(result)
    }
}

/** Maps a thrown exception onto the shared error codes. */
private fun Invoke.rejectTorchFailure(error: Throwable) {
    when (error) {
        is CameraAccessException ->
            reject(error.message ?: "Camera access error", ErrorCode.CAMERA_ACCESS)
        // Thrown when the camera does not support the requested operation.
        is IllegalArgumentException ->
            reject(error.message ?: "Torch is not available on this device", ErrorCode.UNAVAILABLE)
        else ->
            reject(error.message ?: "The torch could not be used", ErrorCode.TORCH_FAILED)
    }
}
