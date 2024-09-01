use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torchlight);

pub fn init<R: Runtime, C: DeserializeOwned>(_app: &AppHandle<R>, api: PluginApi<R, C>) -> crate::Result<Torchlight<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.lembryo.tauri.plugin.torchlight", "TorchlightPlugin").unwrap();
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torchlight).unwrap();
    Ok(Torchlight(handle))
}

pub struct Torchlight<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torchlight<R> {

    pub fn torch_on(&self) -> crate::Result<()> {
        self
            .0
            .run_mobile_plugin("torchOn", ())
            .map_err(Into::into)
    }

    pub fn torch_off(&self) -> crate::Result<()> {
        self
            .0
            .run_mobile_plugin("torchOff", ())
            .map_err(Into::into)
    }
}
