use crate::application::Application;
use crate::progress_tracker;

use chrono::Local;
use eframe::egui;

impl Application {
    pub fn progress_tracker_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ctx, ui);

            egui::ScrollArea::both()
                .stick_to_right(true)
                .show(ui, |ui| {
                    egui::Grid::new("Progress Tracker Grid")
                        .spacing(egui::vec2(
                            self.settings.font_size * 5.0,
                            self.settings.font_size * 2.0,
                        ))
                        .num_columns(7)
                        .striped(true)
                        .show(ui, |ui| {
                            let lab = ui
                                .heading("Lab")
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.settings.show_passed_labs {
                                        true => "Click to hide passed labs",
                                        false => "Click to show passed labs",
                                    });
                                });

                            if lab.clicked() {
                                self.settings.show_passed_labs = !self.settings.show_passed_labs;
                            }

                            ui.heading("Date");
                            let optimal = ui
                                .heading("Optimal")
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            "Click to show achievements done"
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            "Click to show achievements left"
                                        }
                                    });
                                });

                            if optimal.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let minimum = ui
                                .heading("Minimum")
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            "Click to show achievements done"
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            "Click to show achievements left"
                                        }
                                    });
                                });

                            if minimum.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let target = ui
                                .heading("Target")
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            "Click to show achievements done"
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            "Click to show achievements left"
                                        }
                                    });
                                });

                            if target.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let current_minimum = ui
                                .heading("Minimum to reach target grade")
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            "Click to show achievements done"
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            "Click to show achievements left"
                                        }
                                    });
                                });

                            if current_minimum.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            ui.allocate_space(egui::vec2(0.0, 0.0));
                            ui.end_row();
                            for (i, lab) in
                                self.progress_tracker.labs.clone().into_iter().enumerate()
                            {
                                if Local::now() <= lab.date || self.settings.show_passed_labs {
                                    ui.label(format!("Lab {}", i + 1));
                                    ui.label(lab.date.format("%a %b %d, %Y").to_string());
                                    ui.label(lab.optimal.to_string());
                                    ui.label(lab.minimum.to_string());
                                    ui.label(lab.target.to_string());
                                    ui.label(lab.current_minimum.to_string());
                                    ui.end_row();
                                }
                            }
                        });
                });
        });
    }
}
