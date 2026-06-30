// `register_listener` / `remove_listener` are not implemented by us: they are
// provided by Tauri's mobile plugin runtime so the webview can subscribe to the
// `torchModeChanged` event via `addPluginListener`. We list them here only so
// the matching ACL permissions are generated and can be granted.
const COMMANDS: &[&str] = &[
    "torch",
    "is_available",
    "is_enabled",
    "register_listener",
    "remove_listener",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
