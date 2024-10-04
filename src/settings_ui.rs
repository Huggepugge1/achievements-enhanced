use crate::application::Application;
use crate::langs::{get_english, get_swedish, Langs};
use eframe::egui;

impl Application {
    pub fn settings_ui(&mut self, ctx: &egui::Context) {
        let mut font_size = self.settings.font_size;
        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ctx, ui);
            egui::Grid::new("Settings Grid")
                .spacing(egui::vec2(
                    self.settings.font_size * 3.0,
                    self.settings.font_size / 2.0,
                ))
                .show(ui, |ui| {
                    self.heading(ui, self.language.font_size.clone());
                    let slider = ui.add(
                        egui::Slider::new(&mut font_size, 10.0..=30.0)
                            .integer()
                            .show_value(false),
                    );
                    if slider.drag_stopped() {
                        self.settings.font_size = font_size;
                        self.settings.save();
                    }
                    ui.end_row();
                    self.heading(ui, self.language.dark_mode.clone());
                    let dark_mode = self.settings.dark_mode;
                    ui.toggle_value(&mut self.settings.dark_mode, dark_mode.to_string());
                    ui.end_row();
                    self.heading(ui, self.language.target_grade.clone());
                    let slider = ui.add(egui::Slider::new(
                        &mut self.progress_tracker.target_grade,
                        3..=5,
                    ));
                    if slider.drag_stopped() {
                        self.progress_tracker.update();
                        self.settings.save();
                    }
                    ui.end_row();
                    self.heading(ui, self.language.max_per_lab.clone());
                    let slider = ui.add(egui::Slider::new(
                        &mut self.progress_tracker.max_per_lab,
                        1..=4,
                    ));
                    if slider.drag_stopped() {
                        self.progress_tracker.update();
                        self.settings.save();
                    }
                    ui.end_row();
                    self.heading(ui, self.language.language.clone());
                    if ui.button(self.settings.language.to_string()).clicked() {
                        self.settings.language = match self.settings.language {
                            Langs::English => Langs::Swedish,
                            Langs::Swedish => Langs::English,
                        };
                        self.language = match self.settings.language {
                            Langs::English => get_english(),
                            Langs::Swedish => get_swedish(),
                        };
                        self.settings.save();
                    }
                });
        });
    }
}
