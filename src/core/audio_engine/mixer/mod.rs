mod project;
mod track_id;

pub(crate) use project::ProjectData;
pub(crate) use track_id::TrackID;

use crate::core::audio_engine::{MAX_CHANNELS, data_types::PlaybackContext};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub(crate) struct Mixer {
    // --- PROJECT ---
    pub(crate) project: ProjectData,
    // --- PLAYBACK CONTEXT ---
    pub(crate) playback_ctx: PlaybackContext,
}

impl Mixer {
    // --- NEW ---

    /// Creates a new mixer instance with the given project.
    pub(crate) fn new(project: ProjectData, playback_ctx: PlaybackContext) -> Self {
        Self {
            project,
            playback_ctx,
        }
    }

    // --- SEEKING ---

    /// Tells every tracks that the it will seek.
    pub(crate) fn seek(&mut self, playhead: usize) {
        for track in self.project.tracks.values_mut() {
            track.seek(playhead, &self.playback_ctx);
        }
    }

    // --- MIXING PROCESS ---

    /// Processes the tracks in the mixer at the specified playhead.
    pub(crate) fn process(
        &mut self,
        is_exporting: bool,
        is_playing: bool,
        playhead: usize,
        output: &mut [f32],
    ) {
        // Fill the output buffer with zeros before processing
        output.fill(0.0);

        // Process samples and write them to local buffers
        self.project
            .tracks
            .values_mut()
            .par_bridge()
            .for_each(|track| {
                track.process_to_local_buffer(
                    is_exporting,
                    is_playing,
                    playhead,
                    &self.project.tempo_map,
                    &self.playback_ctx,
                );
            });

        let channels = self.playback_ctx.channels;
        let output_samples = output.len() / channels;

        // Add the output of each tracks to the main output buffer
        for track in self.project.tracks.values() {
            // Local buffer is an interleaved buffer with MAX_CHANNELS channels,
            // so we need to get only the required channels for the output buffer
            let local_buf = track.get_local_buffer();
            for sample in 0..output_samples {
                let src_sample_offset = sample * MAX_CHANNELS;
                let dst_sample_offset = sample * channels;

                for channel in 0..channels {
                    output[dst_sample_offset + channel] += local_buf[src_sample_offset + channel];
                }
            }
        }

        // Clamp the output between -1.0 and 1.0 for safety
        output.iter_mut().for_each(|s| *s = s.clamp(-1.0, 1.0))
    }
}
