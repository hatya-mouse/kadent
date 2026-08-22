mod project;
mod track;

fn restore_next_id(used_ids: &[u64]) -> u64 {
    used_ids.iter().max().map_or(0, |max_id| max_id + 1)
}
