use crate::{
    background_thread::{BackgroundTaskStatus, BackgroundThreadCommand},
    consts::{LARGE_BLOCK_SIZE, MEDIUM_BLOCK_SIZE, SMALL_BLOCK_SIZE},
    core::metadata::TrackType,
    ui::{theme, workspaces::EditorUi},
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    track::{RegionID, audio_track::AudioTrack},
};

/// Space between waveform and top/bottom of the region
const WAVEFORM_Y_CLEARANCE: f32 = 4.0;

impl EditorUi {
    // Generates waveforms for each region in the timeline.
    pub(crate) fn generate_waveforms(&mut self) {
        self.ui_state.timeline_state.waveforms.clear();

        let mut commands = Vec::new();
        for (track_id, track_meta) in &self.proj_ctx.project_meta.tracks {
            if track_meta.track_type == TrackType::Audio {
                let Some(track) = self
                    .proj_ctx
                    .project
                    .get_track(track_id)
                    .and_then(|track| track.as_any().downcast_ref::<AudioTrack>())
                else {
                    continue;
                };
                for region_id in track_meta.regions.keys() {
                    let Some(region) = track.get_region(region_id) else {
                        continue;
                    };
                    self.ui_state.status_bar_state.current_task =
                        Some(BackgroundTaskStatus::GenerateWaveform);
                    commands.push(BackgroundThreadCommand::GenerateWaveform {
                        track_id: *track_id,
                        region_id: *region_id,
                        source: region.data_source.clone(),
                        channels: region.info.channels,
                    });
                }
            }
        }

        // Send the commands to the background thread for processing
        for command in commands {
            self.push_background_job(command);
        }
    }

    pub(super) fn draw_waveform_in(
        &mut self,
        ui: &egui::Ui,
        track_id: TrackID,
        region_id: RegionID,
        region_rect: &egui::Rect,
    ) {
        let Some(region) = self
            .proj_ctx
            .project
            .get_track(&track_id)
            .and_then(|t| t.as_any().downcast_ref::<AudioTrack>())
            .and_then(|t| t.get_region(&region_id))
        else {
            return;
        };
        let rect_width = region_rect.width();

        // `region.data`/waveform peaks are stored at the region's own native sample rate, independent of the project's audio_ctx sample rate,
        // so we use region's max_duration to calculate the number of samples that should be displayed in the region's width
        let region_samples = if region.max_duration.0 > 0 {
            (region.info.frames as f64 * region.duration.0 as f64 / region.max_duration.0 as f64)
                as usize
        } else {
            0
        };

        // If samples per pixel is less than a certain threshold, draw the raw waveform directly
        // let samples_per_pixel = region_samples as f32 / rect_width;
        // if samples_per_pixel < SMALL_BLOCK_SIZE as f32 {
        //     self.draw_raw_waveform_in(ui, &track_id, &region_id, samples_per_pixel, region_rect);
        //     return;
        // }

        let Some(waveform_lod) = self
            .ui_state
            .timeline_state
            .waveforms
            .get(&(track_id, region_id))
        else {
            return;
        };

        // Select the most suitable LOD based on the current zoom level
        let large_len = waveform_lod.large.peaks.len();
        let medium_len = waveform_lod.medium.peaks.len();

        let (peaks_full, block_size) = if rect_width < large_len as f32 {
            (&waveform_lod.large.peaks, LARGE_BLOCK_SIZE)
        } else if rect_width < medium_len as f32 {
            (&waveform_lod.medium.peaks, MEDIUM_BLOCK_SIZE)
        } else {
            (&waveform_lod.small.peaks, SMALL_BLOCK_SIZE)
        };

        let visible_peak_count =
            ((region_samples as f32 / block_size as f32).ceil() as usize).min(peaks_full.len());
        let peaks = &peaks_full[..visible_peak_count];

        if peaks.is_empty() {
            return;
        }

        // Set up a painter and draw the waveform
        let painter = ui.painter_at(*region_rect);

        let y_center = region_rect.center().y;
        let half_height = region_rect.height() * 0.5 - WAVEFORM_Y_CLEARANCE;

        // Render only the visible portion of the waveform to optimize performance
        let visible_rect = region_rect.intersect(ui.clip_rect());
        if visible_rect.width() <= 0.0 {
            return;
        }

        // Calculate how many peaks are in the single pixel width
        let peaks_per_pixel = (peaks.len() as f32 / rect_width).max(1.0);
        // Then calculate the start and end pixel positions for rendering
        let start_pixel = (visible_rect.left() - region_rect.left()).floor().max(0.0) as usize;
        let end_pixel = (visible_rect.right() - region_rect.left()).ceil() as usize;

        let mut points = Vec::with_capacity((end_pixel - start_pixel) * 2);
        for pixel in start_pixel..end_pixel {
            let start_index = ((pixel as f32 * peaks_per_pixel) as usize).min(peaks.len());
            let end_index = (((pixel + 1) as f32 * peaks_per_pixel) as usize).min(peaks.len());
            if start_index >= end_index {
                continue;
            }

            // Find the min and max peaks in the range for this pixel
            let (min, max) = peaks[start_index..end_index].iter().fold(
                (f32::INFINITY, f32::NEG_INFINITY),
                |(min, max), &(low, high)| (min.min(low), max.max(high)),
            );

            // Then calculate the start and end y positions for the line segment to draw
            let x = region_rect.left() + pixel as f32;

            // Add line segments
            points.push(egui::pos2(x, y_center - max * half_height));
            points.push(egui::pos2(x, y_center - min * half_height));
        }

        // Draw the whole line
        painter.add(egui::Shape::line(
            points,
            egui::Stroke::new(1.0, theme::waveform_color()),
        ));
    }

    // fn draw_raw_waveform_in(
    //     &self,
    //     ui: &egui::Ui,
    //     track_id: &TrackID,
    //     region_id: &RegionID,
    //     samples_per_pixel: f32,
    //     region_rect: &egui::Rect,
    // ) {
    //     let Some(region) = self
    //         .proj_ctx
    //         .project
    //         .get_track(track_id)
    //         .and_then(|t| t.as_any().downcast_ref::<AudioTrack>())
    //         .and_then(|t| t.get_region(region_id))
    //     else {
    //         return;
    //     };

    //     let visible_rect = region_rect.intersect(ui.clip_rect());
    //     if visible_rect.width() <= 0.0 {
    //         return;
    //     }

    //     let painter = ui.painter_at(*region_rect);
    //     let y_center = region_rect.center().y;
    //     let half_height = region_rect.height() * 0.5 - WAVEFORM_Y_CLEARANCE;
    //     let channels = region.info.channels.max(1);

    //     // Find the start and end x positions for rendering the waveform
    //     let start_pixel = (visible_rect.left() - region_rect.left()).floor().max(0.0) as usize;
    //     let end_pixel = (visible_rect.right() - region_rect.left()).ceil() as usize;

    //     let mut points = Vec::with_capacity(end_pixel - start_pixel);
    //     for pixel in start_pixel..end_pixel {
    //         let frame = (pixel as f32 * samples_per_pixel) as usize;
    //         if frame >= region.info.frames {
    //             break;
    //         }
    //         let x = region_rect.left() + pixel as f32;
    //         let sample = region.data[frame * channels];
    //         points.push(egui::pos2(x, y_center - sample * half_height));
    //     }

    //     painter.add(egui::Shape::line(
    //         points,
    //         egui::Stroke::new(1.0, theme::waveform_color()),
    //     ));
    // }
}
