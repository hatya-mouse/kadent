use crate::consts::{KADENT_DATA_DIR_NAME, PREFERENCES_PATH};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct AppPreferences {
    pub(crate) kasl_std_path: Option<PathBuf>,
}

pub(crate) fn load_preferences() -> AppPreferences {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(KADENT_DATA_DIR_NAME)
        .join(PREFERENCES_PATH);
    let Ok(mut file) = File::open(&full_path) else {
        return AppPreferences::default();
    };

    // Load the JSON string and parse it
    let mut json_string = String::new();
    if file.read_to_string(&mut json_string).is_err() {
        return AppPreferences::default();
    }

    serde_json::from_str(&json_string).unwrap_or_default()
}

pub(crate) fn save_preferences(preferences: &AppPreferences) -> io::Result<()> {
    let app_data_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(KADENT_DATA_DIR_NAME);
    std::fs::create_dir_all(&app_data_path)?;
    let full_path = app_data_path.join(PREFERENCES_PATH);

    // Write the JSON string to the path
    let json_string = serde_json::to_string(&preferences)?;

    let mut file = File::create(&full_path)?;
    file.write_all(json_string.as_bytes())
}
