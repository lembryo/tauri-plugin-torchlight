use tauri::{command, AppHandle, Runtime};

use crate::Result;
use crate::TorchlightExt;

#[command]
pub(crate) async fn torch<R: Runtime>(app: AppHandle<R>, enabled: bool) -> Result<()> {
    app.torchlight().torch(enabled)
}
