use std::{collections::BTreeMap, sync::Arc};

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::settings::SettingsModel;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub states: Arc<AppStates>,
    pub settings: Mutex<SettingsModel>,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self {
            states: Arc::new(AppStates::create_initialized()),
            settings: Mutex::new(settings),
        }
    }

    pub async fn get_list_of_envs(&self) -> Vec<String> {
        let settings = self.settings.lock().await;
        settings.environments.keys().cloned().collect()
    }

    pub async fn add_env(&self, env: &str) {
        let env = env.to_lowercase();

        let mut settings = self.settings.lock().await;

        if settings.environments.contains_key(&env) {
            return;
        }

        settings
            .environments
            .insert(env.to_string(), BTreeMap::new());

        settings.save().await;
    }

    pub async fn add_app_user(&self, env: &str, app: &str, user: String) -> Result<(), AppError> {
        let mut settings = self.settings.lock().await;

        let env = settings
            .environments
            .get_mut(env)
            .ok_or(AppError::EnvNotFound(env.to_string()))?;

        if !env.contains_key(app) {
            env.insert(app.to_string(), vec![user]);
            settings.save().await;
            return Ok(());
        }

        let env = env.get_mut(app).unwrap();

        if env.contains(&user) {
            return Ok(());
        }

        env.push(user);
        settings.save().await;
        Ok(())
    }

    pub async fn get_env_app_users(
        &self,
        env: String,
    ) -> Result<BTreeMap<String, Vec<String>>, AppError> {
        let settings = self.settings.lock().await;

        let env = settings
            .environments
            .get(env.as_str())
            .ok_or(AppError::EnvNotFound(env))?;

        Ok(env.clone())
    }

    pub async fn get_all_configs(&self) -> SettingsModel {
        self.settings.lock().await.clone()
    }
}

#[derive(Debug)]
pub enum AppError {
    EnvNotFound(String),
}
