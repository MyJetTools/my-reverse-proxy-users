use std::{net::SocketAddr, sync::Arc};

use my_http_server::{controllers::swagger::SwaggerMiddleware, MyHttpServer};

use crate::app::*;

pub async fn set_up(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    let controllers = Arc::new(super::controllers::build(app));

    let swagger_middleware = SwaggerMiddleware::new(controllers.clone(), APP_NAME, APP_VERSION);

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.add_middleware(controllers);

    http_server.start(app.states.clone(), my_logger::LOGGER.clone());
}
