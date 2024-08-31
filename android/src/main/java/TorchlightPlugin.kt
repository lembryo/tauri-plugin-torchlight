package com.lembryo.tauri.plugin.torchlight

import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class TorchlightPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun torch_on(invoke: Invoke) {
        // TODO: Implement your plugin's logic here
        Log.d("TorchlightPlugin", "torch_on")
    }

    @Command
    fun torch_off(invoke: Invoke) {
        // TODO: Implement your plugin's logic here
        Log.d("TorchlightPlugin", "torch_on")
    }
}
