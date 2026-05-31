use crate::ui::EditorUi;
use kreniq_engine::{
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteTrack},
    },
};

impl EditorUi {
    pub(crate) fn add_note(&mut self, track_id: &TrackID, region_id: &RegionID, note: Note) {
        // Set the note's start time
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.add_note(note);
        }

        self.modified_project();
    }
}
