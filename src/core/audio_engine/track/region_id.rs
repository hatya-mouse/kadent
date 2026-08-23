#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct RegionID(pub(crate) u64);

impl From<RegionID> for u64 {
    fn from(value: RegionID) -> u64 {
        value.0
    }
}
impl From<u64> for RegionID {
    fn from(value: u64) -> Self {
        RegionID(value)
    }
}
