use std::sync::Arc;

use my_http_server::*;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/config",
    description: "Get All configs",
    summary: "Get All configs",
    controller: "Config",
    result:[
        {status_code: 200, description: "Total Config"},
    ]
)]
pub struct GetConfigAction {
    app: Arc<AppContext>,
}

impl GetConfigAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetConfigAction,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = action.app.get_all_configs().await;
    HttpOutput::as_json(result).into_ok_result(false)
}
