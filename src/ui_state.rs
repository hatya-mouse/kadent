pub struct KnodiqUIState {
    /// Whether to show the add track dialog.
    pub show_add_track_dialog: bool,
}

impl Default for KnodiqUIState {
    fn default() -> Self {
        Self {
            show_add_track_dialog: false,
        }
    }
}
