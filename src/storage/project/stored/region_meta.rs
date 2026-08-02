use crate::core::metadata::RegionMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredRegionMeta {
    pub name: String,
}

impl StoredRegionMeta {
    pub fn from_region_meta(region_meta: &RegionMeta) -> Self {
        Self {
            name: region_meta.name.clone(),
        }
    }
}
