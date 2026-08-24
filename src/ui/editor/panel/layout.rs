use crate::ui::editor::views::{PanelViewState, TimelinePanelState};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum PanelVariant {
    #[default]
    Timeline,
    Inspector,
    Automation,
    NodeGraph,
    PianoRoll,
    CodeEditor,
    ErrorList,
}

impl Display for PanelVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PanelVariant::Timeline => write!(f, "Timeline"),
            PanelVariant::Inspector => write!(f, "Inspector"),
            PanelVariant::Automation => write!(f, "Automation"),
            PanelVariant::NodeGraph => write!(f, "Node Graph"),
            PanelVariant::PianoRoll => write!(f, "Piano Roll"),
            PanelVariant::CodeEditor => write!(f, "Code Editor"),
            PanelVariant::ErrorList => write!(f, "Error List"),
        }
    }
}

impl PanelVariant {
    pub(crate) fn all() -> &'static [Self] {
        &[
            PanelVariant::Timeline,
            PanelVariant::Inspector,
            PanelVariant::Automation,
            PanelVariant::NodeGraph,
            PanelVariant::PianoRoll,
            PanelVariant::CodeEditor,
            PanelVariant::ErrorList,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum SplitDir {
    /// First child on top, second on bottom.
    Horizontal,
    /// First child on left, second on right.
    Vertical,
}

#[derive(Clone, Debug)]
pub(crate) enum PanelNode {
    Leaf(PanelViewState, Uuid),
    Split {
        dir: SplitDir,
        /// Fraction [0.1, 0.9] of total size allocated to `first`.
        ratio: f32,
        first: Box<PanelNode>,
        second: Box<PanelNode>,
    },
}

impl Default for PanelNode {
    fn default() -> Self {
        PanelNode::Leaf(
            PanelViewState::Timeline(TimelinePanelState::default()),
            Uuid::new_v4(),
        )
    }
}
