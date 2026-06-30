# tauri-plugin-torchlight (iOS)

The iOS half of [`tauri-plugin-torchlight`](../README.md). It controls the device
torch through `AVFoundation`'s `AVCaptureDevice` and is loaded automatically by
the plugin's Rust `mobile` bridge — you normally do not interact with this
package directly.

## What it does

- `torch` — turns the torch on/off. When turning on, an optional `level`
  (`0.0...1.0`) sets the brightness; full brightness uses
  `AVCaptureMaxAvailableTorchLevel` to avoid the throw that passing `1.0`
  directly can cause.
- `isAvailable` — reports whether the default video device has a torch.
- `isEnabled` — reports whether the torch is currently active.
- `torchModeChanged` event — emitted via KVO on `isTorchActive` so the WebView
  stays in sync when the system changes torch state (other capture sessions,
  thermal limits, etc.).

The device is always reconfigured between `lockForConfiguration()` /
`unlockForConfiguration()`, with the unlock guaranteed by a `defer` so the
device is never left locked if a torch call throws.

## Requirements

- iOS 13+ (see [`Package.swift`](Package.swift)).

## Tests

`Tests/PluginTests` contains host-runnable unit tests for the pure brightness
math (`TorchMath`). They build against the Tauri scaffolding that the example
app generates under `.tauri/`, so run them from an iOS toolchain (Xcode /
`swift test` on macOS).
