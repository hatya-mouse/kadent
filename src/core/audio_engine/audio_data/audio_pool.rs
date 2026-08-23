use crate::core::audio_engine::audio_data::{AudioData, AudioSource};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Weak},
};

#[derive(Default)]
pub(crate) struct AudioFilePool {
    /// A cache of loaded audio data, keyed by the file path.
    /// The audio data will automatically be dropped when there are no references to it.
    cache: HashMap<PathBuf, Weak<AudioData>>,
}

impl AudioFilePool {
    /// Loads audio data from the given source, or retrieves it from the cache if it has already been loaded.
    pub(crate) fn get_or_load(&mut self, source: &AudioSource) -> Option<Arc<AudioData>> {
        let path = match source {
            AudioSource::Original(path) | AudioSource::Modified(path) => path,
            AudioSource::Zero => return None,
        };

        if let Some(data) = self.cache.get(path)
            && let Some(arc) = data.upgrade()
        {
            return Some(arc);
        }

        // If the data is not in the cache or has been dropped, load it from the file
        let loaded_data = Arc::new(source.get_data()?);
        self.cache
            .insert(path.clone(), Arc::downgrade(&loaded_data));
        Some(loaded_data)
    }

    /// Cleans up the cache by removing entries that have been dropped.
    pub(crate) fn cleanup(&mut self) {
        self.cache
            .retain(|_, weak_data| weak_data.strong_count() > 0);
    }
}
