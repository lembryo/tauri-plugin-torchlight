package com.lembryo.tauri.plugin.torchlight

import android.Manifest
import android.app.Activity
import android.content.Context
import android.hardware.camera2.CameraAccessException
import android.hardware.camera2.CameraManager
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@TauriPlugin(permissions = [
    Permission(strings = [Manifest.permission.CAMERA], alias = "camera")
])
class TorchlightPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun torchOn(invoke: Invoke) {
        Log.d("TorchlightPlugin", "torch-on")

        val cameraId: String
        val cm = activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
        try {
            cameraId = cm.cameraIdList[0]
        } catch (e: CameraAccessException) {
            e.printStackTrace()
            return
        }
        cm.setTorchMode(cameraId, true)
    }

    @Command
    fun torchOff(invoke: Invoke) {
        Log.d("TorchlightPlugin", "torch-off")

        val cameraId: String
        val cm = activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
        try {
            cameraId = cm.cameraIdList[0]
        } catch (e: CameraAccessException) {
            e.printStackTrace()
            return
        }
        cm.setTorchMode(cameraId, false)
    }
}
