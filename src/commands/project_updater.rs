use crate::ui::workspaces::EditorUi;
use std::time::Instant;

impl EditorUi {
    /// Marks the project as modified and updates the last edit time. Should be called whenever the project is modified.
    pub(super) fn modified_project(&mut self) {
        self.ui_state.last_edit_time = Some(Instant::now());
    }
}
