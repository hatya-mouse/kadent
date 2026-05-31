use crate::load_write::{FromBytes, project_meta_io::StoredProjMeta, traits::safe_read};
use kreniq_engine::mixer::Project;
use std::io::{Cursor, Read};

pub(crate) struct LoadProjResult {
    pub(crate) project: Project,
    pub(crate) proj_meta: StoredProjMeta,
}

impl FromBytes for LoadProjResult {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the project metadata
        let mut proj_meta_len_bytes = [0u8; 8];
        cursor.read_exact(&mut proj_meta_len_bytes)?;
        let proj_meta_len = u64::from_le_bytes(proj_meta_len_bytes) as usize;

        // Read the project metadata bytes and prase it
        let proj_meta_bytes = safe_read(&mut cursor, proj_meta_len)?;
        let proj_meta = StoredProjMeta::from_bytes(&proj_meta_bytes)?;

        // Read the rest of the file and parse the project
        let mut project_bytes = Vec::new();
        cursor.read_to_end(&mut project_bytes)?;
        let project = Project::from_bytes(&project_bytes)?;

        // Construct the new LoadProjResult
        let result = LoadProjResult { project, proj_meta };

        Ok(result)
    }
}
