import Foundation

/// Pure helpers for torch math, kept free of AVFoundation/UIKit so they can be
/// unit-tested without a device or simulator.
public enum TorchMath {

  /// Lowest level that still counts as "on".
  ///
  /// `AVCaptureDevice.setTorchModeOn(level:)` raises `NSInvalidArgumentException`
  /// for exactly `0.0` — an Objective-C exception, which Swift `do`/`catch`
  /// cannot catch, so it would terminate the process. Android maps a requested
  /// `0.0` onto its minimum "on" strength; flooring here gives both platforms
  /// the same contract: `0` means "as dim as possible", never "off".
  public static let minLevel: Float = 0.01

  /// Clamps an optional `0.0...1.0` brightness into a level that is always safe
  /// to hand to `setTorchModeOn(level:)`.
  ///
  /// `nil` and non-finite values mean "no explicit level requested" and map to
  /// full brightness.
  public static func clampLevel(_ level: Double?) -> Float {
    guard let level = level else { return 1.0 }
    let value = Float(level)
    guard value.isFinite else { return 1.0 }
    return min(max(value, minLevel), 1.0)
  }

  /// Whether the requested level is effectively "maximum", in which case the
  /// caller should use `AVCaptureMaxAvailableTorchLevel` instead of passing
  /// `1.0` to `setTorchModeOn(level:)` (which can throw).
  public static func isMaxLevel(_ level: Float) -> Bool {
    return level >= 1.0
  }
}
