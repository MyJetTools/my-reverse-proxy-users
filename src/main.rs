use std::sync::Arc;

use app::AppContext;

mod app;
mod http_server;
mod settings;

#[tokio::main]
async fn main() {
    let settings = crate::settings::SettingsModel::new().await;
    let app_ctx = AppContext::new(settings);

    let app_ctx = Arc::new(app_ctx);

    crate::http_server::set_up(&app_ctx).await;

    app_ctx.states.wait_until_shutdown().await;
}
