use kasl::core::error::ErrorRecord;
use kreniq_engine::graph::error::NodeError;
use std::fmt::Display;

#[derive(Debug)]
pub enum KaslNodeError {
    Compile(Vec<ErrorRecord>),
    FileRead(std::io::Error),
    Backend(String),
}

impl Display for KaslNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KaslNodeError::Compile(records) => {
                for record in records {
                    write!(f, "{}", record)?;
                }
                Ok(())
            }
            KaslNodeError::FileRead(io_error) => {
                write!(f, "{}", io_error)
            }
            KaslNodeError::Backend(error_str) => {
                write!(f, "{}", error_str)
            }
        }
    }
}

unsafe impl Send for KaslNodeError {}

impl NodeError for KaslNodeError {}
