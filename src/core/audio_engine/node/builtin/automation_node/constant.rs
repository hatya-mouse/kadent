use crate::core::audio_engine::node::builtin::Keyframe;

#[derive(Debug, Clone, Default)]
pub(crate) struct ConstantAutomationCursor {
    pub(crate) current_index: Option<usize>,
}

impl ConstantAutomationCursor {
    pub(super) fn clear_cache(&mut self) {
        self.current_index = None;
    }

    pub(super) fn get_constant_keyframe_value<T>(
        &mut self,
        keyframes: &[Keyframe<T>],
        keyframe_samples: &[usize],
        target_sample: usize,
    ) -> T
    where
        T: Default + Copy,
    {
        if keyframes.is_empty() {
            return T::default();
        }

        if keyframes.len() == 1 {
            return keyframes[0].value;
        }

        // Get the sample of the current keyframe
        if let Some(current_index) = self.current_index {
            let keyframe_sample = keyframe_samples[current_index];

            // If the target sample is before the first keyframe, return the value of the first keyframe
            // |    |                *
            // 0    ^ target_sample  ^ first keyframe
            if current_index == 0 && target_sample < keyframe_sample {
                return keyframes[0].value;
            }

            if let Some(next_keyframe_sample) = keyframe_samples.get(current_index + 1) {
                if target_sample >= keyframe_sample && target_sample <= *next_keyframe_sample {
                    return keyframes[current_index].value;
                }
            } else {
                // If there is no next keyframe (which means that the current keyframe is the last one),
                // return the value of the current keyframe
                if target_sample >= keyframe_sample {
                    return keyframes[current_index].value;
                }
            }
        }

        self.calculate_keyframe_index_and_get_value(keyframes, keyframe_samples, target_sample)
    }

    /// Calculates the keyframe index for the given target sample and returns the corresponding keyframe value.
    fn calculate_keyframe_index_and_get_value<T>(
        &mut self,
        keyframes: &[Keyframe<T>],
        keyframe_samples: &[usize],
        target_sample: usize,
    ) -> T
    where
        T: Default + Copy,
    {
        // Binary search returns the index of the keyframe right AFTER the target sample,
        // so we need to subtract 1 to get the index of the keyframe right BEFORE the target sample
        // If the target sample exists in the keyframe_samples, use the index directly
        self.current_index = Some(
            keyframe_samples
                .binary_search(&target_sample)
                .unwrap_or_else(|index| index.saturating_sub(1)),
        );

        if let Some(new_index) = self.current_index {
            keyframes[new_index].value
        } else {
            T::default()
        }
    }
}
