mod preferences;
mod recent_projects;

pub(crate) use preferences::{AppPreferences, load_preferences, save_preferences};
pub(crate) use recent_projects::{add_and_store_recent_projects, load_recent_projects};
