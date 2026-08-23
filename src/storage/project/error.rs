#[derive(Debug)]
pub(crate) enum LoadError {
    /// The file is not a Kadent ProjectData file.
    NotAProjectFile,
    /// An error occured while decoding the binary.
    DecodeError(sode::DecodeError),
    /// An error occurred while reading the file.
    IoError(std::io::Error),
}

#[derive(Debug)]
pub(crate) enum SaveError {
    /// An error occurred while encoding the project data.
    EncodeError(sode::EncodeError),
    /// An error occurred while writing the file.
    IoError(std::io::Error),
}
