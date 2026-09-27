// `register_listener` / `remove_listener` are not implemented by us on mobile:
// they are provided by Tauri's mobile plugin runtime so the webview can
// subscribe to the `torchModeChanged` event via `addPluginListener`. On desktop
// the plugin registers no-op stand-ins so the same call succeeds there too.
// Either way they are listed here so the matching ACL permissions are generated
// and can be granted.
const COMMANDS: &[&str] = &[
    "torch",
    "toggle",
    "is_available",
    "is_enabled",
    "capabilities",
    "register_listener",
    "remove_listener",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
