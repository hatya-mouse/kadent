#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct NodeID(pub(crate) u64);

impl Into<u64> for NodeID {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for NodeID {
    fn from(value: u64) -> Self {
        NodeID(value)
    }
}
