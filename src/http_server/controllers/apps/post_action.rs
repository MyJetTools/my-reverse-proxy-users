use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "POST",
    route: "/api/apps",
    description: "Add User for App in Environment",
    summary: "Add User for App in Environment",
    controller: "Apps",
    input_data: AddAppUserHttpInputModel,
    result:[
        {status_code: 204, description: "Added Ok"},
    ]
)]
pub struct PostAppAction {
    app: Arc<AppContext>,
}

impl PostAppAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &PostAppAction,
    input_data: AddAppUserHttpInputModel,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    action
        .app
        .add_app_user(input_data.env.as_str(), &input_data.app, input_data.user)
        .await?;

    HttpOutput::Empty.into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct AddAppUserHttpInputModel {
    #[http_body(description = "Environment ID")]
    pub env: String,

    #[http_body(description = "App ID")]
    pub app: String,

    #[http_body(description = "User ID")]
    pub user: String,
}
