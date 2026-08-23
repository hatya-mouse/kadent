pub(crate) mod audio_track;
pub(crate) mod error;
pub(crate) mod note_track;
mod region_id;

pub(crate) use region_id::RegionID;

use crate::core::audio_engine::{
    audio_data::AudioFilePool,
    data_types::{AudioContext, PlaybackContext},
    graph::Graph,
    timing::{TempoMap, TimeBounds},
    track::error::TrackError,
};
use std::any::Any;

pub(crate) trait Track: Send + Any {
    /// Clones the track.
    fn clone_box(&self) -> Box<dyn Track>;

    /// Returns a reference to the Graph.
    fn get_graph(&self) -> &Graph;

    /// Returns a mutable reference to the Graph.
    fn get_graph_mut(&mut self) -> &mut Graph;

    /// Gets the region bounds for the given region ID.
    fn get_region_bounds(&self, region_id: &RegionID) -> Option<&TimeBounds>;

    /// Sets the region bounds to the given one.
    fn set_region_bounds(&mut self, region_id: &RegionID, new_bounds: TimeBounds);

    /// Removes the region from the track.
    fn remove_region(&mut self, region_id: &RegionID);

    /// Sets the playback context to the new one.
    fn update_type_info(&mut self) {
        self.get_graph_mut().update_type_info();
    }

    /// Prepares for the seeking.
    fn seek(&mut self, playhead: usize, playback_ctx: &PlaybackContext);

    /// Prepares the track for processing.
    fn prepare(
        &mut self,
        audio_pool: &mut AudioFilePool,
        tempo_map: &TempoMap,
        audio_ctx: &AudioContext,
        playback_ctx: &PlaybackContext,
    ) -> Result<(), TrackError>;

    /// Processes the track and writes the processed output to the local buffer.
    fn process_to_local_buffer(
        &mut self,
        is_playing: bool,
        playhead: usize,
        tempo_map: &TempoMap,
        playback_ctx: &PlaybackContext,
    );

    /// Returns the processed audio data in the local buffer.
    fn get_local_buffer(&self) -> &[f32];

    /// Converts a reference to the track to any.
    fn as_any(&self) -> &dyn Any;

    /// Converts a mutable reference to the track to any.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Clone for Box<dyn Track> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
