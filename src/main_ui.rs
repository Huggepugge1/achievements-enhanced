use crate::application::{Application, Filters};
use eframe::egui;

impl Application {
    pub fn menu_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.style_mut().spacing.item_spacing = egui::vec2(30.0, 0.0);
        egui::menu::bar(ui, |ui| {
            let file_button = ui.button(egui::RichText::new("File").font(egui::FontId::new(
                self.settings.font_size * 1.5,
                egui::FontFamily::Proportional,
            )));
            let file_popup_id = ui.make_persistent_id("File popup");
            egui::containers::popup::popup_below_widget(
                ui,
                file_popup_id,
                &file_button,
                egui::containers::popup::PopupCloseBehavior::CloseOnClickOutside,
                |ui| {
                    let save_button = ui
                        .label("Save")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if save_button.hovered() {
                        save_button.clone().highlight();
                    }
                    if save_button.clicked() {
                        if let Err(e) = self.save_achievements() {
                            eprintln!("Error saving achievements: {}", e);
                        };
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                },
            );
            let edit_button = ui.button(egui::RichText::new("Edit").font(egui::FontId::new(
                self.settings.font_size * 1.5,
                egui::FontFamily::Proportional,
            )));
            let edit_popup_id = ui.make_persistent_id("Edit popup");
            egui::containers::popup::popup_below_widget(
                ui,
                edit_popup_id,
                &edit_button,
                egui::containers::popup::PopupCloseBehavior::CloseOnClickOutside,
                |ui| {
                    let clear_done_button = ui
                        .label("Clear Done")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_done_button.hovered() {
                        clear_done_button.clone().highlight();
                    }
                    if clear_done_button.clicked() {
                        self.clear_done();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let clear_present_soon_button = ui
                        .label("Clear Present Soon")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_present_soon_button.hovered() {
                        clear_present_soon_button.clone().highlight();
                    }
                    if clear_present_soon_button.clicked() {
                        self.clear_present_soon();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let clear_filters_button = ui
                        .label("Clear Filters")
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_filters_button.hovered() {
                        clear_filters_button.clone().highlight();
                    }
                    if clear_filters_button.clicked() {
                        self.filters = Filters::new();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                },
            );
            let achievements_button =
                ui.button(egui::RichText::new("Achievements").font(egui::FontId::new(
                    self.settings.font_size * 1.5,
                    egui::FontFamily::Proportional,
                )));

            let progress_tracker_button = ui.button(egui::RichText::new("Progress Tracker").font(
                egui::FontId::new(
                    self.settings.font_size * 1.5,
                    egui::FontFamily::Proportional,
                ),
            ));

            if file_button.clicked() {
                ui.memory_mut(|memory| {
                    memory.open_popup(file_popup_id);
                });
            } else if edit_button.clicked() {
                ui.memory_mut(|memory| {
                    memory.open_popup(edit_popup_id);
                });
            } else if achievements_button.clicked() {
                self.active_window = crate::application::ActiveWindow::Achievements;
            } else if progress_tracker_button.clicked() {
                self.active_window = crate::application::ActiveWindow::ProgressTracker;
            }
        });
        ui.allocate_space(egui::vec2(0.0, 15.0));
    }
}
