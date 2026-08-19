use crate::ui::editor::{PanelView, TimelineCoord};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum PanelViewState {
    Timeline {
        follow_playhead: bool,
        track_list_width: f32,
        timeline_coord: TimelineCoord,
    },
    PianoRoll(TimelineCoord),
    CodeEditor(Option<(PathBuf, String)>),
}

impl PanelViewState {
    pub(crate) fn view(&self) -> PanelView {
        match self {
            PanelViewState::Timeline { .. } => PanelView::Timeline,
            PanelViewState::PianoRoll(_) => PanelView::PianoRoll,
            PanelViewState::CodeEditor(_) => PanelView::CodeEditor,
        }
    }
}
