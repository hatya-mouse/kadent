use crate::core::audio_engine::node::builtin::Keyframe;

/// An automation cursor that keeps track of the current index in a list of gradient values.
/// This is solely used to interpolate between **float** keyframes, not integer or boolean keyframes.
#[derive(Debug, Clone, Default)]
pub(crate) struct FloatAutomationCursor {
    /// Currently processing index in the calculated gradient values.
    pub(crate) current_index: Option<usize>,
}

impl FloatAutomationCursor {
    pub(super) fn clear_cache(&mut self) {
        self.current_index = None;
    }

    pub(super) fn get_interpolated_value(
        &mut self,
        keyframes: &[Keyframe<f32>],
        keyframe_samples: &[usize],
        target_sample: usize,
    ) -> f32 {
        // If there are no keyframes, return 0.0 as the default value
        if keyframes.is_empty() {
            return 0.0;
        }
        // If there's only one keyframe, return its value directly without interpolation
        if keyframes.len() == 1 {
            return keyframes[0].value;
        }

        // Check if the current index is valid and if the target sample falls within the range of the cached segment
        if let Some(idx) = self.current_index {
            let start_sample = keyframe_samples[idx];
            if let Some(&end_sample) = keyframe_samples.get(idx + 1) {
                if target_sample >= start_sample && target_sample < end_sample {
                    return Self::interpolate_segment(
                        &keyframes[idx],
                        &keyframes[idx + 1],
                        start_sample,
                        end_sample,
                        target_sample,
                    );
                }
            } else if target_sample >= start_sample {
                return keyframes[idx].value;
            }
        }

        // If the target sample is before the first keyframe, return the first keyframe's value
        if target_sample <= keyframe_samples[0] {
            self.current_index = Some(0);
            return keyframes[0].value;
        }

        // If the target sample is beyond the last keyframe, return the last keyframe's value
        if target_sample >= *keyframe_samples.last().unwrap() {
            let last_idx = keyframes.len() - 1;
            self.current_index = Some(last_idx);
            return keyframes[last_idx].value;
        }

        // The cache is out of date, so we need to find the correct segment for interpolation
        let next_idx = keyframe_samples.partition_point(|&s| s <= target_sample);
        let idx = next_idx.saturating_sub(1);
        self.current_index = Some(idx);

        let start_sample = keyframe_samples[idx];
        let end_sample = keyframe_samples[idx + 1];

        Self::interpolate_segment(
            &keyframes[idx],
            &keyframes[idx + 1],
            start_sample,
            end_sample,
            target_sample,
        )
    }

    #[inline(always)]
    fn interpolate_segment(
        k1: &Keyframe<f32>,
        k2: &Keyframe<f32>,
        start_sample: usize,
        end_sample: usize,
        target_sample: usize,
    ) -> f32 {
        let delta_samples = (end_sample - start_sample) as f32;
        if delta_samples <= 0.0 {
            return k1.value;
        }

        let elapsed = (target_sample - start_sample) as f32;
        let x = (elapsed / delta_samples).clamp(0.0, 1.0);
        let progress = k1.curve.evaluate_curve(x);

        k1.value + (k2.value - k1.value) * progress
    }
}
