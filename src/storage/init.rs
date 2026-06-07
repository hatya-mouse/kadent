use crate::core::kasl_node::KaslNode;
use kadent_engine::mixer::Project;
use std::path::Path;

/// Initialize all KaslNodes in the project with their search paths and project directory,
/// then compile them so the blueprint (and port names) are ready before the first UI render.
/// After compilation, edges are re-applied with type checking so stale edges are dropped.
pub(crate) fn init_kasl_nodes(project: &mut Project, search_paths: &[String], project_dir: &Path) {
    for track in project.tracks.values_mut() {
        for node in track.get_graph_mut().get_node_map_mut().values_mut() {
            if let Some(kasl_node) = node.as_any_mut().downcast_mut::<KaslNode>() {
                kasl_node.set_search_paths(search_paths.to_vec());
                kasl_node.set_project_dir(project_dir.to_path_buf());

                if let Err(errors) = kasl_node.compile() {
                    eprintln!("KaslNode compile failed on load: {:?}", errors);
                }
            }
        }
    }

    // Re-apply edges with type checking now that nodes are compiled.
    // Drops any edges that reference ports no longer valid after a KASL source change.
    for track in project.tracks.values_mut() {
        let graph = track.get_graph_mut();
        let edges: Vec<_> = graph.get_edges().clone();
        for &edge in &edges {
            let _ = graph.remove_edge(edge);
        }
        for edge in edges {
            if let Err(e) = graph.add_edge(edge) {
                eprintln!("Dropping stale edge on load: {e:?}");
            }
        }
    }
}
