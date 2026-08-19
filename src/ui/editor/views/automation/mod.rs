use crate::{
    consts::TIMELINE_LEFT_PADDING,
    ui::{
        EditorState,
        components::centered_text::centered_text,
        editor::{PanelView, TimelineCoord, views::PanelViewState},
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};
use kadent_engine::node::builtin::{AutomationNode, AutomationTrack};
use uuid::Uuid;

const VERTICAL_PADDING: f32 = 20.0;

impl EditorState {
    pub(in crate::ui::editor) fn automation(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let PanelViewState::Automation(timeline_coord) = self
            .views
            .get_panel_state_or_insert(panel_id, PanelView::Automation, || {
                PanelViewState::Automation(TimelineCoord::new(
                    80.0,
                    1e-2,
                    egui::vec2(TIMELINE_LEFT_PADDING, 0.0),
                ))
            })
            .clone()
        else {
            return;
        };
        let timeline_width = self.timeline_content_width(&timeline_coord);

        // Get the track and the selected automation node
        let (Some(track_id), Some(node_id)) = (self.selection.track_id(), self.selection.node_id())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };
        let Some(automation_node) = self
            .project
            .data
            .get_track_mut(&track_id)
            .and_then(|track| track.get_graph_mut().get_node_mut(&node_id))
            .and_then(|node| node.as_any_mut().downcast_mut::<AutomationNode>())
        else {
            centered_text(ui, "No Automation Node Selected");
            return;
        };

        // Draw the automation timeline and keyframes
        egui::ScrollArea::both()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.set_min_width(timeline_width);
            });

        // Draw keyframes and curve based on the type of automation track
        let track = &mut automation_node.track;
        match track {
            AutomationTrack::Float { keyframes, .. } => {}
            AutomationTrack::Int { keyframes, .. } => {}
            AutomationTrack::Bool { keyframes, .. } => {}
        }
    }
}
