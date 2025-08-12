use mldl_types::RunRequest;

#[derive(thiserror::Error, Debug)]
pub enum RunnerError {
    #[error("not implemented")]
    NotImplemented,
}

pub fn run_benchmarks(_req: RunRequest) -> Result<(), RunnerError> {
    // Next steps: spawn Python, manage venv .mldl-venv, stream logs.
    Err(RunnerError::NotImplemented)
}
