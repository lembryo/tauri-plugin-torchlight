use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::Error;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Torchlight<R>> {
    Ok(Torchlight(app.clone()))
}

/// Desktop implementation.
///
/// Desktop machines have no controllable camera torch, so the mutating command
/// returns a clear [`Error::Unsupported`] instead of silently pretending to
/// succeed, while the query commands report that no torch is available.
pub struct Torchlight<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Torchlight<R> {
    pub fn torch(&self, _enabled: bool, _level: Option<f64>) -> crate::Result<()> {
        Err(Error::Unsupported)
    }

    pub fn is_available(&self) -> crate::Result<bool> {
        Ok(false)
    }

    pub fn is_enabled(&self) -> crate::Result<bool> {
        Ok(false)
    }
}
