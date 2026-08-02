#[derive(Debug)]
pub(crate) enum LoadError {
    /// The file is not a Kadent Project file
    NotAProjectFile,
    /// The file is possibly corrupted or incomplete
    Postcard(postcard::Error),
    /// An error occurred while reading the file
    IoError(std::io::Error),
}
