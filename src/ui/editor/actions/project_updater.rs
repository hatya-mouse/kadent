use crate::ui::EditorState;
use std::time::Instant;

impl EditorState {
    /// Marks the project as modified and updates the last edit time. Should be called whenever the project is modified.
    pub(crate) fn modified_project(&mut self) {
        self.actions.last_edit_time = Some(Instant::now());
    }
}
