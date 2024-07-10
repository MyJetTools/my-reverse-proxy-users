use crate::app::AppError;
use my_http_server::*;

impl From<AppError> for HttpFailResult {
    fn from(value: AppError) -> Self {
        match value {
            AppError::EnvNotFound(env) => {
                HttpFailResult::as_not_found(format!("Env {} not found", env), false)
            }
        }
    }
}
