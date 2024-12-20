mod achievement_csv;
mod achievement_ui;
mod achievements;
mod application;
mod burndown;
mod default_values;
mod git;
mod langs;
mod main_ui;
mod progress_tracker;
mod progress_tracker_ui;
mod settings_ui;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let mut native_options = NativeOptions::default();

    native_options.viewport.maximized = Some(true);

    eframe::run_native(
        "Achievements Enhanced",
        native_options,
        Box::new(|cc| Ok(Box::new(application::Application::new(cc)))),
    )
}
