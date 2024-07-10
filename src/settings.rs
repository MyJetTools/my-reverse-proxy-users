use std::collections::BTreeMap;

use my_http_server::macros::MyHttpInputObjectStructure;
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "~/.my-reverse-proxy-users";

#[derive(Debug, Clone, Serialize, Deserialize, MyHttpInputObjectStructure)]
pub struct SettingsModel {
    pub environments: BTreeMap<String, BTreeMap<String, Vec<String>>>,
}

impl SettingsModel {
    pub async fn new() -> Self {
        let file = rust_extensions::file_utils::format_path(FILE_NAME);

        let data = tokio::fs::read(file.as_str()).await;

        if let Err(err) = &data {
            panic!("Can not open file: {}. Err: {}", file.as_str(), err);
        }

        let data = data.unwrap();

        if data.is_empty() {
            return Self {
                environments: BTreeMap::new(),
            };
        }

        let result: Result<Self, _> = serde_yaml::from_slice(data.as_slice());

        if let Err(err) = &result {
            panic!("Can not parse file: {}. Err: {}", file.as_str(), err);
        }

        result.unwrap()
    }

    pub async fn save(&self) {
        let file = rust_extensions::file_utils::format_path(FILE_NAME);

        let result = serde_yaml::to_string(self).unwrap();

        tokio::fs::write(file.as_str(), result.as_bytes())
            .await
            .unwrap();
    }
}
