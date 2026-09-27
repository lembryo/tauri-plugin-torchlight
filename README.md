# Tauri Plugin torchlight

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-torchlight.svg)](https://crates.io/crates/tauri-plugin-torchlight)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-torchlight)](https://docs.rs/tauri-plugin-torchlight)
[![npm](https://img.shields.io/npm/v/tauri-plugin-torchlight-api.svg)](https://www.npmjs.com/package/tauri-plugin-torchlight-api)
[![license](https://img.shields.io/crates/l/tauri-plugin-torchlight.svg)](#license)

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

This plugin ships as two packages that are released together and share the same
version number:

| Package | Registry | Purpose |
| --- | --- | --- |
| `tauri-plugin-torchlight` | [crates.io](https://crates.io/crates/tauri-plugin-torchlight) | the Rust plugin itself |
| `tauri-plugin-torchlight-api` | [npm](https://www.npmjs.com/package/tauri-plugin-torchlight-api) | typed JavaScript/TypeScript bindings |

### Rust Setup

```bash
cargo add tauri-plugin-torchlight
```

Or add it to your `src-tauri/Cargo.toml` directly:

```toml
[dependencies]
tauri-plugin-torchlight = "1"
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

The `torchlight:default` set enables `torch`, `is_available`, `is_enabled`,
`register_listener` and `remove_listener`. You can also grant them individually
(`torchlight:allow-torch`, etc.) — note that `onTorchModeChanged` needs both
`allow-register-listener` and `allow-remove-listener`.

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

## Release

Publishing is automated by the `publish` GitHub Actions workflow
(`.github/workflows/publish.yml`), which runs on every push to `main`:

1. Bump the version in **both** `Cargo.toml` and `package.json` (keep them in
   sync) and merge into `main`.
2. The workflow publishes the crate to crates.io and the bindings to npm.

Versions that are already published are skipped, so merging into `main` without
a version bump is a no-op. The workflow needs two repository secrets:
`CARGO_TOKEN` (crates.io) and `NPM_TOKEN` (npm).

## License

This plugin is licensed under the MIT or Apache 2.0 license, at your option.
