# Tauri Plugin torchlight

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-torchlight.svg)](https://crates.io/crates/tauri-plugin-torchlight)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-torchlight)](https://docs.rs/tauri-plugin-torchlight)
[![npm](https://img.shields.io/npm/v/tauri-plugin-torchlight-api.svg)](https://www.npmjs.com/package/tauri-plugin-torchlight-api)
[![license](https://img.shields.io/crates/l/tauri-plugin-torchlight.svg)](#license)

A Tauri v2 plugin to control a smartphone's flashlight (torch): turn it on/off,
set its brightness, query availability, capabilities and current state, and
react to system torch changes.

Designed for mobile applications built with **Tauri v2** (Android & iOS).

📖 **Documentation:** [English](https://lembryo.github.io/tauri-plugin-torchlight/en/) ·
[日本語](https://lembryo.github.io/tauri-plugin-torchlight/ja/)

## Features

- Turn the torch on or off (`setTorch`), or flip it with a single **atomic,
  natively implemented** `toggle()` that returns the resulting state.
- Optional **brightness** control (`0.0`–`1.0`) on devices that support it
  (iOS, and Android 13+ devices that expose more than one strength level).
- **Capabilities** probing (`capabilities()`), which separates "this device has
  torch hardware" from "the torch is usable right now" and tells you whether a
  `level` has any effect at all.
- **State** query (`isEnabled`) and a **`torchModeChanged`** event so your UI
  stays in sync when the OS, another app, or thermal limits change the torch.
- **Typed errors**: a `TorchlightError` with a machine-readable `.kind`, so you
  never have to parse English prose.
- Graceful, explicit behaviour on desktop (mutating commands return a clear
  `unsupported` error instead of silently doing nothing, and the event listener
  still registers so callers need no platform special-casing).
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
and typed errors instead of raw `invoke` calls):

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

The `torchlight:default` set grants all seven permissions:

| Permission | Enables |
| --- | --- |
| `allow-torch` | `setTorch()` |
| `allow-toggle` | `toggle()` |
| `allow-is-available` | `isAvailable()` |
| `allow-is-enabled` | `isEnabled()` |
| `allow-capabilities` | `capabilities()` |
| `allow-register-listener` | `onTorchModeChanged()` |
| `allow-remove-listener` | `listener.unregister()` |

You can also grant them individually (`torchlight:allow-torch`, etc.) — note
that `onTorchModeChanged` needs both `allow-register-listener` and
`allow-remove-listener`.

### Using the TypeScript API

```typescript
import {
    setTorch,
    isAvailable,
    isEnabled,
    capabilities,
    toggle,
    onTorchModeChanged,
    TorchlightError,
    type TorchCapabilities,
    type TorchModeChangedPayload,
    type TorchErrorKind,
} from "tauri-plugin-torchlight-api"

// Check support before showing a torch button.
if (await isAvailable()) {
    await setTorch(true)            // full brightness
    await setTorch(true, 0.5)       // half brightness where supported
    const lit = await isEnabled()   // -> true
    const now = await toggle()      // flips the state natively, returns the new one
    await toggle(0.3)               // the level is applied if the flip turns it on
    await setTorch(false)           // off
}

// Stay in sync when the system changes the torch.
const listener = await onTorchModeChanged(({ enabled, available }) => {
    console.log("torch:", enabled, "usable right now:", available)
})
// later: listener.unregister()
```

### Availability vs. usability

`isAvailable()` and `TorchCapabilities.available` answer a **static hardware**
question: does this device have a torch at all? The answer never changes while
the app runs.

`TorchCapabilities.usable` — and the `available` field of the
`TorchModeChangedPayload` — answer a **runtime** question: can the torch be
driven *right now*? It goes `false` when another app takes the camera or the
device is under thermal duress, and back to `true` afterwards.

Drive "should this device show a torch button at all?" from `available`, and
"should the button be enabled at this moment?" from `usable` plus the event.

```typescript
const caps: TorchCapabilities = await capabilities()
// {
//   available: boolean            // hardware presence, static
//   usable: boolean               // works right now, can change
//   brightnessSupported: boolean  // does `level` do anything?
//   levelSteps: number | null     // discrete steps (Android), null = continuous/none
// }

if (caps.brightnessSupported) {
    // Show a brightness slider; `levelSteps` can size it on Android.
} else {
    // Hide it: the torch will always turn on at full power.
}
```

`brightnessSupported` is always `true` on iOS. On Android it is `false` below
API 33, and also on API 33+ devices that report only a single strength level.
When it is `false`, passing a `level` still succeeds — the torch just turns on
at full power.

### The `level` contract

- Range is `0.0`–`1.0`; out-of-range values are **clamped**, not rejected.
- `0` means **"as dim as this device allows"**, never "off". Use
  `setTorch(false)` to turn the torch off.
- Non-finite values (`NaN`, `±Infinity`) are treated as "no level requested",
  i.e. full brightness.
- Omitting `level` also means full brightness.
- Calling `setTorch(true, level)` again while the torch is already on changes
  the brightness **without** cycling the torch off and on.

### Error handling

Every function rejects with a `TorchlightError`. Branch on `.kind`; the
`.message` is for humans and is not a stable API.

```typescript
import { setTorch, TorchlightError } from "tauri-plugin-torchlight-api"

try {
    await setTorch(true, 0.5)
} catch (cause) {
    if (cause instanceof TorchlightError) {
        switch (cause.kind) {
            case "unsupported":   // desktop: there is no torch hardware
            case "unavailable":   // this device has no usable torch
            case "cameraAccess":  // the camera could not be accessed (Android)
            case "torchFailed":   // the torch could not be driven (e.g. thermal)
            case "internal":      // IPC, serialization, ACL denial, or a bug
                console.error(cause.kind, cause.message)
        }
    }
}
```

The bindings normalize everything Tauri can reject with — including bare
strings from an unregistered command or an ACL denial — into a
`TorchlightError`, so `instanceof` always holds.

### Events on desktop

`onTorchModeChanged` registers successfully on desktop and simply never fires,
so consumers do not need to special-case the platform.

### Without the API package (raw `invoke`)

```javascript
import { invoke } from "@tauri-apps/api/core"

await invoke("plugin:torchlight|torch", { enabled: true, level: 0.5 })
const enabledNow = await invoke("plugin:torchlight|toggle", { level: 0.5 })
const available = await invoke("plugin:torchlight|is_available")
const enabled = await invoke("plugin:torchlight|is_enabled")
const caps = await invoke("plugin:torchlight|capabilities")
```

Raw `invoke` rejects with the serialized `{ kind, message }` object rather than
a `TorchlightError` instance.

## Platform notes

- **Android**: uses `CameraManager.setTorchMode`; brightness uses
  `turnOnTorchWithStrengthLevel` on API 33+. Requires `minSdk 23`. No runtime
  permission is needed; the plugin only declares
  `android.hardware.camera.flash` as a non-required feature.
  - The plugin prefers a **back-facing** camera that has a flash unit, falling
    back to any camera with one, instead of trusting `cameraIdList` ordering
    (which can pick a flash-capable front or auxiliary camera).
  - A `CameraManager.TorchCallback` is registered on load (which also seeds the
    initial state) and **unregistered on destroy**, so the process-wide
    callback registry does not keep the Activity alive or deliver duplicate
    events after a configuration change.
  - The cached state is updated synchronously after a successful call, so
    `isEnabled()` is accurate immediately rather than only after the
    asynchronous callback arrives.
- **iOS**: uses `AVCaptureDevice`; full brightness uses
  `AVCaptureMaxAvailableTorchLevel`. Requires iOS 13+.
  - KVO observes **both** `isTorchActive` and `isTorchAvailable`, because
    availability can change while the torch is off — which `isTorchActive`
    alone would never report.
  - Nothing is emitted at `load()`: it runs before the webview can register a
    listener, and Tauri drops an event payload when none exists. Read the
    initial state with `isEnabled()` / `capabilities()`.
  - See [`ios/README.md`](ios/README.md) for the native details.
- **Desktop**: there is no hardware torch — `torch` and `toggle` return an
  `unsupported` error, `isAvailable`/`isEnabled` return `false`, `capabilities`
  reports everything as unavailable, and the event listener registers but never
  fires.

## Development

```bash
# Rust core + unit tests
cargo test

# JavaScript bindings
npm install
npm run build
npm run typecheck   # tsc --noEmit
npm test            # pretest runs the rollup build, then tsc --noEmit
```

### Android unit tests

This repository does **not** contain a Gradle wrapper, and the standalone
`android/` project additionally needs the `.tauri/tauri-api` scaffolding that
only the Tauri CLI generates. Run the tests from a host app's generated Android
project instead — for example, from the driver app that consumes this plugin as
a submodule:

```bash
cd src-tauri/gen/android && ./gradlew :tauri-plugin-torchlight:test
```

### iOS unit tests

See [`ios/README.md`](ios/README.md).

## Release

Publishing is automated by the `publish` GitHub Actions workflow
([`.github/workflows/publish.yml`](.github/workflows/publish.yml)), which runs
on every push to `main` (and can be dispatched manually).

1. Bump the version in **both** `Cargo.toml` and `package.json` — they are
   published as a pair and the workflow fails if they disagree — and merge into
   `main`.
2. A **test gate** runs first: a `Cargo.toml` / `package.json` version
   consistency check, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
   `npm ci` and `npm run build`. (`npm ci` relies on the committed
   `package-lock.json`. `Cargo.lock` is deliberately *not* committed — a library
   leaves dependency resolution to its consumers.)
3. Only if the gate passes, the workflow publishes **npm first**, and then
   crates.io once npm has succeeded. npm is the reversible half of the release
   (it allows unpublishing within 72h, while a crates.io yank does not
   un-publish), so a failure cannot leave a half-release with a crate that
   points at bindings nobody can install.

Versions that are already published are skipped, so merging into `main` without
a version bump is a no-op. The workflow needs two repository secrets:
`CARGO_TOKEN` (crates.io) and `NPM_TOKEN` (npm).

## License

This plugin is licensed under the MIT or Apache 2.0 license, at your option.
