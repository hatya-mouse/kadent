#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct TrackID(pub(crate) u64);

impl From<TrackID> for u64 {
    fn from(value: TrackID) -> u64 {
        value.0
    }
}

impl From<u64> for TrackID {
    fn from(value: u64) -> Self {
        TrackID(value)
    }
}
