use crate::application::Application;
use crate::progress_tracker;
use eframe::egui;

impl Application {
    pub fn progress_tracker_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ctx, ui);

            ui.heading("Target Grade");
            let grade_button = ui.button(self.progress_tracker.target_grade.to_string());
            let grade_popup_id = ui.make_persistent_id("Grade popup");
            egui::containers::popup::popup_below_widget(
                ui,
                grade_popup_id,
                &grade_button,
                egui::containers::popup::PopupCloseBehavior::CloseOnClickOutside,
                |ui| {
                    let save_button = ui
                        .label("3")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if save_button.hovered() {
                        save_button.clone().highlight();
                    }
                    if save_button.clicked() {
                        self.progress_tracker.target_grade = 3;
                        self.progress_tracker.update();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let save_button = ui
                        .label("4")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if save_button.hovered() {
                        save_button.clone().highlight();
                    }
                    if save_button.clicked() {
                        self.progress_tracker.target_grade = 4;
                        self.progress_tracker.update();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let save_button = ui
                        .label("5")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if save_button.hovered() {
                        save_button.clone().highlight();
                    }
                    if save_button.clicked() {
                        self.progress_tracker.target_grade = 5;
                        self.progress_tracker.update();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                },
            );

            if grade_button.clicked() {
                ui.memory_mut(|memory| {
                    memory.toggle_popup(grade_popup_id);
                });
            }
            egui::ScrollArea::both()
                .stick_to_right(true)
                .show(ui, |ui| {
                    egui::Grid::new(0)
                        .spacing(egui::vec2(50.0, 15.0))
                        .min_row_height(self.settings.line_height)
                        .num_columns(3)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.heading("Lab");
                            ui.heading("Date");
                            let perfect_student = ui
                                .heading("Perfect Student")
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

                            if perfect_student.clicked() {
                                self.progress_tracker.mode = self.progress_tracker.mode.toggle();
                                self.progress_tracker.update();
                            }

                            ui.end_row();
                            for (i, lab) in
                                self.progress_tracker.labs.clone().into_iter().enumerate()
                            {
                                ui.label(format!("Lab {}", i + 1));
                                ui.label(lab.date.format("%a %b %d, %Y").to_string());
                                ui.label(lab.perfect_student.to_string());
                                ui.allocate_space(egui::vec2(10.0, 0.0));
                                ui.end_row();
                            }
                        });
                });
        });
    }
}
