use std::fmt::Display;

use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PanelView {
    #[default]
    Timeline,
    Inspector,
    Automation,
    NodeGraph,
    PianoRoll,
    CodeEditor,
    ErrorList,
}

impl Display for PanelView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PanelView::Timeline => write!(f, "Timeline"),
            PanelView::Inspector => write!(f, "Inspector"),
            PanelView::Automation => write!(f, "Automation"),
            PanelView::NodeGraph => write!(f, "Node Graph"),
            PanelView::PianoRoll => write!(f, "Piano Roll"),
            PanelView::CodeEditor => write!(f, "Code Editor"),
            PanelView::ErrorList => write!(f, "Error List"),
        }
    }
}

impl PanelView {
    pub fn all() -> &'static [Self] {
        &[
            PanelView::Timeline,
            PanelView::Inspector,
            PanelView::Automation,
            PanelView::NodeGraph,
            PanelView::PianoRoll,
            PanelView::CodeEditor,
            PanelView::ErrorList,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SplitDir {
    /// First child on top, second on bottom.
    Horizontal,
    /// First child on left, second on right.
    Vertical,
}

#[derive(Clone, Debug)]
pub enum PanelNode {
    Leaf(PanelView, Uuid),
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
        PanelNode::Leaf(PanelView::Timeline, Uuid::new_v4())
    }
}
