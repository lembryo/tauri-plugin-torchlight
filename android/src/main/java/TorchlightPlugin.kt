package com.lembryo.tauri.plugin.torchlight

import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin(permissions = [
    Permission(strings = [Manifest.permission.CAMERA], alias = "camera")
])
class TorchlightPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun torchOn(invoke: Invoke) {
        // TODO: Implement your plugin's logic here
        Log.d("TorchlightPlugin", "torch-on")
    }

    @Command
    fun torchOff(invoke: Invoke) {
        // TODO: Implement your plugin's logic here
        Log.d("TorchlightPlugin", "torch-off")
    }
}
