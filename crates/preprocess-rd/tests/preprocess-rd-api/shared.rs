#[derive(Debug, thiserror::Error)]
#[error("Test failed")]
pub struct TestError;
