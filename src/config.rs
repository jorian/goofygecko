use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub trace_level: String,
    pub enable_tracing: bool,
    pub discord: String,
    pub assets_directory: String,
    pub output_directory: String,
}
