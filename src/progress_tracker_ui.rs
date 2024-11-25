use crate::achievements::Sprint;
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
                            let lab = self
                                .heading(ui, self.language.lab.clone())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.settings.show_passed_labs {
                                        false => self.language.click_to_show_passed_labs.clone(),
                                        true => self.language.click_to_hide_passed_labs.clone(),
                                    });
                                });

                            if lab.clicked() {
                                self.settings.show_passed_labs = !self.settings.show_passed_labs;
                            }

                            ui.heading(self.language.date.clone());
                            let optimal = self
                                .heading(ui, self.language.optimal.clone())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            self.language.click_to_show_achievements_done.clone()
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            self.language.click_to_show_achievements_left.clone()
                                        }
                                    });
                                });

                            if optimal.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let minimum = self
                                .heading(ui, self.language.minimum.clone())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            self.language.click_to_show_achievements_done.clone()
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            self.language.click_to_show_achievements_left.clone()
                                        }
                                    });
                                });

                            if minimum.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let target = self
                                .heading(ui, self.language.target.clone())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            self.language.click_to_show_achievements_done.clone()
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            self.language.click_to_show_achievements_left.clone()
                                        }
                                    });
                                });

                            if target.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            let current_minimum = self
                                .heading(ui, self.language.minimum_to_reach_target_grade.clone())
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_ui(|ui| {
                                    ui.label(match self.progress_tracker.mode {
                                        progress_tracker::ProgressTrackerMode::Left => {
                                            self.language.click_to_show_achievements_done.clone()
                                        }
                                        progress_tracker::ProgressTrackerMode::Done => {
                                            self.language.click_to_show_achievements_left.clone()
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
                                    ui.label(format!("{} {}", self.language.lab, i + 1));
                                    ui.label(lab.date.format("%a %b %d, %Y").to_string());
                                    ui.label(lab.optimal.to_string());
                                    ui.label(lab.minimum.to_string());
                                    ui.label(lab.target.to_string());
                                    ui.label(lab.current_minimum.to_string());
                                    ui.end_row();
                                }
                            }
                        });
                    ui.allocate_space(egui::vec2(0.0, 50.0));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.grade3,
                        self.achievements
                            .iter()
                            .filter(|e| e.grade == 3 && e.done)
                            .count(),
                        self.achievements.iter().filter(|e| e.grade == 3).count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.grade4,
                        self.achievements
                            .iter()
                            .filter(|e| e.grade == 4 && e.done)
                            .count(),
                        self.achievements.iter().filter(|e| e.grade == 4).count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.grade5,
                        self.achievements
                            .iter()
                            .filter(|e| e.grade == 5 && e.done)
                            .count(),
                        self.achievements.iter().filter(|e| e.grade == 5).count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.sprint1,
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint1 && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint1)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.sprint2,
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint2 && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint2)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.sprint3,
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint3 && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint3)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.sprint4,
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint4 && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Sprint4)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.project,
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Project && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Project)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.other,
                        self.achievements
                            .iter()
                            .filter(|e| (e.sprint == Sprint::Unclear || e.sprint == Sprint::Lab)
                                && e.done)
                            .count(),
                        self.achievements
                            .iter()
                            .filter(|e| e.sprint == Sprint::Unclear || e.sprint == Sprint::Lab)
                            .count(),
                    ));
                    ui.label(format!(
                        "{}: {}/{}",
                        self.language.total,
                        self.achievements.iter().filter(|e| e.done).count(),
                        self.achievements.len(),
                    ));
                });
        });
    }
}
