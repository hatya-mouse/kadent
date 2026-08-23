/// Represents the size and alignment of a type in memory.
/// `sample_size` represents the size of the value for a single sample, so you need to multiply it by the buffer size
/// to get the actual size of the buffer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct TypeInfo {
    pub(crate) sample_size: usize,
    pub(crate) align: usize,
}

impl Default for TypeInfo {
    fn default() -> Self {
        Self {
            sample_size: 0,
            align: 1,
        }
    }
}

impl TypeInfo {
    pub(crate) fn new(sample_size: usize, align: usize) -> Self {
        Self { sample_size, align }
    }

    /// Returns the actual size of the buffer, given the buffer size.
    pub(crate) fn actual_size(&self, buffer_size: usize) -> usize {
        self.sample_size * buffer_size
    }
}
