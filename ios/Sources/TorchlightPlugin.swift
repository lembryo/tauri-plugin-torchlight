import AVFoundation
import SwiftRs
import Tauri
import UIKit
import WebKit

class TorchOptions: Decodable {
  let enabled: Bool
  /// Optional brightness in the 0.0...1.0 range.
  let level: Double?
}

class ToggleOptions: Decodable {
  /// Optional brightness applied when the toggle turns the torch on.
  let level: Double?
}

/// Error codes mirrored by the Rust `Error::kind()` and the TypeScript `TorchErrorKind`.
private enum ErrorCode {
  static let unavailable = "unavailable"
  static let torchFailed = "torchFailed"
}

class TorchlightPlugin: Plugin {

  private var activeObservation: NSKeyValueObservation?
  private var availableObservation: NSKeyValueObservation?

  /// Serializes the read-modify-write in `toggle` and the configuration lock,
  /// so two concurrent invocations cannot both observe the same starting state
  /// or unbalance each other's `lockForConfiguration`.
  private let torchQueue = DispatchQueue(label: "com.lembryo.tauri.plugin.torchlight")

  /// The default video capture device, but only when it actually has a torch.
  private var torchDevice: AVCaptureDevice? {
    guard let device = AVCaptureDevice.default(for: .video), device.hasTorch else {
      return nil
    }
    return device
  }

  override func load(webview: WKWebView) {
    super.load(webview: webview)

    // Keep the JS side in sync with the system: the torch can be turned off by
    // another capture session, by thermal limits, or by the OS. Availability is
    // observed separately because it can change while the torch is off, which
    // `isTorchActive` alone would never report.
    if let device = torchDevice {
      activeObservation = device.observe(\.isTorchActive, options: [.new]) { [weak self] device, _ in
        self?.emitTorchModeChanged(device)
      }
      availableObservation = device.observe(\.isTorchAvailable, options: [.new]) {
        [weak self] device, _ in
        self?.emitTorchModeChanged(device)
      }
      // Deliberately no emit here: `load` runs before the webview can register a
      // listener, and Tauri's `trigger` drops a payload when none exists. iOS
      // reads its state straight off the device, so there is nothing to seed
      // either. Callers get their initial state from `isEnabled`/`capabilities`.
    }
  }

  deinit {
    activeObservation?.invalidate()
    availableObservation?.invalidate()
  }

  private func emitTorchModeChanged(_ device: AVCaptureDevice) {
    trigger(
      "torchModeChanged",
      data: ["enabled": device.isTorchActive, "available": device.isTorchAvailable]
    )
  }

  /// Applies the requested state. Callers must already be on `torchQueue`.
  private func applyTorch(_ device: AVCaptureDevice, enabled: Bool, level: Double?) throws {
    try device.lockForConfiguration()
    // Ensure the device is always unlocked, even if a `try` below throws.
    defer { device.unlockForConfiguration() }

    if enabled {
      let clamped = TorchMath.clampLevel(level)
      if TorchMath.isMaxLevel(clamped) {
        // Passing exactly 1.0 can throw; the dedicated constant must be used.
        try device.setTorchModeOn(level: AVCaptureMaxAvailableTorchLevel)
      } else {
        try device.setTorchModeOn(level: clamped)
      }
    } else {
      device.torchMode = .off
    }
  }

  @objc public func torch(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(TorchOptions.self)

    guard let device = torchDevice else {
      invoke.reject("Torch is not available on this device", code: ErrorCode.unavailable)
      return
    }

    torchQueue.sync {
      do {
        try applyTorch(device, enabled: args.enabled, level: args.level)
      } catch {
        // e.g. AVErrorTorchLevelUnavailable under thermal pressure.
        invoke.reject(
          "Torch could not be used: \(error.localizedDescription)",
          code: ErrorCode.torchFailed
        )
        return
      }
      invoke.resolve()
    }
  }

  @objc public func toggle(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ToggleOptions.self)

    guard let device = torchDevice else {
      invoke.reject("Torch is not available on this device", code: ErrorCode.unavailable)
      return
    }

    torchQueue.sync {
      let next = !device.isTorchActive
      do {
        try applyTorch(device, enabled: next, level: next ? args.level : nil)
      } catch {
        invoke.reject(
          "Torch could not be used: \(error.localizedDescription)",
          code: ErrorCode.torchFailed
        )
        return
      }
      invoke.resolve(["enabled": next])
    }
  }

  @objc public func isAvailable(_ invoke: Invoke) throws {
    invoke.resolve(["available": torchDevice != nil])
  }

  @objc public func isEnabled(_ invoke: Invoke) throws {
    invoke.resolve(["enabled": torchDevice?.isTorchActive ?? false])
  }

  @objc public func capabilities(_ invoke: Invoke) throws {
    guard let device = torchDevice else {
      invoke.resolve([
        "available": false,
        "usable": false,
        "brightnessSupported": false,
        "levelSteps": NSNull(),
      ])
      return
    }

    // iOS exposes a continuous 0...1 torch level on every device with a torch,
    // so there is no discrete step count to report.
    invoke.resolve([
      "available": true,
      "usable": device.isTorchAvailable,
      "brightnessSupported": true,
      "levelSteps": NSNull(),
    ])
  }
}

@_cdecl("init_plugin_torchlight")
func initPlugin() -> Plugin {
  return TorchlightPlugin()
}
