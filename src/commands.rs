use tauri::{command, AppHandle, Runtime};

use crate::Result;
use crate::TorchlightExt;

/// Turn the torch on or off.
///
/// `level` is an optional brightness in the `0.0..=1.0` range. It is ignored
/// when turning the torch off, and on devices/platforms without brightness
/// control the torch simply turns on at full power.
#[command]
pub(crate) async fn torch<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
    level: Option<f64>,
) -> Result<()> {
    app.torchlight().torch(enabled, level)
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
