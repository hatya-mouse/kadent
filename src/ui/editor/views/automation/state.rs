use kadent_engine::node::builtin::CurveType;

#[derive(Default)]
pub(crate) struct AutomationState {
    /// The last used curve type for the new keyframe.
    pub last_curve_type: Option<CurveType>,
}
