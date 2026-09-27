# ProGuard/R8 rules applied to apps that consume this library.
#
# The plugin is reached reflectively from Tauri's PluginManager, so the class and
# its @Command methods must survive minification in a consuming app's release
# build.
-keep class com.lembryo.tauri.plugin.torchlight.TorchlightPlugin { *; }
-keep class com.lembryo.tauri.plugin.torchlight.TorchOptions { *; }
-keep class com.lembryo.tauri.plugin.torchlight.ToggleOptions { *; }
