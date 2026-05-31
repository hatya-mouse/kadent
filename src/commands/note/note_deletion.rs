use crate::ui::EditorUi;
use krenic_engine::{
    mixer::TrackID,
    track::{
        note_track::{NoteID, NoteTrack},
        RegionID,
    },
};

impl EditorUi {
    pub(crate) fn remove_note(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
    ) {
        // Set the note's start time
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.remove_note(note_id);
        }

        self.ui_state.deselect_note();

        self.modified_project();
    }
}
