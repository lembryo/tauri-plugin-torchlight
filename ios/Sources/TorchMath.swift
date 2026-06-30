import Foundation

/// Pure helpers for torch math, kept free of AVFoundation/UIKit so they can be
/// unit-tested without a device or simulator.
public enum TorchMath {

    /// Clamps an optional `0.0...1.0` brightness into a valid `Float` level.
    ///
    /// `nil` means "no explicit level requested" and maps to full brightness.
    public static func clampLevel(_ level: Double?) -> Float {
        let value = Float(level ?? 1.0)
        return min(max(value, 0.0), 1.0)
    }

    /// Whether the requested level is effectively "maximum", in which case the
    /// caller should use `AVCaptureMaxAvailableTorchLevel` instead of passing
    /// `1.0` to `setTorchModeOn(level:)` (which can throw).
    public static func isMaxLevel(_ level: Float) -> Bool {
        return level >= 1.0
    }
}
