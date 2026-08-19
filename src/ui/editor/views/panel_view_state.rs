use crate::ui::editor::{PanelView, TimelineCoord};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum PanelViewState {
    Automation(TimelineCoord),
    CodeEditor(Option<(PathBuf, String)>),
    PianoRoll(TimelineCoord),
    Timeline {
        follow_playhead: bool,
        track_list_width: f32,
        timeline_coord: TimelineCoord,
    },
}

impl PanelViewState {
    pub(crate) fn view(&self) -> PanelView {
        match self {
            PanelViewState::Automation(_) => PanelView::Automation,
            PanelViewState::CodeEditor(_) => PanelView::CodeEditor,
            PanelViewState::PianoRoll(_) => PanelView::PianoRoll,
            PanelViewState::Timeline { .. } => PanelView::Timeline,
        }
    }
}
