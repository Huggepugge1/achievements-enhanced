mod achievement_csv;
mod achievement_ui;
mod achievements;
mod application;
mod default_values;
mod git;
mod langs;
mod main_ui;
mod progress_tracker;
mod progress_tracker_ui;
mod settings_ui;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(&["/C", "python3", "updater.py", "v2.0.0"])
        .output()
        .expect("failed to execute process");

    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("python3")
        .args(&["updater.py", "v2.0.0"])
        .output()
        .expect("failed to execute process");

    let mut native_options = NativeOptions::default();

    native_options.viewport.maximized = Some(true);

    eframe::run_native(
        "Achievements Enhanced",
        native_options,
        Box::new(|cc| Ok(Box::new(application::Application::new(cc)))),
    )
}
