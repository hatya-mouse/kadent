use crate::core::kasl_node::KaslNode;
use kadent_engine::node::{
    Node,
    builtin::{AudioInputNode, AudioOutputNode, NoteInputNode},
};
use serde::{Deserialize, Serialize};

/// Mirror of the `Box<dyn Node>` for persistence.
#[derive(Serialize, Deserialize)]
pub(crate) enum StoredNode {
    AudioInput,
    AudioOutput,
    NoteInput,
    Kasl { file_path: Option<String> },
}

impl StoredNode {
    /// Returns `None` if the node is of an unrecognized type.
    pub fn from_node(node: &dyn Node) -> Option<Self> {
        let any_node = node.as_any();
        if any_node.is::<AudioInputNode>() {
            Some(Self::AudioInput)
        } else if any_node.is::<AudioOutputNode>() {
            Some(Self::AudioOutput)
        } else if any_node.is::<NoteInputNode>() {
            Some(Self::NoteInput)
        } else {
            any_node
                .downcast_ref::<KaslNode>()
                .map(|kasl_node| Self::Kasl {
                    file_path: kasl_node.get_file_path().cloned(),
                })
        }
    }

    pub fn to_node(&self) -> Box<dyn Node> {
        match self {
            Self::AudioInput => Box::new(AudioInputNode::default()),
            Self::AudioOutput => Box::new(AudioOutputNode::default()),
            Self::NoteInput => Box::new(NoteInputNode::default()),
            Self::Kasl { file_path } => {
                let mut node = KaslNode::new();
                if let Some(path) = file_path {
                    node.set_file_path(path.clone());
                }
                Box::new(node)
            }
        }
    }
}
