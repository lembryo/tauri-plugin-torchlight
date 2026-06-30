# Tauri Plugin torchlight

A Tauri v2 plugin to control a smartphone's flashlight (torch): turn it on/off,
set its brightness, query availability and current state, and react to system
torch changes.

Designed for mobile applications built with **Tauri v2** (Android & iOS).

## Features

- Turn the torch on or off.
- Optional **brightness** control (`0.0`–`1.0`) on devices that support it
  (iOS, and Android 13+).
- **Availability** check (`isAvailable`) — only the camera that actually has a
  flash unit is used.
- **State** query (`isEnabled`) and a **`torchModeChanged`** event so your UI
  stays in sync when the OS, another app, or thermal limits change the torch.
- Graceful, explicit behaviour on desktop (commands return a clear
  "unsupported" error instead of silently doing nothing).
- Cross-platform: Android (`CameraManager`) and iOS (`AVFoundation`).

## Installation

### Rust Setup

Add the plugin to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-torchlight = { git = "https://github.com/lembryo/tauri-plugin-torchlight.git" }
```

### JavaScript/TypeScript Setup

Install the API bindings (optional but recommended — gives you typed helpers
instead of raw `invoke` calls):

```bash
npm install tauri-plugin-torchlight-api
# or: pnpm add / yarn add
```

## Usage

Register the plugin in `src-tauri/src/lib.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_torchlight::init()) // Add this line
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Grant the permission in `src-tauri/capabilities/default.json`:

```json
{
    "permissions": [
        "torchlight:default"
    ]
}
```

The `torchlight:default` set enables `torch`, `is_available` and `is_enabled`.
You can also grant them individually (`torchlight:allow-torch`, etc.).

### Using the TypeScript API

```typescript
import {
    setTorch,
    isAvailable,
    isEnabled,
    toggle,
    onTorchModeChanged,
} from "tauri-plugin-torchlight-api"

// Check support before showing a torch button.
if (await isAvailable()) {
    await setTorch(true)            // full brightness
    await setTorch(true, 0.5)       // half brightness where supported
    const lit = await isEnabled()   // -> true
    await toggle()                  // flips the current state, returns the new one
    await setTorch(false)           // off
}

// Stay in sync when the system changes the torch.
const listener = await onTorchModeChanged(({ enabled, available }) => {
    console.log("torch:", enabled, "available:", available)
})
// later: listener.unregister()
```

### Without the API package (raw `invoke`)

```javascript
import { invoke } from "@tauri-apps/api/core"

invoke("plugin:torchlight|torch", { enabled: true, level: 0.5 })
    .then(() => {
        // success
    })
    .catch((e) => console.error(e))

const available = await invoke("plugin:torchlight|is_available")
const enabled = await invoke("plugin:torchlight|is_enabled")
```

## Platform notes

- **Android**: uses `CameraManager.setTorchMode`; brightness uses
  `turnOnTorchWithStrengthLevel` on API 33+. Requires `minSdk 23`. No runtime
  permission is needed; the plugin only declares
  `android.hardware.camera.flash` as a non-required feature.
- **iOS**: uses `AVCaptureDevice`; full brightness uses
  `AVCaptureMaxAvailableTorchLevel`. Requires iOS 13+.
- **Desktop**: there is no hardware torch — `torch` returns an "unsupported"
  error and `isAvailable`/`isEnabled` return `false`.

## Development

```bash
# Rust core + unit tests
cargo test

# JavaScript bindings
npm install
npm run build

# Android unit tests (host JVM)
cd android && ./gradlew test
```

## License

This plugin is licensed under the MIT or Apache 2.0 license, at your option.
