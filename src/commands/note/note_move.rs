use crate::ui::EditorUi;
use kreniq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{NoteID, NoteTrack},
    },
};

impl EditorUi {
    pub(crate) fn set_note_start(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_start: Beats,
    ) {
        // Set the note's start time
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_start(note_id, new_start);
        }

        self.modified_project();
    }

    pub(crate) fn set_note_pitch(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_pitch: f32,
    ) {
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_pitch(note_id, new_pitch);
        }
        self.modified_project();
    }

    pub(crate) fn set_note_duration(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_duration: Beats,
    ) {
        // Set the note's duration
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_duration(note_id, new_duration);
        }

        self.modified_project();
    }

    pub(crate) fn set_note_velocity(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_velocity: f32,
    ) {
        if let Some(region) = self
            .project
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_velocity(note_id, new_velocity);
        }

        self.modified_project();
    }
}
