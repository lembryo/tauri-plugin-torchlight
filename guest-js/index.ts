import { invoke, addPluginListener, type PluginListener } from "@tauri-apps/api/core"

/** Machine-readable discriminator carried by every {@link TorchlightError}. */
export type TorchErrorKind =
    /** The platform has no torch hardware (desktop). */
    | "unsupported"
    /** This device has no usable torch. */
    | "unavailable"
    /** The camera could not be accessed (Android). */
    | "cameraAccess"
    /** The torch could not be driven, e.g. under thermal duress. */
    | "torchFailed"
    /** Anything else: IPC, serialization or a bug in the plugin. */
    | "internal"

/** Shape the Rust side serializes its errors into. */
interface RawTorchlightError {
    kind: TorchErrorKind
    message: string
}

function isRawError(value: unknown): value is RawTorchlightError {
    return (
        typeof value === "object" &&
        value !== null &&
        typeof (value as RawTorchlightError).kind === "string" &&
        typeof (value as RawTorchlightError).message === "string"
    )
}

/**
 * Error thrown by every function in this module.
 *
 * Branch on {@link TorchlightError.kind}; the message is for humans and is not
 * a stable API.
 */
export class TorchlightError extends Error {
    readonly kind: TorchErrorKind

    constructor(kind: TorchErrorKind, message: string) {
        super(message)
        this.name = "TorchlightError"
        this.kind = kind
        // Keeps `instanceof` working when the bundle targets older runtimes.
        Object.setPrototypeOf(this, TorchlightError.prototype)
    }
}

/** Normalizes whatever Tauri rejected with into a {@link TorchlightError}. */
function toTorchlightError(cause: unknown): TorchlightError {
    if (cause instanceof TorchlightError) {
        return cause
    }
    if (isRawError(cause)) {
        return new TorchlightError(cause.kind, cause.message)
    }
    // A command that was never registered, an ACL denial, or a native reject
    // that bypassed our error type all arrive as a bare string.
    return new TorchlightError("internal", typeof cause === "string" ? cause : String(cause))
}

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
        return await invoke<T>(`plugin:torchlight|${command}`, args)
    } catch (cause) {
        throw toTorchlightError(cause)
    }
}

/** Payload delivered with the {@link onTorchModeChanged} event. */
export interface TorchModeChangedPayload {
    /** Whether the torch is currently lit. */
    enabled: boolean
    /**
     * Whether the torch is usable *right now*.
     *
     * This is the runtime predicate, not {@link isAvailable}'s hardware check:
     * it goes `false` when another app takes the camera or the device is under
     * thermal duress, and back to `true` afterwards.
     */
    available: boolean
}

/** What this device's torch can actually do. */
export interface TorchCapabilities {
    /** Whether the device has a torch at all. Static for the life of the app. */
    available: boolean
    /** Whether the torch can be used right now. Can change at runtime. */
    usable: boolean
    /**
     * Whether a `level` actually changes the brightness.
     *
     * `false` on iOS never happens; on Android it is `false` below API 33 and on
     * devices that report a single strength level. When `false`, passing a
     * `level` still succeeds — the torch just turns on at full power.
     */
    brightnessSupported: boolean
    /**
     * Number of discrete brightness steps, when the device has a discrete scale
     * (Android). `null` means a continuous scale (iOS) or no brightness control.
     */
    levelSteps: number | null
}

/**
 * Turn the torch on or off.
 *
 * @param enabled `true` turns the torch on, `false` turns it off.
 * @param level Optional brightness in the `0.0`–`1.0` range, used only when
 *   turning on and only on devices where {@link TorchCapabilities.brightnessSupported}
 *   is `true`. Out-of-range and non-finite values are normalized; `0` means "as
 *   dim as this device allows", never "off". Omitting it uses full brightness.
 *
 * Calling this again with `enabled: true` and a different `level` changes the
 * brightness without turning the torch off.
 *
 * @throws {TorchlightError}
 */
export async function setTorch(enabled: boolean, level?: number): Promise<void> {
    await call<void>("torch", { enabled, level })
}

/**
 * Whether the device has torch *hardware*.
 *
 * This is a static property of the device and never changes while the app runs.
 * For "can I use it right now", read {@link TorchCapabilities.usable} or follow
 * {@link onTorchModeChanged}.
 *
 * @throws {TorchlightError}
 */
export async function isAvailable(): Promise<boolean> {
    return await call<boolean>("is_available")
}

/**
 * Whether the torch is currently lit.
 *
 * @throws {TorchlightError}
 */
export async function isEnabled(): Promise<boolean> {
    return await call<boolean>("is_enabled")
}

/**
 * What this device's torch can do — hardware presence, current usability and
 * whether brightness control has any effect.
 *
 * Use this instead of {@link isAvailable} when the UI offers a brightness
 * control, so it can be hidden or disabled on devices that ignore it.
 *
 * @throws {TorchlightError}
 */
export async function capabilities(): Promise<TorchCapabilities> {
    return await call<TorchCapabilities>("capabilities")
}

/**
 * Flip the torch and return the state it ended up in (`true` = on).
 *
 * The read-and-write happens inside the native plugin, so two concurrent
 * toggles cannot both observe the same starting state.
 *
 * @param level Optional brightness applied when the toggle turns the torch on.
 * @throws {TorchlightError}
 */
export async function toggle(level?: number): Promise<boolean> {
    return await call<boolean>("toggle", { level })
}

/**
 * Listen for torch state changes coming from the system (another app taking the
 * camera, thermal limits, the OS turning it off, etc.).
 *
 * Your own {@link setTorch} and {@link toggle} calls also fire the event.
 *
 * On desktop the listener registers successfully and simply never fires, so
 * callers do not have to special-case the platform.
 *
 * Remember to call `.unregister()` on the returned listener when you are done.
 *
 * @throws {TorchlightError}
 */
export async function onTorchModeChanged(
    handler: (payload: TorchModeChangedPayload) => void,
): Promise<PluginListener> {
    try {
        return await addPluginListener("torchlight", "torchModeChanged", handler)
    } catch (cause) {
        throw toTorchlightError(cause)
    }
}
