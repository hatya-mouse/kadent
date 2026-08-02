//! Shared math for cursor-centered zoom: given a scale change (e.g. pixels-per-beat or
//! pixels-per-note), compute the scroll offset needed so that whatever value was under the
//! cursor before the zoom stays under the cursor after it.

/// Returns the new scroll offset along one axis so that `value_at_cursor` (in the axis's domain
/// units, e.g. beats or MIDI note number) stays at the same screen position after the scale
/// changes from `old_scale` to `new_scale` (both "pixels per domain unit").
pub(crate) fn zoom_scroll_offset(
    current_offset: f32,
    value_at_cursor: f32,
    old_scale: f32,
    new_scale: f32,
) -> f32 {
    current_offset + value_at_cursor * (new_scale - old_scale)
}
