use crate::utils::get_resources_path;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub(super) struct LicenseDocument {
    pub(super) licenses: Vec<LicenseItem>,
    pub(super) crates: Vec<DependencyItem>,
    pub(super) fonts: Vec<DependencyItem>,
}

#[derive(Debug, Deserialize)]
pub(super) struct LicenseItem {
    pub(super) name: String,
    pub(super) id: String,
    pub(super) text: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct DependencyItem {
    pub(super) name: String,
    pub(super) version: Option<String>,
    pub(super) authors: Option<Vec<String>>,
    pub(super) description: Option<String>,
    pub(super) license_index: usize,
    pub(super) notice: Option<String>,
}

pub(super) fn load_crate_licenses() -> std::io::Result<LicenseDocument> {
    let licenses_path = get_resources_path().map(|path| path.join("licenses.json"))?;
    let mut file = File::open(licenses_path)?;

    // Load the JSON string and parse it
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    serde_json::from_str(&json_string)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
