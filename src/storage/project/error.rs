#[derive(Debug)]
pub(crate) enum ParseContext {
    Node,
    KaslNode,
    ProjectMeta,
    GraphMeta,
    NodeMeta,
    RegionMeta,
    TrackMeta,
    TempoMap,
    TempoEvent,
    Track,
    NoteTrack,
    Note,
    NoteRegion,
    ProjectConfig,
    HardwareConfig,
    Graph,
    LoadProjResult,
    Project,
}

#[derive(Debug)]
pub(crate) enum LoadError {
    /// The file is not a Kadent Project file
    NotAProjectFile,
    /// The file is possibly corrupted or incomplete
    FileCorrupted {
        context: ParseContext,
        source: std::io::Error,
    },
    /// An error occurred while reading the file
    IoError(std::io::Error),
}

pub(crate) trait Contextualize<T> {
    fn with_ctx(self, ctx: ParseContext) -> Result<T, LoadError>;
}

impl<T> Contextualize<T> for Result<T, std::io::Error> {
    fn with_ctx(self, ctx: ParseContext) -> Result<T, LoadError> {
        self.map_err(|e| LoadError::FileCorrupted {
            context: ctx,
            source: e,
        })
    }
}
