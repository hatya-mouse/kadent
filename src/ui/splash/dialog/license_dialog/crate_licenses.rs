use serde::Deserialize;

const LICENSE_STRING: &str = include_str!("../../../../../licenses.json");

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
    serde_json::from_str(LICENSE_STRING)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
