use crate::core::audio_engine::graph::error::GraphError;

#[derive(Debug)]
pub(crate) enum TrackError {
    GraphError(GraphError),
    ThreadSpawnFailed(String),
    RenderWorkerPanicked,
}
