use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct TrackID(pub(crate) u64);
