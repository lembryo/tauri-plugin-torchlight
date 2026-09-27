use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};
pub use models::{
    AvailabilityResponse, EnabledResponse, ToggleRequest, TorchCapabilities, TorchRequest,
    MIN_LEVEL,
};

use crate::commands::{capabilities, is_available, is_enabled, toggle, torch};

#[cfg(desktop)]
use desktop::Torchlight;
#[cfg(mobile)]
use mobile::Torchlight;

pub trait TorchlightExt<R: Runtime> {
    fn torchlight(&self) -> &Torchlight<R>;
}

impl<R: Runtime, T: Manager<R>> TorchlightExt<R> for T {
    fn torchlight(&self) -> &Torchlight<R> {
        self.state::<Torchlight<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let builder = Builder::new("torchlight");

    // On mobile, `register_listener` / `remove_listener` must stay unhandled here
    // so Tauri forwards them to the native plugin, which implements them. On
    // desktop nothing would handle them, so no-op stand-ins are registered to
    // keep `addPluginListener` from rejecting with "command not found".
    #[cfg(mobile)]
    let builder = builder.invoke_handler(tauri::generate_handler![
        torch,
        toggle,
        is_available,
        is_enabled,
        capabilities,
    ]);
    #[cfg(desktop)]
    let builder = builder.invoke_handler(tauri::generate_handler![
        torch,
        toggle,
        is_available,
        is_enabled,
        capabilities,
        commands::desktop_listeners::register_listener,
        commands::desktop_listeners::remove_listener,
    ]);

    builder
        .setup(|app, api| {
            #[cfg(mobile)]
            let torchlight = mobile::init(app, api)?;
            #[cfg(desktop)]
            let torchlight = desktop::init(app, api)?;
            app.manage(torchlight);
            Ok(())
        })
        .build()
}
