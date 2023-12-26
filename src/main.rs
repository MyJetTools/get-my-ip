use std::sync::Arc;

use app::AppContext;

mod app;
mod http;

#[tokio::main]
async fn main() {
    let app = Arc::new(AppContext::new());
    crate::http::start(&app);
    app.app_states.wait_until_shutdown().await;
}
