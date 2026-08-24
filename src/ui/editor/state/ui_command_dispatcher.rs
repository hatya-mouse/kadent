use crate::ui::editor::DialogType;
use eframe::egui;
use std::collections::VecDeque;

pub(crate) enum UiCommand {
    /// Show a given dialog.
    ShowDialog(DialogType),
    /// Show a temporary status in the status bar.
    ShowTempStatus(String, egui::Color32),
}

#[derive(Default)]
pub(crate) struct UiCommandDispatcher {
    /// Pending UI commands to be executed at the end of the frame.
    pub(crate) pending: VecDeque<UiCommand>,
}

impl UiCommandDispatcher {
    /// Pushes an UI command to the pending queue.
    pub(crate) fn push_command(&mut self, command: UiCommand) {
        self.pending.push_back(command);
    }
}
