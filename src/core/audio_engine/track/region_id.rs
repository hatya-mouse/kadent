#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct RegionID(pub(crate) u64);

impl Into<u64> for RegionID {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for RegionID {
    fn from(value: u64) -> Self {
        RegionID(value)
    }
}
