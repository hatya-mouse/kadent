use crate::ui::editor::{
    PanelVariant,
    views::{AutomationPanelState, CodeEditorPanelState, PianoRollPanelState, TimelinePanelState},
};

#[derive(Clone, Debug)]
pub(crate) enum PanelViewState {
    Timeline(TimelinePanelState),
    PianoRoll(PianoRollPanelState),
    NodeGraph,
    Inspector,
    ErrorList,
    Automation(AutomationPanelState),
    CodeEditor(CodeEditorPanelState),
}

impl PanelViewState {
    pub(crate) fn variant(&self) -> PanelVariant {
        match self {
            PanelViewState::Timeline { .. } => PanelVariant::Timeline,
            PanelViewState::PianoRoll(_) => PanelVariant::PianoRoll,
            PanelViewState::NodeGraph => PanelVariant::NodeGraph,
            PanelViewState::Inspector => PanelVariant::Inspector,
            PanelViewState::ErrorList => PanelVariant::ErrorList,
            PanelViewState::Automation(_) => PanelVariant::Automation,
            PanelViewState::CodeEditor { .. } => PanelVariant::CodeEditor,
        }
    }

    pub(crate) fn from_variant(variant: &PanelVariant) -> Self {
        match variant {
            PanelVariant::Timeline => PanelViewState::Timeline(TimelinePanelState::default()),
            PanelVariant::PianoRoll => PanelViewState::PianoRoll(PianoRollPanelState::default()),
            PanelVariant::NodeGraph => PanelViewState::NodeGraph,
            PanelVariant::Inspector => PanelViewState::Inspector,
            PanelVariant::ErrorList => PanelViewState::ErrorList,
            PanelVariant::Automation => PanelViewState::Automation(AutomationPanelState::default()),
            PanelVariant::CodeEditor => PanelViewState::CodeEditor(CodeEditorPanelState::default()),
        }
    }
}
