use crate::core::audio_engine::graph::OutputKey;
use crate::core::audio_engine::{
    data_types::PlaybackContext, graph::InputSource, mixer::ProjectData,
};
use crate::core::kasl_node::KaslNode;
use std::path::Path;

/// Initialize all nodes in the project data.
pub(crate) fn init_nodes(
    project: &mut ProjectData,
    search_path: Option<String>,
    project_dir: &Path,
    playback_ctx: &PlaybackContext,
) {
    let search_paths = search_path.into_iter().collect::<Vec<_>>();
    for track in project.tracks.values_mut() {
        for node in track.get_graph_mut().get_node_map_mut().values_mut() {
            node.update_type_info();

            // For the KASL node, set the search paths and project directory, then compile it
            if let Some(kasl_node) = node.as_any_mut().downcast_mut::<KaslNode>() {
                kasl_node.set_search_paths(search_paths.clone());
                kasl_node.set_project_dir(project_dir.to_path_buf());

                if let Err(errors) = kasl_node.compile(playback_ctx) {
                    eprintln!("KaslNode compile failed on load: {:?}", errors);
                }
            }
        }
    }

    // Re-apply edges with type checking now that nodes are compiled.
    // Drops any edges that reference ports no longer valid after a KASL source change.
    for track in project.tracks.values_mut() {
        let graph = track.get_graph_mut();
        for (to, input_source) in graph.get_input_sources().clone() {
            if let InputSource::Edge(OutputKey(from_node, from_output)) = input_source {
                graph.remove_edge(to);
                graph.add_edge(OutputKey(from_node, from_output), to).ok();
            }
        }
    }
}
