use crate::core::audio_engine::{
    graph::error::GraphError, thread::AudioError, track::error::TrackError,
};
use crate::ui::theme;
use eframe::egui;

pub(super) fn draw_error_item(ui: &mut egui::Ui, error: &AudioError) {
    let message = error_message(error);

    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(message)
                        .size(theme::normal_font_size())
                        .color(theme::primary_fg(ui.visuals().dark_mode))
                        .monospace(),
                )
                .selectable(false)
                .wrap(),
            );
        });

    let stroke = egui::Stroke::new(0.5, theme::border_color(ui.visuals().dark_mode));
    let rect = ui.min_rect();
    ui.painter()
        .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);
}

fn error_message(error: &AudioError) -> String {
    match error {
        AudioError::TrackPrepareFailed(track_id, e) => {
            format!(
                "Track {track_id:?} failed to prepare: {}",
                track_error_message(e)
            )
        }
        AudioError::ThreadSpawnFailed(e) => format!("Failed to spawn thread: {e}"),
        AudioError::PlayStreamError(e) => format!("Play stream error: {e}"),
        AudioError::BuildStreamError(e) => format!("Build stream error: {e}"),
        AudioError::CommandFailed(_) => "Audio command failed".to_string(),
    }
}

fn track_error_message(error: &TrackError) -> String {
    match error {
        TrackError::GraphError(e) => graph_error_message(e),
        TrackError::ThreadSpawnFailed(e) => format!("Failed to spawn thread: {e}"),
        TrackError::RenderWorkerPanicked => format!("Audio thread render worker has panicked"),
    }
}

fn graph_error_message(error: &GraphError) -> String {
    match error {
        GraphError::NodeNotFound(id) => format!("Node not found: {id:?}"),
        GraphError::NodeError(e) => format!("Node error: {e}"),
        GraphError::NodeCycle(id) => format!("Cycle detected at node {id:?}"),
        GraphError::OutputTypeUnavailable(id, idx) => {
            format!("Output type unavailable: port {idx} on {id:?}")
        }
        GraphError::InputTypeUnavailable(id, idx) => {
            format!("Input type unavailable: port {idx} on {id:?}")
        }
        GraphError::NodeTypeMismatch((src, src_port, dst, dst_port)) => {
            format!("Type mismatch: {src:?}[{src_port}] -> {dst:?}[{dst_port}]")
        }
    }
}
