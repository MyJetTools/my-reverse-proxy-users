use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut controllers = ControllersMiddleware::new(None, None);

    //Envs

    controllers
        .register_get_action(super::environments::GetEnvsAction::new(Arc::clone(app)).into());

    controllers
        .register_post_action(super::environments::PostEnvAction::new(Arc::clone(app)).into());

    //Apps

    controllers.register_get_action(super::apps::GetAppsAction::new(Arc::clone(app)).into());

    controllers.register_post_action(super::apps::PostAppAction::new(Arc::clone(app)).into());

    //Config

    controllers.register_get_action(super::config::GetConfigAction::new(Arc::clone(app)).into());

    controllers
}
