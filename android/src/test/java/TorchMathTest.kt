package com.lembryo.tauri.plugin.torchlight

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * Host-side unit tests for the pure torch math. Run with `./gradlew test`.
 */
class TorchMathTest {

    @Test
    fun fullBrightnessMapsToMaxStrength() {
        assertEquals(5, TorchMath.strengthFromLevel(1.0, 5))
    }

    @Test
    fun midBrightnessIsRoundedOntoTheStrengthRange() {
        assertEquals(3, TorchMath.strengthFromLevel(0.5, 5))
    }

    @Test
    fun strengthIsNeverBelowOneWhenTurningOn() {
        // 0.0 (and any tiny value) must still produce an "on" strength.
        assertEquals(1, TorchMath.strengthFromLevel(0.0, 5))
        assertEquals(1, TorchMath.strengthFromLevel(0.05, 5))
    }

    @Test
    fun levelIsClampedIntoRange() {
        assertEquals(5, TorchMath.strengthFromLevel(2.0, 5))
        assertEquals(1, TorchMath.strengthFromLevel(-1.0, 5))
    }

    @Test
    fun devicesWithoutBrightnessControlReturnPlainOn() {
        assertEquals(1, TorchMath.strengthFromLevel(0.5, 1))
        assertFalse(TorchMath.supportsBrightness(1))
        assertFalse(TorchMath.supportsBrightness(0))
    }

    @Test
    fun devicesWithBrightnessControlAreDetected() {
        assertTrue(TorchMath.supportsBrightness(2))
        assertTrue(TorchMath.supportsBrightness(10))
    }
}
