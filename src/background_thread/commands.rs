use crate::core::{metadata::ProjectMeta, project_ctx::EditorContext};
use kadent_engine::{
    data_types::{PlaybackContext, Ticks},
    mixer::{Project, TrackID},
    track::{RegionID, audio_track::AudioSource},
};
use std::path::PathBuf;

pub(crate) struct DecodedAudio {
    pub path: PathBuf,
    pub frames: usize,
    pub sample_rate: u64,
    pub channels: usize,
}

/// Min/max peaks for a single waveform LOD tier. `peaks[i] = (min, max)` sample value within
/// the i-th block of the region's audio data, downsampled to a fixed block size for this tier.
pub(crate) struct WaveformPeaks {
    pub peaks: Vec<(f32, f32)>,
}

/// Precomputed waveform peaks at three fixed resolutions.
pub(crate) struct WaveformLod {
    pub small: WaveformPeaks,
    pub medium: WaveformPeaks,
    pub large: WaveformPeaks,
}

pub(crate) enum BackgroundThreadCommand {
    SaveProject {
        path: PathBuf,
        project: Box<Project>,
        project_meta: Box<ProjectMeta>,
        code_buffers: Vec<(PathBuf, String)>,
    },
    OpenProject {
        path: PathBuf,
    },
    WriteWav {
        path: PathBuf,
        samples: Vec<f32>,
        export_ctx: PlaybackContext,
    },
    ImportAudio {
        file_name: Option<String>,
        start: Ticks,
        path: PathBuf,
    },
    GenerateWaveform {
        track_id: TrackID,
        region_id: RegionID,
        source: AudioSource,
        channels: usize,
    },
}

pub(crate) enum BackgroundThreadResult {
    SavedProject(std::io::Result<()>),
    OpenedProject(Option<Box<EditorContext>>),
    WroteWav(hound::Result<()>),
    ImportedAudio {
        file_name: Option<String>,
        start: Ticks,
        result: hound::Result<DecodedAudio>,
    },
    GeneratedWaveform {
        track_id: TrackID,
        region_id: RegionID,
        waveform: WaveformLod,
    },
}

pub(crate) enum BackgroundTaskStatus {
    Save,
    Open,
    Export,
    Import,
    GenerateWaveform,
}
