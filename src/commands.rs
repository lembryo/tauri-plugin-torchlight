use tauri::{command, AppHandle, Runtime};

use crate::models::{sanitize_level, TorchCapabilities};
use crate::Result;
use crate::TorchlightExt;

/// Turn the torch on or off.
///
/// `level` is an optional brightness in the `0.0..=1.0` range. It is ignored
/// when turning the torch off, and on devices/platforms without brightness
/// control the torch simply turns on at full power. Non-finite values are
/// treated as "no level requested"; `0.0` means "as dim as this device allows",
/// never "off".
#[command]
pub(crate) async fn torch<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
    level: Option<f64>,
) -> Result<()> {
    app.torchlight().torch(enabled, sanitize_level(level))
}

/// Flip the torch and report the state it ended up in.
///
/// The read-and-write happens inside the native plugin, so two concurrent
/// toggles cannot both observe the same starting state.
#[command]
pub(crate) async fn toggle<R: Runtime>(app: AppHandle<R>, level: Option<f64>) -> Result<bool> {
    app.torchlight().toggle(sanitize_level(level))
}

/// Whether the device has a controllable torch.
#[command]
pub(crate) async fn is_available<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.torchlight().is_available()
}

/// Whether the torch is currently lit.
#[command]
pub(crate) async fn is_enabled<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.torchlight().is_enabled()
}

/// What this device's torch can actually do.
#[command]
pub(crate) async fn capabilities<R: Runtime>(app: AppHandle<R>) -> Result<TorchCapabilities> {
    app.torchlight().capabilities()
}

/// Desktop-only no-op stand-ins for the event-subscription commands that
/// Tauri's mobile runtime implements natively.
///
/// Without these, `addPluginListener` rejects on desktop with "command not
/// found", which forces every consumer to special-case the platform. Accepting
/// the registration and simply never emitting keeps the API uniform: the torch
/// never changes on a machine that has none.
#[cfg(desktop)]
pub(crate) mod desktop_listeners {
    use tauri::{command, ipc::Channel};

    use crate::Result;

    #[command]
    pub(crate) async fn register_listener(_event: String, _handler: Channel<()>) -> Result<()> {
        Ok(())
    }

    #[command]
    pub(crate) async fn remove_listener(_event: String, _channel_id: u32) -> Result<()> {
        Ok(())
    }
}
