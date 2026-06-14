//! Utilities not strictly tied to Kadent.

/// Spawns a background thread to initialize the value.
/// The value must implement `Default`. The macro returns an `Arc<Mutex<T>>` where `T` is the type of the value being initialized.
#[macro_export]
macro_rules! spawn_background_init {
    ($init_expr:expr) => {{
        let data = Arc::new(Mutex::new(Default::default()));
        let data_clone = Arc::clone(&data);

        // Spawn a background thread to initialize the value
        thread::spawn(move || {
            let loaded = $init_expr;
            if let Ok(mut guard) = data_clone.lock() {
                *guard = loaded;
            }
        });

        data
    }};
}

/// Generates a version number string.
pub(crate) fn version_string() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Version: {version}")
}
