use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpec {
    pub id: Option<Uuid>,
    pub hostname: String,
    pub os: String,
    pub cpu_model: Option<String>,
    pub cpu_cores: Option<u32>,
    pub memory_gb: Option<f32>,
    pub gpu_model: Option<String>,
    pub gpu_driver: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkDescriptor {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    pub device: DeviceSpec,
    pub benchmark: BenchmarkDescriptor,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    pub run_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub metrics: serde_json::Value,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedResult {
    pub result: RunResult,
    pub signature: String,
}


