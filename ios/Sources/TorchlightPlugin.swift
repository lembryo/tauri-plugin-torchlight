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

class TorchlightPlugin: Plugin {

    private var torchObservation: NSKeyValueObservation?

    /// The default video capture device, but only when it actually has a torch.
    private var torchDevice: AVCaptureDevice? {
        guard let device = AVCaptureDevice.default(for: .video), device.hasTorch else {
            return nil
        }
        return device
    }

    override func load(webview: WKWebView) {
        super.load(webview: webview)

        // Keep the JS side in sync with the system: the torch can be turned off
        // by another capture session, by thermal limits, or by the OS.
        if let device = torchDevice {
            torchObservation = device.observe(\.isTorchActive, options: [.new]) { [weak self] device, _ in
                self?.trigger(
                    "torchModeChanged",
                    data: ["enabled": device.isTorchActive, "available": device.isTorchAvailable]
                )
            }
        }
    }

    deinit {
        torchObservation?.invalidate()
    }

    @objc public func torch(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(TorchOptions.self)

        guard let device = torchDevice else {
            invoke.reject("Torch is not available on this device")
            return
        }

        do {
            try device.lockForConfiguration()
            // Ensure the device is always unlocked, even if a `try` below throws.
            defer { device.unlockForConfiguration() }

            if args.enabled {
                let level = TorchMath.clampLevel(args.level)
                if TorchMath.isMaxLevel(level) {
                    // Passing exactly 1.0 can throw; the dedicated constant must be used.
                    try device.setTorchModeOn(level: AVCaptureMaxAvailableTorchLevel)
                } else {
                    try device.setTorchModeOn(level: level)
                }
            } else {
                device.torchMode = .off
            }

            invoke.resolve()
        } catch {
            // e.g. AVErrorTorchLevelUnavailable under thermal pressure.
            invoke.reject("Torch could not be used: \(error.localizedDescription)")
        }
    }

    @objc public func isAvailable(_ invoke: Invoke) throws {
        invoke.resolve(["available": torchDevice != nil])
    }

    @objc public func isEnabled(_ invoke: Invoke) throws {
        invoke.resolve(["enabled": torchDevice?.isTorchActive ?? false])
    }
}

@_cdecl("init_plugin_torchlight")
func initPlugin() -> Plugin {
    return TorchlightPlugin()
}
