use crate::utils::get_resources_path;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize)]
struct LicenseDocument {
    licenses: Vec<CrateLicenseItem>,
}

#[derive(Deserialize)]
pub(super) struct CrateLicenseItem {
    name: String,
    id: String,
    text: String,
    source_path: String,
    used_by: Vec<UsedBy>,
}

#[derive(Deserialize)]
pub(super) struct UsedBy {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

#[derive(Deserialize)]
pub(super) struct CrateInfo {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
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
