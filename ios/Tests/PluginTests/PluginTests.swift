import XCTest

@testable import tauri_plugin_torchlight

final class TorchMathTests: XCTestCase {

    func testNilLevelMapsToFullBrightness() {
        XCTAssertEqual(TorchMath.clampLevel(nil), 1.0)
        XCTAssertTrue(TorchMath.isMaxLevel(TorchMath.clampLevel(nil)))
    }

    func testLevelIsClampedIntoRange() {
        XCTAssertEqual(TorchMath.clampLevel(2.0), 1.0)
        XCTAssertEqual(TorchMath.clampLevel(-1.0), 0.0)
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
