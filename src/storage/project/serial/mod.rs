mod audio_track;
mod automation;
mod graph;
mod misc;
mod node;
mod note_track;
mod project;
mod tempo_map;
mod track;

fn restore_next_id<T>(used_ids: &[T]) -> u64
where
    T: Into<u64> + Copy,
{
    used_ids
        .iter()
        .map(|id| Into::into(*id))
        .max()
        .map_or(0, |max_id| max_id + 1)
}
