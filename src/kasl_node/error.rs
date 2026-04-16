use kasl::core::error::ErrorRecord;
use knodiq_engine::graph::error::NodeError;
use std::fmt::Display;

#[derive(Debug)]
pub struct KaslNodeError {
    pub records: Vec<ErrorRecord>,
}

impl KaslNodeError {
    pub fn new(records: Vec<ErrorRecord>) -> Self {
        Self { records }
    }
}

impl Display for KaslNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for record in &self.records {
            write!(f, "{}", record)?;
        }
        Ok(())
    }
}

unsafe impl Send for KaslNodeError {}

impl NodeError for KaslNodeError {}
