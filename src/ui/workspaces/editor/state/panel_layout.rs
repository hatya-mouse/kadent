use std::fmt::Display;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PanelView {
    #[default]
    Timeline,
    PianoRoll,
    NodeGraph,
    Inspector,
    ErrorList,
    CodeEditor,
}

impl Display for PanelView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PanelView::Timeline => write!(f, "Timeline"),
            PanelView::PianoRoll => write!(f, "Piano Roll"),
            PanelView::NodeGraph => write!(f, "Node Graph"),
            PanelView::Inspector => write!(f, "Inspector"),
            PanelView::ErrorList => write!(f, "Error List"),
            PanelView::CodeEditor => write!(f, "Code Editor"),
        }
    }
}

impl PanelView {
    pub fn all() -> &'static [Self] {
        &[
            PanelView::Timeline,
            PanelView::PianoRoll,
            PanelView::NodeGraph,
            PanelView::Inspector,
            PanelView::ErrorList,
            PanelView::CodeEditor,
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
    Leaf(PanelView),
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
        PanelNode::Leaf(PanelView::Timeline)
    }
}
