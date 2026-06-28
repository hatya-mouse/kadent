use crate::storage::project::{
    AsBytes, FromBytes,
    error::{Contextualize, LoadError, ParseContext},
};

pub struct StoredRegionMeta {
    pub name: String,
}

impl StoredRegionMeta {
    pub fn from_region_meta(region_meta: &crate::core::metadata::RegionMeta) -> Self {
        Self {
            name: region_meta.name.clone(),
        }
    }
}

impl AsBytes for StoredRegionMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the name to the bytes vector
        bytes.extend(self.name.as_bytes());
    }
}

impl FromBytes for StoredRegionMeta {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        // Read the name from the bytes vector
        let name = String::from_utf8(bytes.to_vec())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .with_ctx(ParseContext::RegionMeta)?;
        Ok(Self { name })
    }
}
