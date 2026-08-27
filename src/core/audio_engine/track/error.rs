use crate::core::audio_engine::graph::error::GraphError;

pub(crate) enum TrackError {
    GraphError(GraphError),
    ThreadSpawnFailed(String),
    RenderWorkerPanicked,
}
