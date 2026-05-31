use crate::{
    kasl_node::KaslNode,
    metadata::{GraphMeta, NodeMeta, NodeType, ProjectMeta, RegionMeta, TrackMeta},
    ui::EditorUi,
    ui_state::dialog_state::TrackType,
};
use eframe::egui;
use kreniq_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
    track::{
        Track,
        note_track::{Note, NoteRegion, NoteTrack},
    },
};

impl EditorUi {
    pub(crate) fn setup_project(
        project_dir: &std::path::Path,
        project: &mut Project,
        project_meta: &mut ProjectMeta,
        audio_ctx: &AudioContext,
    ) {
        // Create a new note region
        let mut note_region = NoteRegion::new(Beats(0.0), Beats(8.0));
        let region_meta =
            RegionMeta::new("Synth Region 1".to_string(), Beats(0.0), Beats(8.0), None);
        // Add some notes to the region
        note_region.add_note(Note::new(Beats(0.0), Beats(3.9), 34.0, 1.0));
        note_region.add_note(Note::new(Beats(0.0), Beats(1.9), 86.0, 1.0));
        note_region.add_note(Note::new(Beats(2.0), Beats(0.5), 86.0, 1.0));
        note_region.add_note(Note::new(Beats(2.5), Beats(0.166), 84.0, 1.0));
        note_region.add_note(Note::new(Beats(2.666), Beats(0.166), 86.0, 1.0));
        note_region.add_note(Note::new(Beats(2.822), Beats(0.166), 84.0, 1.0));
        note_region.add_note(Note::new(Beats(3.0), Beats(0.3), 81.0, 1.0));
        note_region.add_note(Note::new(Beats(3.5), Beats(2.4), 79.0, 1.0));

        note_region.add_note(Note::new(Beats(4.0), Beats(3.9), 36.0, 1.0));
        note_region.add_note(Note::new(Beats(6.0), Beats(0.5), 81.0, 1.0));
        note_region.add_note(Note::new(Beats(6.5), Beats(0.166), 79.0, 1.0));
        note_region.add_note(Note::new(Beats(6.666), Beats(0.166), 81.0, 1.0));
        note_region.add_note(Note::new(Beats(6.822), Beats(0.166), 79.0, 1.0));
        note_region.add_note(Note::new(Beats(7.0), Beats(0.3), 77.0, 1.0));
        note_region.add_note(Note::new(Beats(7.5), Beats(0.5), 74.0, 1.0));

        note_region.add_note(Note::new(Beats(8.0), Beats(3.9), 38.0, 1.0));
        note_region.add_note(Note::new(Beats(8.0), Beats(0.5), 72.0, 1.0));
        note_region.add_note(Note::new(Beats(8.5), Beats(0.5), 74.0, 1.0));
        note_region.add_note(Note::new(Beats(9.0), Beats(0.5), 77.0, 1.0));
        note_region.add_note(Note::new(Beats(9.5), Beats(0.5), 72.0, 1.0));
        note_region.add_note(Note::new(Beats(10.0), Beats(0.5), 74.0, 1.0));
        note_region.add_note(Note::new(Beats(10.5), Beats(1.4), 81.0, 1.0));

        note_region.add_note(Note::new(Beats(12.0), Beats(1.9), 38.0, 1.0));
        note_region.add_note(Note::new(Beats(12.0), Beats(0.5), 72.0, 1.0));
        note_region.add_note(Note::new(Beats(12.5), Beats(0.5), 74.0, 1.0));
        note_region.add_note(Note::new(Beats(13.0), Beats(0.5), 77.0, 1.0));
        note_region.add_note(Note::new(Beats(13.5), Beats(0.5), 72.0, 1.0));
        note_region.add_note(Note::new(Beats(14.0), Beats(1.9), 41.0, 1.0));
        note_region.add_note(Note::new(Beats(14.0), Beats(0.5), 74.0, 1.0));
        note_region.add_note(Note::new(Beats(14.5), Beats(0.5), 81.0, 1.0));
        note_region.add_note(Note::new(Beats(15.0), Beats(0.5), 79.0, 1.0));
        note_region.add_note(Note::new(Beats(15.5), Beats(0.5), 81.0, 1.0));

        // Create a new track
        let mut note_track = NoteTrack::new(audio_ctx.clone());
        let mut track_meta = TrackMeta::new(
            "Synth".to_string(),
            egui::Color32::from_rgb(42, 180, 255),
            TrackType::Note,
        );
        track_meta.graph = GraphMeta::from_graph(note_track.get_graph());
        let region_id = note_track.add_region(note_region);
        track_meta.add_region(region_id, region_meta);

        // Get the graph from the track
        let graph = note_track.get_graph_mut();

        let program = r#"
        import std
        import math/float
        import convert
        import kreniq

        input notes = [kreniq.Voice(); kreniq.max_voices]
        output sample = kreniq.zero_sample()

        let pi2 = 6.28318530
        let vib_rate = 6.0
        let vib_depth = 0.003
        let fm_ratio = 3.0
        let fm_depth = 0.3

        func main() {
            var out = 0.0

            var i = 0
            loop kreniq.max_voices {
                if notes[i].is_active {
                    let t = notes[i].age
                    let base_freq = 440.0 * float.pow(2.0, (notes[i].pitch - 69.0) / 12.0)

                    if base_freq > 500.0 {
                        let vib = float.fast_sin(pi2 * vib_rate * t) * vib_depth
                        let vib_freq = base_freq * (1.0 + vib)
                        let mod_sig = float.fast_sin(vib_freq * fm_ratio * t * pi2) * fm_depth
                        let carrier = float.sgn(float.fast_sin(vib_freq * t * pi2 + mod_sig))
                        out = out + carrier * notes[i].velocity * 0.3
                    } else {
                        let mod_sig = float.fast_sin(base_freq * fm_ratio * t * pi2) * fm_depth
                        let carrier = float.sgn(float.sin(base_freq * t * pi2 + mod_sig))
                        out = out + carrier * notes[i].velocity * 0.3
                    }
                }
                i = i + 1
            }

            let fm_out = (out / convert.int_to_float(kreniq.max_voices))

            sample[0] = fm_out
            sample[1] = fm_out
        }
                "#;

        let kreniq_lib = format!(
            r#"
        let channels = {}
        let sample_rate = {}
        let max_voices = {}
        let buffer_size = {}

        typealias Sample = [Float; 2]

        func zero_sample() -> Sample {{
            return [0.0; channels]
        }}

        struct Voice {{
            var pitch = 0.0
            var velocity = 0.0
            var age = 0.0
            var is_active = false
        }}
                        "#,
            audio_ctx.channels, audio_ctx.sample_rate, audio_ctx.max_voices, audio_ctx.buffer_size
        );

        // Write the kreniq standard library to the app data directory
        let app_data = dirs::data_dir()
            .expect("Could not get data dir")
            .join("kreniq");
        std::fs::create_dir_all(&app_data).unwrap();
        let kasl_path = app_data.join("kreniq.kasl");
        std::fs::write(&kasl_path, kreniq_lib).expect("Failed to write kreniq.kasl");

        // Write the main KASL program into the project's kasl/ directory
        let kasl_dir = project_dir.join("kasl");
        std::fs::create_dir_all(&kasl_dir).unwrap();
        std::fs::write(kasl_dir.join("main.kasl"), program).expect("Failed to write main.kasl");

        // Create a new kasl node pointing to the relative path
        let mut kasl_node = KaslNode::new();
        kasl_node.set_search_paths(project_meta.kasl_search_paths.clone());
        kasl_node.set_file_path("kasl/main.kasl".to_string());
        kasl_node.set_project_dir(project_dir.to_path_buf());
        match kasl_node.compile() {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Failed to compile the kasl code: {}", err);
                return;
            }
        };

        // Add the node to the graph
        let node_id = graph.add_node(Box::new(kasl_node));
        track_meta.graph.set_node_meta(
            node_id,
            NodeMeta::new(
                NodeType::Kasl,
                "KASL Node".to_string(),
                egui::pos2(0.0, 0.0),
            ),
        );
        // Connect the node
        let graph_input_id = graph.get_input_id();
        let graph_output_id = graph.get_output_id();
        graph.add_edge_unchecked((graph_input_id, 0, node_id, 0));
        graph.add_edge_unchecked((node_id, 0, graph_output_id, 0));

        // Add the note track to the project
        let track_id = project.add_track(Box::new(note_track));
        project_meta.add_track(track_id, track_meta);
    }
}
