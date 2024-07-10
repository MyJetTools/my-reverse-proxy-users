use std::sync::Arc;

use my_http_server::*;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/envs",
    description: "Get List of Environments",
    summary: "Get List of Environments",
    controller: "Environments",
    result:[
        {status_code: 200, description: "List of environments", model:"Vec<String>"},
    ]
)]
pub struct GetEnvsAction {
    app: Arc<AppContext>,
}

impl GetEnvsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetEnvsAction,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = action.app.get_list_of_envs().await;
    HttpOutput::as_json(result).into_ok_result(false)
}
