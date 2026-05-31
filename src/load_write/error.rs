#[derive(Debug)]
pub(crate) enum LoadError {
    /// The file is not a Kreniq Project file
    NotAProjectFile,
    /// The file is possibly corrupted or incomplete
    FileCorrupted(std::io::Error),
    /// An error occurred while reading the file
    IoError(std::io::Error),
}
