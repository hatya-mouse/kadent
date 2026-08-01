use crate::storage::project::{
    FromBytes,
    error::{Contextualize, LoadError, ParseContext},
    trait_impl::project_meta::StoredProjMeta,
    traits::safe_read,
};
use kadent_engine::mixer::Project;
use std::io::{Cursor, Read};

pub(crate) struct LoadProjResult {
    pub(crate) project: Project,
    pub(crate) project_meta: StoredProjMeta,
}

impl FromBytes for LoadProjResult {
    fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let mut cursor = Cursor::new(bytes);

        // Read the project metadata
        let mut project_meta_len_bytes = [0u8; 8];
        cursor
            .read_exact(&mut project_meta_len_bytes)
            .with_ctx(ParseContext::LoadProjResult)?;
        let project_meta_len = u64::from_le_bytes(project_meta_len_bytes) as usize;

        // Read the project metadata bytes and prase it
        let project_meta_bytes =
            safe_read(&mut cursor, project_meta_len).with_ctx(ParseContext::LoadProjResult)?;
        let project_meta = StoredProjMeta::from_bytes(&project_meta_bytes)?;

        // Read the rest of the file and parse the project
        let mut project_bytes = Vec::new();
        cursor
            .read_to_end(&mut project_bytes)
            .with_ctx(ParseContext::LoadProjResult)?;
        let project = Project::from_bytes(&project_bytes)?;

        // Construct the new LoadProjResult
        let result = LoadProjResult {
            project,
            project_meta,
        };

        Ok(result)
    }
}
