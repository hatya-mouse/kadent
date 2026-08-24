use crate::{
    core::audio_engine::{
        data_types::Ticks,
        mixer::TrackID,
        track::{
            RegionID,
            note_track::{NoteID, NoteTrack},
        },
    },
    ui::editor::EditorUi,
};

impl EditorUi {
    pub(crate) fn move_note(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_start: Ticks,
    ) {
        // Set the note's start time
        if let Some(region) = self
            .state
            .project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_start(note_id, new_start);
        }

        self.state.actions.modified_project();
    }

    pub(crate) fn set_note_pitch(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_pitch: f32,
    ) {
        if let Some(region) = self
            .state
            .project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_pitch(note_id, new_pitch);
        }
        self.state.actions.modified_project();
    }

    pub(crate) fn set_note_duration(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_duration: Ticks,
    ) {
        // Set the note's duration
        if let Some(region) = self
            .state
            .project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_duration(note_id, new_duration);
        }

        self.state.actions.modified_project();
    }

    pub(crate) fn set_note_velocity(
        &mut self,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        new_velocity: f32,
    ) {
        if let Some(region) = self
            .state
            .project
            .data
            .get_track_mut(track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(region_id))
        {
            region.set_velocity(note_id, new_velocity);
        }

        self.state.actions.modified_project();
    }
}
