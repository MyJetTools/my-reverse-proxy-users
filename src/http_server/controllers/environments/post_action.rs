use std::sync::Arc;

use my_http_server::*;

use my_http_server::macros::*;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "POST",
    route: "/api/envs",
    description: "Add Environment",
    summary: "Add Environment",
    controller: "Environments",
    input_data: AddEnvironmentHttpInput,
    result:[
        {status_code: 204, description: "Ok result"},
    ]
)]
pub struct PostEnvAction {
    app: Arc<AppContext>,
}

impl PostEnvAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostEnvAction,
    input_data: AddEnvironmentHttpInput,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    action.app.add_env(&input_data.id).await;

    HttpOutput::Empty.into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct AddEnvironmentHttpInput {
    #[http_body(description = "Environment ID")]
    pub id: String,
}
