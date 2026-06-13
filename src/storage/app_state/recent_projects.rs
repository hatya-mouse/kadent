use crate::{
    consts::{KADENT_DATA_DIR_NAME, RECENT_PROJCETS_PATH},
    ui::workspaces::splash::state::RecentProjData,
};
use eframe::egui::TextBuffer;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub(crate) fn load_recent_projects() -> Vec<RecentProjData> {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(KADENT_DATA_DIR_NAME)
        .join(RECENT_PROJCETS_PATH);
    let Ok(mut file) = File::open(&full_path) else {
        return Vec::new();
    };

    // Load the JSON string and parse it
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).ok();

    let paths: Vec<PathBuf> = match serde_json::from_str(&json_string) {
        Ok(paths) => paths,
        Err(_) => {
            return Vec::new();
        }
    };

    // Create RecentProjData from paths
    paths
        .iter()
        .map(|path| RecentProjData {
            name: path
                .file_name()
                .map(|os_str| os_str.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Untitled".to_string()),
            path_str: path.to_string_lossy().to_string(),
            path: path.clone(),
        })
        .collect()
}

pub(crate) fn save_recent_projects(paths: &[PathBuf]) {
    let app_data_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(KADENT_DATA_DIR_NAME);
    std::fs::create_dir_all(&app_data_path).ok();

    let full_path = app_data_path.join(RECENT_PROJCETS_PATH);

    // Write the JSON string to the path
    let Ok(json_string) = serde_json::to_string(&paths) else {
        eprintln!("JSON ERROR");
        return;
    };

    let Ok(mut file) = File::create(&full_path) else {
        eprintln!("FILE CREATION ERROR");
        return;
    };
    file.write_all(json_string.as_bytes()).ok();
}
