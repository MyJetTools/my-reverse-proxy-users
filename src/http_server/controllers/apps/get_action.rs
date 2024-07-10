use std::{collections::BTreeMap, sync::Arc};

use my_http_server::macros::*;
use my_http_server::*;
use serde::*;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/apps",
    description: "Get All Applications and users of Environments",
    summary: "Get All Applications and users of Environments",
    controller: "Apps",
    input_data: GetAppsHttpInputModel,
    result:[
        {status_code: 200, description: "List of Applications", model:"GetAppsHttpResponse"},
    ]
)]
pub struct GetAppsAction {
    app: Arc<AppContext>,
}

impl GetAppsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetAppsAction,
    input_data: GetAppsHttpInputModel,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let data = action.app.get_env_app_users(input_data.env).await?;

    HttpOutput::as_json(GetAppsHttpResponse { data }).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct GetAppsHttpInputModel {
    #[http_query(description = "Environment ID")]
    pub env: String,
}

#[derive(MyHttpInputObjectStructure, Debug, Serialize, Deserialize)]
pub struct GetAppsHttpResponse {
    pub data: BTreeMap<String, Vec<String>>,
}
