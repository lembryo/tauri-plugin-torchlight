import XCTest

@testable import tauri_plugin_torchlight

final class TorchMathTests: XCTestCase {

  func testNilLevelMapsToFullBrightness() {
    XCTAssertEqual(TorchMath.clampLevel(nil), 1.0)
    XCTAssertTrue(TorchMath.isMaxLevel(TorchMath.clampLevel(nil)))
  }

  func testLevelIsClampedIntoRange() {
    XCTAssertEqual(TorchMath.clampLevel(2.0), 1.0)
    XCTAssertEqual(TorchMath.clampLevel(-1.0), TorchMath.minLevel)
  }

  /// `setTorchModeOn(level: 0.0)` raises an Objective-C exception that Swift
  /// cannot catch, so zero must never reach AVFoundation. It means "as dim as
  /// possible", matching Android's "never below an on-strength" behaviour.
  func testZeroLevelIsFlooredToTheMinimumOnLevel() {
    XCTAssertEqual(TorchMath.clampLevel(0.0), TorchMath.minLevel)
    XCTAssertGreaterThan(TorchMath.clampLevel(0.0), 0.0)
    XCTAssertFalse(TorchMath.isMaxLevel(TorchMath.clampLevel(0.0)))
  }

  func testNonFiniteLevelsFallBackToFullBrightness() {
    XCTAssertEqual(TorchMath.clampLevel(Double.nan), 1.0)
    XCTAssertEqual(TorchMath.clampLevel(Double.infinity), 1.0)
    XCTAssertEqual(TorchMath.clampLevel(-Double.infinity), 1.0)
  }

  func testMidLevelIsPreserved() {
    XCTAssertEqual(TorchMath.clampLevel(0.5), 0.5)
    XCTAssertFalse(TorchMath.isMaxLevel(0.5))
  }

  func testMaxLevelDetection() {
    XCTAssertTrue(TorchMath.isMaxLevel(1.0))
    XCTAssertFalse(TorchMath.isMaxLevel(0.99))
  }
}
