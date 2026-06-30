package com.lembryo.tauri.plugin.torchlight

import kotlin.math.roundToInt

/**
 * Pure helpers for the torch math, kept free of Android framework types so they
 * can be unit-tested on the host JVM.
 */
internal object TorchMath {

    /**
     * Maps a `0.0..1.0` brightness onto the discrete `1..maxLevel` strength range
     * used by [android.hardware.camera2.CameraManager.turnOnTorchWithStrengthLevel].
     *
     * The result is always clamped to a valid, on (`>= 1`) strength.
     */
    fun strengthFromLevel(level: Double, maxLevel: Int): Int {
        if (maxLevel <= 1) return 1
        val clamped = level.coerceIn(0.0, 1.0)
        val scaled = (clamped * maxLevel).roundToInt()
        return scaled.coerceIn(1, maxLevel)
    }

    /**
     * Whether a per-strength brightness level should be applied for the given
     * device maximum. Devices that report `maxLevel <= 1` have no brightness
     * control, so a plain on/off toggle must be used instead.
     */
    fun supportsBrightness(maxLevel: Int): Boolean = maxLevel > 1
}
