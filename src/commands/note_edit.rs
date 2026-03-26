use crate::app::KnodiqApp;
use knodiq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{NoteID, NoteTrack},
    },
};

impl KnodiqApp {
    pub(crate) fn set_note_start(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_start: Beats,
    ) {
        // Set the note's start time
        if let Some(region_id) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region_id.set_start(note_id, new_start);
        }

        self.modified_project();
    }
}
