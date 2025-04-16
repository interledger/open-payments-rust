use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub key_id: String,
    pub private_key_path: PathBuf,
    pub jwks_path: Option<PathBuf>,
    pub auth_server: Option<String>,
    pub resource_server: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            key_id: "".into(),
            private_key_path: PathBuf::from("private.key"),
            jwks_path: Some(PathBuf::from("jwks.json")),
            auth_server: None,
            resource_server: None,
        }
    }
}
