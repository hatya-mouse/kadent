use crate::core::audio_engine::graph::node_id::NodeID;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(crate) enum GraphError {
    NodeNotFound(NodeID),
    NodeError(Box<dyn NodeError>),
    NodeCycle(NodeID),
    OutputTypeUnavailable(NodeID, usize),
    InputTypeUnavailable(NodeID, usize),
    NodeTypeMismatch((NodeID, usize, NodeID, usize)),
}

pub(crate) trait NodeError: Send + Debug + Display {}
