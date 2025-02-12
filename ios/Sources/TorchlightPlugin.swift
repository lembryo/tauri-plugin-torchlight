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
            print("Torch not available")
            return
        }

        do {
            try device.lockForConfiguration()

            if args.enabled {
                try device.setTorchModeOn(level: 1.0)
                device.torchMode = AVCaptureDevice.TorchMode.on
            } else {
                device.torchMode = AVCaptureDevice.TorchMode.off
            }

            device.unlockForConfiguration()
        } catch {
            print("Torch could not be used: \(error)")
        }
    }
}

@_cdecl("init_plugin_torchlight")
func initPlugin() -> Plugin {
    return TorchlightPlugin()
}
