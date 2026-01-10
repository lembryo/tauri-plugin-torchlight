import AVFoundation
import SwiftRs
import Tauri
import UIKit
import WebKit

class OptionArgs: Decodable {
    let enabled: Bool
}

class TorchlightPlugin: Plugin {

    @objc public func torch(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(OptionArgs.self)

        guard let device = AVCaptureDevice.default(for: .video), device.hasTorch else {
            invoke.reject("Torch not available")
            return
        }

        do {
            try device.lockForConfiguration()

            if args.enabled {
                try device.setTorchModeOn(level: 1.0)
            } else {
                device.torchMode = .off
            }

            device.unlockForConfiguration()
            invoke.resolve()
        } catch {
            invoke.reject("Torch could not be used: \(error)")
        }
    }
}

@_cdecl("init_plugin_torchlight")
func initPlugin() -> Plugin {
    return TorchlightPlugin()
}
