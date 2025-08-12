use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSummary {
    pub cpu_model: String,
    pub gpu_model: Option<String>,
    pub ram_gb: u16,
    pub os: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    pub suite_version: String,
    pub tasks: Vec<String>,
}
