use crate::ui::editor::{PanelView, TimelineCoord};

#[derive(Clone)]
pub(crate) enum PanelViewState {
    CodeEditor {
        file_list_width: f32,
    },
    Automation(TimelineCoord),
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
            PanelViewState::CodeEditor { .. } => PanelView::CodeEditor,
            PanelViewState::Automation(_) => PanelView::Automation,
            PanelViewState::PianoRoll(_) => PanelView::PianoRoll,
            PanelViewState::Timeline { .. } => PanelView::Timeline,
        }
    }
}
