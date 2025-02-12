use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Torchlight<R>> {
    Ok(Torchlight(app.clone()))
}

pub struct Torchlight<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Torchlight<R> {
    pub fn torch(&self, enabled: bool) -> crate::Result<()> {
        println!("torch {}", enabled);
        Ok(())
    }
}
