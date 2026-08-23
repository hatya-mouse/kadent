#[derive(Debug, Clone, Copy)]
pub(crate) enum CurveType {
    /// Linear interpolation between keyframes.
    Linear,
    /// Step interpolation between keyframes.
    Step,
    /// Smooth interpolation between keyframes with the given tension.
    Smooth { tension: f32 },
}

impl CurveType {
    #[inline(always)]
    pub(crate) fn evaluate_curve(&self, x: f32) -> f32 {
        match self {
            // Keep the value constantly until the next keyframe
            CurveType::Step => 0.0,
            // Linear interpolation between keyframes
            CurveType::Linear => x,
            // Smooth interpolation between keyframes with the given tension
            CurveType::Smooth { tension } => {
                if tension.abs() < 1e-4 {
                    // If the tension if small enough, just return the linear interpolation value
                    x
                } else {
                    (x * (1.0 + tension)) / (x * tension + 1.0)
                }
            }
        }
    }

    pub(crate) fn all() -> &'static [CurveType] {
        &[
            CurveType::Linear,
            CurveType::Step,
            CurveType::Smooth { tension: 1.0 },
        ]
    }

    pub(crate) fn is_same_type(&self, other: &CurveType) -> bool {
        matches!((self, other), (CurveType::Linear, CurveType::Linear))
            || matches!((self, other), (CurveType::Step, CurveType::Step))
            || matches!(
                (self, other),
                (CurveType::Smooth { .. }, CurveType::Smooth { .. })
            )
    }
}
