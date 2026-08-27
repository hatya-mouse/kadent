use crate::core::{metadata::ProjectMeta, project_ctx::ProjectContext};
use crate::storage::app_state::AppPreferences;
use crate::ui::editor::CodeBuffer;
use crate::{
    core::audio_engine::{
        audio_data::{AudioData, AudioSource},
        data_types::{PlaybackContext, Ticks},
        mixer::{ProjectData, TrackID},
        track::RegionID,
    },
    storage::project::SaveError,
};
use kasl::core::error::ErrorRecord;
use std::path::PathBuf;
use uuid::Uuid;

pub(crate) struct DecodedAudio {
    pub(crate) path: PathBuf,
    pub(crate) frames: usize,
    pub(crate) sample_rate: u64,
}

/// Min/max peaks for a single waveform LOD tier. `peaks[i] = (min, max)` sample value within
/// the i-th block of the region's audio data, downsampled to a fixed block size for this tier.
pub(crate) struct WaveformPeaks {
    pub(crate) peaks: Vec<(f32, f32)>,
}

/// Precomputed waveform peaks at three fixed resolutions.
pub(crate) struct WaveformLod {
    pub(crate) data: AudioData,
    pub(crate) small: WaveformPeaks,
    pub(crate) medium: WaveformPeaks,
    pub(crate) large: WaveformPeaks,
}

pub(crate) enum BackgroundThreadCommand {
    SaveProject {
        path: PathBuf,
        project: Box<ProjectData>,
        project_meta: Box<ProjectMeta>,
        code_buffers: Vec<CodeBuffer>,
    },
    OpenProject {
        path: PathBuf,
        preferences: AppPreferences,
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
    },
    LintKasl {
        buffer_id: Uuid,
        code: String,
    },
}

pub(crate) enum BackgroundThreadResult {
    SavedProject(Result<(), SaveError>),
    OpenedProject(Option<Box<ProjectContext>>),
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
    LintKasl {
        buffer_id: Uuid,
        errors: Vec<ErrorRecord>,
    },
}

pub(crate) enum BackgroundTaskStatus {
    Save,
    Open,
    Export,
    Import,
    GenerateWaveform,
}
