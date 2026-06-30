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

@TauriPlugin
class TorchlightPlugin(private val activity: Activity) : Plugin(activity) {

    private val cameraManager: CameraManager by lazy {
        activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
    }

    /** Id of a camera that actually has a flash unit (usually the rear camera), or null. */
    private val torchCameraId: String? by lazy { findTorchCameraId() }

    @Volatile
    private var torchEnabled: Boolean = false

    private val torchCallback = object : CameraManager.TorchCallback() {
        override fun onTorchModeChanged(cameraId: String, enabled: Boolean) {
            if (cameraId != torchCameraId) return
            torchEnabled = enabled
            emitTorchModeChanged(enabled = enabled, available = true)
        }

        override fun onTorchModeUnavailable(cameraId: String) {
            if (cameraId != torchCameraId) return
            torchEnabled = false
            emitTorchModeChanged(enabled = false, available = false)
        }
    }

    override fun load(webView: WebView) {
        super.load(webView)
        // Keep our cached state in sync with the system: the torch can be turned
        // off by another app, by thermal limits, or by the OS at any time.
        if (torchCameraId != null) {
            cameraManager.registerTorchCallback(torchCallback, Handler(Looper.getMainLooper()))
        }
    }

    private fun emitTorchModeChanged(enabled: Boolean, available: Boolean) {
        val payload = JSObject()
        payload.put("enabled", enabled)
        payload.put("available", available)
        trigger("torchModeChanged", payload)
    }

    private fun findTorchCameraId(): String? {
        return try {
            cameraManager.cameraIdList.firstOrNull { id ->
                cameraManager.getCameraCharacteristics(id)
                    .get(CameraCharacteristics.FLASH_INFO_AVAILABLE) == true
            }
        } catch (e: CameraAccessException) {
            null
        } catch (e: IllegalArgumentException) {
            null
        }
    }

    @Command
    fun torch(invoke: Invoke) {
        val args = invoke.parseArgs(TorchOptions::class.java)
        val cameraId = torchCameraId
        if (cameraId == null) {
            invoke.reject("Torch is not available on this device")
            return
        }

        try {
            val level = args.level
            if (args.enabled && level != null && Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                applyTorchWithBrightness(cameraId, level)
            } else {
                cameraManager.setTorchMode(cameraId, args.enabled)
            }
            invoke.resolve()
        } catch (e: CameraAccessException) {
            invoke.reject(e.message ?: "Camera access error")
        } catch (e: IllegalArgumentException) {
            // Thrown when the camera does not support the requested operation.
            invoke.reject(e.message ?: "Torch is not available on this device")
        }
    }

    private fun applyTorchWithBrightness(cameraId: String, level: Double) {
        val maxLevel = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            cameraManager.getCameraCharacteristics(cameraId)
                .get(CameraCharacteristics.FLASH_INFO_STRENGTH_MAXIMUM_LEVEL) ?: 1
        } else {
            1
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU && TorchMath.supportsBrightness(maxLevel)) {
            val strength = TorchMath.strengthFromLevel(level, maxLevel)
            cameraManager.turnOnTorchWithStrengthLevel(cameraId, strength)
        } else {
            cameraManager.setTorchMode(cameraId, true)
        }
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
}
