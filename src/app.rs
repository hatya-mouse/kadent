use crate::{
    metadata::{ProjectMeta, RegionMeta, TrackMeta},
    ui_state::{KnodiqUIState, dialog_state::TrackType},
};
use eframe::egui;
use kasl::core::localization::format_error;
use kasl_node::KaslNode;
use knodiq_engine::{
    audio_thread::{AudioThread, AudioThreadHandle, error::AudioError},
    data_types::{AudioContext, Beats},
    mixer::Project,
    track::{
        Track,
        note_track::{Note, NoteRegion, NoteTrack},
    },
};

pub struct KnodiqApp {
    /// A master source of the project.
    pub project: Project,
    /// Whether the audio is playing.
    pub is_playing: bool,
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
    /// The metadata of the project.
    pub project_meta: ProjectMeta,
    /// UI states to store the current UI state.
    pub ui_state: KnodiqUIState,
}

impl KnodiqApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Install the image loader
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // UI Setup
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);

        // Create a new project and spawn an audio thread
        let audio_ctx = AudioContext {
            channels: 2,
            sample_rate: 48000,
            buffer_size: 512,
            max_voices: 32,
        };
        let mut project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
        let mut project_meta = ProjectMeta::new();

        // Test project by adding some tracks and nodes
        Self::setup_project(&mut project, &mut project_meta, &audio_ctx);

        let thread_handle = AudioThread::spawn(audio_ctx, project.clone()).unwrap();

        Self {
            project,
            is_playing: false,
            thread_handle,
            errors: Vec::new(),
            project_meta,
            ui_state: KnodiqUIState::default(),
        }
    }

    fn setup_project(
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
            TrackType::NoteTrack,
        );
        let region_id = note_track.add_region(note_region);
        track_meta.add_region(region_id, region_meta);

        // Get the graph from the track
        let graph = note_track.get_graph_mut();

        let program = r#"
import std
import math/float
import convert
import knodiq

input notes = [knodiq.Voice(); knodiq.max_voices]
output sample = knodiq.zero_sample()

let pi2 = 6.28318530
let vib_rate = 6.0
let vib_depth = 0.003
let fm_ratio = 3.0
let fm_depth = 0.3

func main() {
    var out = 0.0

    var i = 0
    loop knodiq.max_voices {
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

    let fm_out = (out / convert.int_to_float(knodiq.max_voices))

    sample[0] = fm_out
    sample[1] = fm_out
}
        "#;

        let knodiq_lib = format!(
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

        let mut lib_paths = Vec::new();

        // Save the generated kasl library to temporary directory
        let app_data = dirs::data_dir()
            .expect("Could not get data dir")
            .join("knodiq");
        std::fs::create_dir_all(&app_data).unwrap();
        let kasl_path = app_data.join("knodiq.kasl");
        std::fs::write(&kasl_path, knodiq_lib).expect("Failed to write knodiq.kasl");
        lib_paths.push(app_data.to_str().unwrap().to_string());

        // Add the standard library directory
        if let Some(mut home_dir) = dirs::home_dir() {
            home_dir.push(".kasl/std/");
            if let Some(path_str) = home_dir.to_str() {
                lib_paths.push(path_str.to_string());
            }
        }

        // Create a new kasl node
        let mut kasl_node = KaslNode::new();
        kasl_node.set_search_paths(lib_paths);
        kasl_node.set_code(program.to_string());
        match kasl_node.compile() {
            Ok(()) => (),
            Err(err) => {
                eprintln!(
                    "Failed to compile the kasl code: {}",
                    err.iter()
                        .map(|record| format_error(record, "en"))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                return;
            }
        };

        // Add the node to the graph
        let node_id = graph.add_node(Box::new(kasl_node));
        // Connect the node
        let graph_input_id = graph.get_input_id();
        let graph_output_id = graph.get_output_id();
        graph
            .connect(&graph_input_id, "notes", &node_id, "notes")
            .unwrap();
        graph
            .connect(&node_id, "sample", &graph_output_id, "audio")
            .unwrap();

        // Add the note track to the project
        let track_id = project.add_track(Box::new(note_track));
        project_meta.add_track(track_id, track_meta);
    }

    fn base_style(ctx: &egui::Context) {
        ctx.global_style_mut(|style| {
            // Make labels unselectable by default
            style.interaction.selectable_labels = false;

            // Remove window shadows
            style.visuals.window_shadow = egui::Shadow::NONE;
        });
    }
}
