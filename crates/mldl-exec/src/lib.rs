use chrono::Utc;
use mldl_types::{BenchmarkDescriptor, RunRequest, RunResult, SignedResult};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("prepare failed: {0}")]
    Prepare(String),
    #[error("run failed: {0}")]
    Run(String),
    #[error("collect failed: {0}")]
    Collect(String),
    #[error("sign failed: {0}")]
    Sign(String),
    #[error("submit failed: {0}")]
    Submit(String),
}

pub trait BenchmarkRunner {
    fn prepare(&mut self, request: &RunRequest) -> Result<(), ExecError>;
    fn run(&mut self) -> Result<(), ExecError>;
    fn collect(&mut self) -> Result<RunResult, ExecError>;
}

pub trait ResultSigner {
    fn sign(&self, result: &RunResult) -> Result<SignedResult, ExecError>;
}

pub struct NopSigner;
impl ResultSigner for NopSigner {
    fn sign(&self, result: &RunResult) -> Result<SignedResult, ExecError> {
        Ok(SignedResult {
            result: result.clone(),
            signature: "insecure-dev-signature".to_string(),
        })
    }
}

/// Interface for submitting a signed benchmark result to a backend.
pub trait ResultSubmitter {
    fn submit(&self, signed: &SignedResult) -> Result<(), ExecError>;
}

/// No-op submitter for development and tests.
pub struct NopSubmitter;

impl ResultSubmitter for NopSubmitter {
    fn submit(&self, _signed: &SignedResult) -> Result<(), ExecError> { Ok(()) }
}

pub fn run_benchmark<R: BenchmarkRunner, S: ResultSigner>(
    runner: &mut R,
    signer: &S,
    request: &RunRequest,
) -> Result<SignedResult, ExecError> {
    runner.prepare(request)?;
    runner.run()?;
    let mut result = runner.collect()?;
    // Ensure IDs and times are populated even for dummy runners
    if result.run_id.is_nil() {
        result.run_id = Uuid::new_v4();
    }
    if result.started_at.timestamp() == 0 {
        result.started_at = Utc::now();
    }
    if result.completed_at.timestamp() == 0 {
        result.completed_at = Utc::now();
    }
    signer.sign(&result)
}

pub struct DummyRunner {
    request: Option<RunRequest>,
}

impl DummyRunner {
    pub fn new() -> Self { Self { request: None } }
}

impl BenchmarkRunner for DummyRunner {
    fn prepare(&mut self, request: &RunRequest) -> Result<(), ExecError> {
        self.request = Some(request.clone());
        Ok(())
    }
    fn run(&mut self) -> Result<(), ExecError> { Ok(()) }
    fn collect(&mut self) -> Result<RunResult, ExecError> {
        let now = Utc::now();
        Ok(RunResult {
            run_id: Uuid::new_v4(),
            started_at: now,
            completed_at: now,
            metrics: serde_json::json!({ "ok": true }),
            artifacts: vec![],
        })
    }
}

pub fn make_dummy_request(benchmark: BenchmarkDescriptor) -> RunRequest {
    let device = mldl_types::DeviceSpec {
        id: None,
        hostname: "unknown".into(),
        os: "unknown".into(),
        cpu_model: None,
        cpu_cores: None,
        memory_gb: None,
        gpu_model: None,
        gpu_driver: None,
    };
    RunRequest { device, benchmark, parameters: serde_json::json!({}) }
}


