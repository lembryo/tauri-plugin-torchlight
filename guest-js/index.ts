import { invoke, addPluginListener, type PluginListener } from "@tauri-apps/api/core"

/** Payload delivered with the {@link onTorchModeChanged} event. */
export interface TorchModeChangedPayload {
    /** Whether the torch is currently lit. */
    enabled: boolean
    /** Whether a torch is currently usable on the device. */
    available: boolean
}

/**
 * Turn the torch on or off.
 *
 * @param enabled `true` turns the torch on, `false` turns it off.
 * @param level Optional brightness in the `0.0`–`1.0` range, used only when
 *   turning on and only on devices that support brightness control. Out-of-range
 *   values are clamped; omitting it uses full brightness.
 */
export async function setTorch(enabled: boolean, level?: number): Promise<void> {
    await invoke("plugin:torchlight|torch", { enabled, level })
}

/** Whether the device has a controllable torch. */
export async function isAvailable(): Promise<boolean> {
    return await invoke("plugin:torchlight|is_available")
}

/** Whether the torch is currently lit. */
export async function isEnabled(): Promise<boolean> {
    return await invoke("plugin:torchlight|is_enabled")
}

/**
 * Toggle the torch based on its current state.
 *
 * @returns the new state (`true` = on).
 */
export async function toggle(): Promise<boolean> {
    const next = !(await isEnabled())
    await setTorch(next)
    return next
}

/**
 * Listen for torch state changes coming from the system (another app taking the
 * camera, thermal limits, the OS turning it off, etc.).
 *
 * Remember to call `.unregister()` on the returned listener when you are done.
 */
export async function onTorchModeChanged(
    handler: (payload: TorchModeChangedPayload) => void,
): Promise<PluginListener> {
    return await addPluginListener("torchlight", "torchModeChanged", handler)
}
