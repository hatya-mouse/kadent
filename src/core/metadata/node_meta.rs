use crate::core::audio_engine::node::{
    Node,
    builtin::{AudioInputNode, AudioOutputNode, NoteInputNode},
};
use crate::core::kasl_node::KaslNode;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum NodeType {
    NoteInput = 0,
    AudioInput = 1,
    AudioOutput = 2,
    Kasl = 3,
    Automation = 4,
}

impl From<u8> for NodeType {
    fn from(value: u8) -> Self {
        match value {
            0 => NodeType::NoteInput,
            1 => NodeType::AudioInput,
            2 => NodeType::AudioOutput,
            3 => NodeType::Kasl,
            4 => NodeType::Automation,
            _ => panic!("Unknown node type"),
        }
    }
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::NoteInput => write!(f, "Note Input"),
            NodeType::AudioInput => write!(f, "Audio Input"),
            NodeType::AudioOutput => write!(f, "Audio Output"),
            NodeType::Kasl => write!(f, "KASL"),
            NodeType::Automation => write!(f, "Automation"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeMeta {
    /// The type of the node.
    pub(crate) node_type: NodeType,
    /// The name of the node, used for display purposes.
    pub(crate) display_name: String,
    /// The position of the node in canvas space.
    pub(crate) pos: egui::Pos2,
}

impl NodeMeta {
    pub(crate) fn new(node_type: NodeType, display_name: String, pos: egui::Pos2) -> Self {
        Self {
            node_type,
            display_name,
            pos,
        }
    }

    pub(crate) fn from_node(node: &dyn Node) -> Self {
        let any_node = node.as_any();
        let node_type = if any_node.is::<NoteInputNode>() {
            NodeType::NoteInput
        } else if any_node.is::<AudioInputNode>() {
            NodeType::AudioInput
        } else if any_node.is::<AudioOutputNode>() {
            NodeType::AudioOutput
        } else if any_node.is::<KaslNode>() {
            NodeType::Kasl
        } else {
            panic!("Unknown node type");
        };

        let display_name = node_type.to_string();
        Self {
            node_type,
            display_name,
            pos: egui::Pos2::new(0.0, 0.0),
        }
    }
}
