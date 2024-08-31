const COMMANDS: &[&str] = &["torch_on", "torch_off"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
