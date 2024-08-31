use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torchlight);

pub fn init<R: Runtime, C: DeserializeOwned>(_app: &AppHandle<R>, api: PluginApi<R, C>) -> crate::Result<Torchlight<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "TorchlightPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torchlight)?;
    Ok(Torchlight(handle))
}

pub struct Torchlight<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torchlight<R> {
    pub fn torch_on(&self) -> crate::Result<()> {
        self
            .0
            .run_mobile_plugin("torch_on")
            .map_err(Into::into)
    }

    pub fn torch_off(&self) -> crate::Result<()> {
        self
            .0
            .run_mobile_plugin("torch_off")
            .map_err(Into::into)
    }
}
