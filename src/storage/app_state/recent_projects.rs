use crate::{consts::RECENT_PROJCETS_PATH, ui::workspaces::splash::state::RecentProjData};
use std::{
    fs::{self, File},
    io::Read,
};

pub(crate) fn load_recent_projects() -> std::io::Result<Vec<RecentProjData>> {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(RECENT_PROJCETS_PATH);
    let mut file = File::open(&full_path)?;

    // Load the JSON string and parse it
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let Ok(recent_projects) = serde_json::from_str(&json_string) else {
        return Ok(Vec::new());
    };
    Ok(recent_projects)
}

pub(crate) fn save_recent_projects(recent_projects: &[RecentProjData]) -> std::io::Result<()> {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(RECENT_PROJCETS_PATH);
    std::fs::create_dir_all(&app_data).unwrap();

    // Write the JSON string to the path
    let Ok(json_string) = serde_json::to_string(recent_projects) else {
        return Ok(());
    };
    std::fs::write(&full_path, json_string)
}
