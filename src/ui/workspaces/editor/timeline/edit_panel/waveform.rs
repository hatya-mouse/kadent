use crate::{
    background_thread::{BackgroundTaskStatus, BackgroundThreadCommand, WaveformLod},
    consts::{LARGE_BLOCK_SIZE, MEDIUM_BLOCK_SIZE, SMALL_BLOCK_SIZE},
    core::metadata::TrackType,
    ui::{theme, workspaces::EditorUi},
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    timing::TimeBounds,
    track::{RegionID, audio_track::AudioTrack},
    utils::{samples_per_tick, seconds_to_samples},
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
        // Get the region and the waveform LOD data
        let Some(region) = self
            .proj_ctx
            .project
            .get_track(&track_id)
            .and_then(|t| t.as_any().downcast_ref::<AudioTrack>())
            .and_then(|t| t.get_region(&region_id))
        else {
            return;
        };
        let Some(region_meta) = self
            .proj_ctx
            .project_meta
            .get_track(&track_id)
            .and_then(|t| t.regions.get(&region_id))
        else {
            return;
        };
        let Some(waveform_lod) = self
            .ui_state
            .timeline_state
            .waveforms
            .get(&(track_id, region_id))
        else {
            return;
        };

        let rect_width = region_rect.width();
        if rect_width <= 0.0 {
            return;
        }

        // If samples per pixel is less than a certain threshold, draw the raw waveform directly
        let full_data_frames = waveform_lod.data.info.frames;
        let region_data_frames = calculate_data_frames(
            &region_meta.bounds,
            waveform_lod.data.info.sample_rate,
            region.bpm,
            self.ui_state.audio_ctx.resolution,
        );
        if region_data_frames == 0 {
            return;
        }

        // Calculate the number of source frames per pixel to determine if we should draw the raw waveform or use LOD
        let src_frames_per_pixel = region_data_frames as f32 / rect_width;
        if src_frames_per_pixel < SMALL_BLOCK_SIZE as f32 {
            self.draw_raw_waveform_in(
                ui,
                waveform_lod,
                region.data_offset,
                src_frames_per_pixel,
                region_rect,
            );
            return;
        }

        // Select the most suitable LOD based on the current zoom level
        let (peaks_full, block_size) = if src_frames_per_pixel >= LARGE_BLOCK_SIZE as f32 {
            (&waveform_lod.large.peaks, LARGE_BLOCK_SIZE)
        } else if src_frames_per_pixel >= MEDIUM_BLOCK_SIZE as f32 {
            (&waveform_lod.medium.peaks, MEDIUM_BLOCK_SIZE)
        } else {
            (&waveform_lod.small.peaks, SMALL_BLOCK_SIZE)
        };
        if peaks_full.is_empty() {
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

        // Calculate the start and end pixel positions for rendering
        let start_pixel = (visible_rect.left() - region_rect.left()).floor().max(0.0) as usize;
        let end_pixel = (visible_rect.right() - region_rect.left()).ceil() as usize;

        let mut points = Vec::with_capacity((end_pixel - start_pixel) * 2);
        for pixel in start_pixel..end_pixel {
            let pixel_start_frame =
                region.data_offset + (pixel as f32 * src_frames_per_pixel) as usize;
            let pixel_end_frame =
                region.data_offset + ((pixel + 1) as f32 * src_frames_per_pixel) as usize;
            if pixel_start_frame >= full_data_frames {
                continue;
            }

            // Calculate the start and end indices for the peaks in the selected LOD
            let start_peak_idx = (pixel_start_frame / block_size).min(peaks_full.len());
            let end_peak_idx = pixel_end_frame
                .div_ceil(block_size)
                .min(peaks_full.len())
                .max(start_peak_idx + 1);

            if start_peak_idx >= peaks_full.len() {
                break;
            }

            let slice = &peaks_full[start_peak_idx..end_peak_idx.min(peaks_full.len())];
            if slice.is_empty() {
                continue;
            }

            // Find the min and max peaks in the range for this pixel
            let (min, max) = slice.iter().fold(
                (f32::INFINITY, f32::NEG_INFINITY),
                |(min, max), &(low, high)| (min.min(low), max.max(high)),
            );

            // Then calculate the start and end y positions for the line segment to draw
            let x = region_rect.left() + pixel as f32;
            points.push(egui::pos2(x, y_center - max * half_height));
            points.push(egui::pos2(x, y_center - min * half_height));
        }

        // Draw the whole line
        painter.add(egui::Shape::line(
            points,
            egui::Stroke::new(1.0, theme::waveform_color()),
        ));
    }

    fn draw_raw_waveform_in(
        &self,
        ui: &egui::Ui,
        waveform_lod: &WaveformLod,
        data_offset: usize,
        samples_per_pixel: f32,
        region_rect: &egui::Rect,
    ) {
        let visible_rect = region_rect.intersect(ui.clip_rect());
        if visible_rect.width() <= 0.0 {
            return;
        }

        let painter = ui.painter_at(*region_rect);
        let y_center = region_rect.center().y;
        let half_height = region_rect.height() * 0.5 - WAVEFORM_Y_CLEARANCE;
        let channels = waveform_lod.data.info.channels.max(1);

        // Find the start and end x positions for rendering the waveform
        let start_pixel = (visible_rect.left() - region_rect.left()).floor().max(0.0) as usize;
        let end_pixel = (visible_rect.right() - region_rect.left()).ceil() as usize;

        let mut points = Vec::with_capacity(end_pixel - start_pixel);
        for pixel in start_pixel..end_pixel {
            let frame = data_offset + (pixel as f32 * samples_per_pixel) as usize;
            if frame >= waveform_lod.data.info.frames {
                // Break when reached the end of the waveform data
                break;
            }
            let x = region_rect.left() + pixel as f32;
            let sample = waveform_lod
                .data
                .get_sample(frame * channels)
                .unwrap_or(&0.0);
            points.push(egui::pos2(x, y_center - sample * half_height));
        }

        if !points.is_empty() {
            painter.add(egui::Shape::line(
                points,
                egui::Stroke::new(1.0, theme::waveform_color()),
            ));
        }
    }
}

/// Calculates the number of source frames consumed within the given TimeBounds.
pub fn calculate_data_frames(
    bounds: &TimeBounds,
    data_sample_rate: u64,
    region_bpm: f64,
    resolution: u64,
) -> usize {
    match *bounds {
        TimeBounds::Musical { duration, .. } => {
            let samples_per_tick = samples_per_tick(data_sample_rate, region_bpm, resolution);
            (duration.0.max(0) as f64 * samples_per_tick).round() as usize
        }
        TimeBounds::Time {
            duration_seconds, ..
        } => seconds_to_samples(duration_seconds.max(0.0), data_sample_rate),
    }
}
