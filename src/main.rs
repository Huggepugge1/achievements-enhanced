mod achievements;

use eframe::{egui, CreationContext, NativeOptions};
use serde;
use serde_json;

use std::fs::File;
use std::io::prelude::*;

struct Settings {
    line_height: f32,
    font_size: f32,
}

struct Application {
    settings: Settings,
    achievements: Vec<achievements::Achievement>,
}

impl Application {
    fn new(_cc: &CreationContext) -> Self {
        let achievements = achievements::get_all_achievements();

        Self {
            settings: Settings {
                line_height: 35.0,
                font_size: 14.0,
            },
            achievements,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonSave {
    done: Vec<bool>,
    present_soon: Vec<bool>,
}

fn save(achievements: Vec<achievements::Achievement>) {
    let mut done = Vec::new();
    let mut present_soon = Vec::new();
    for achievement in achievements.iter() {
        done.push(achievement.done);
        present_soon.push(achievement.present_soon);
    }
    let json_save = JsonSave { done, present_soon };

    let json = serde_json::to_string(&json_save).unwrap();
    let mut file = File::create("achievements.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn clear_done(achievements: &mut Vec<achievements::Achievement>) {
    for achievement in achievements.iter_mut() {
        achievement.done = false;
    }
}

fn clear_present_soon(achievements: &mut Vec<achievements::Achievement>) {
    for achievement in achievements.iter_mut() {
        achievement.present_soon = false;
    }
}

impl eframe::App for Application {
    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        save(self.achievements.clone());
    }
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            save(self.achievements.clone());
            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
        }

        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            save(self.achievements.clone());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ctx.style_mut(|ctx| {
                    ctx.wrap_mode = Some(egui::TextWrapMode::Extend);
                    ctx.override_font_id = Some(egui::FontId::new(
                        self.settings.font_size,
                        egui::FontFamily::Monospace,
                    ));
                });

                let heap_achievements = Box::new(self.achievements.clone());
                let max_width = ui.available_width();
                let max_height = ui.available_height();
                egui::ScrollArea::both().scroll_bar_rect(egui::Rect::from_min_max(egui::Pos2::new(max_width - 5.0, 0.0), egui::Pos2::new(max_width, max_height))).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        egui::Grid::new(0)
                            .spacing(egui::vec2(50.0, 15.0))
                            .min_row_height(self.settings.line_height)
                            .num_columns(11)
                            .with_row_color(move |row_index, _style| {
                                if row_index > 0
                                    && row_index < heap_achievements.len() + 1
                                    && heap_achievements[row_index - 1].done
                                {
                                    let green = egui::Color32::from_rgba_unmultiplied(0, 255, 0, 50);
                                    Some(green)
                                } else if row_index > 0
                                    && row_index < heap_achievements.len() + 1
                                    && heap_achievements[row_index - 1].present_soon
                                {
                                    let yellow = egui::Color32::from_rgba_unmultiplied(255, 255, 0, 25);
                                    Some(yellow)
                                } else if row_index % 2 == 0 {
                                    Some(egui::Color32::from_black_alpha(64))
                                } else {
                                    None
                                }
                            })
                            .show(ui, |ui| {
                                ui.heading("ID");
                                ui.heading("Title");
                                ui.heading("Deadline");
                                ui.heading("Done");
                                ui.heading("Present Soon");
                                ui.heading("Grade");
                                ui.heading("Presenting type");
                                ui.heading("Language");
                                ui.heading("Sprint");
                                ui.heading("Comment");
                                ui.end_row();

                                for (i, achievement) in self.achievements.clone().iter().enumerate() {
                                    ui.add(egui::Hyperlink::from_label_and_url(
                                        egui::RichText::new(format!("{}", achievement.id))
                                            .font(egui::FontId::new(
                                                self.settings.font_size,
                                                egui::FontFamily::Monospace,
                                            ))
                                            .text_style(egui::TextStyle::Monospace),
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.link),
                                    ));
                                    ui.add(egui::Hyperlink::from_label_and_url(
                                        egui::RichText::new(format!("{}", achievement.title))
                                            .font(egui::FontId::new(
                                                self.settings.font_size,
                                                egui::FontFamily::Monospace,
                                            ))
                                            .color(egui::Color32::LIGHT_GRAY)
                                            .text_style(egui::TextStyle::Monospace),
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.link),
                                    ));
                                    match achievement.deadline {
                                        Some(deadline) => {
                                            ui.label(deadline.format("%b %d, %Y").to_string());
                                        }
                                        None => {
                                            ui.label("");
                                        }
                                    }
                                    ui.centered_and_justified(|ui| {
                                        ui.checkbox(&mut self.achievements[i].done, "");
                                    });
                                    ui.centered_and_justified(|ui| {
                                        ui.checkbox(&mut self.achievements[i].present_soon, "");
                                    });
                                    ui.label(achievement.grade.to_string());
                                    match &achievement.presenting_type {
                                        achievements::AchievementPresention::Single(presenting_type) => {
                                            ui.label(format!("{:?}", presenting_type));
                                        },
                                        achievements::AchievementPresention::Either(presenting_type1, presenting_type2) => {
                                            ui.label(format!("{:?} OR {:?}", presenting_type1, presenting_type2));
                                        },
                                    }
                                    match &achievement.programming_language {
                                        achievements::AchievementLanguage::Single(programming_language) => {
                                            if programming_language == &achievements::ProgrammingLanguage::NoLanguage {
                                                ui.label("No Specific Language");
                                            } else {
                                                ui.label(format!("{:?}", programming_language));
                                            }
                                        },
                                        achievements::AchievementLanguage::Both(programming_language1, programming_language2) => {
                                            ui.label(format!("{:?} AND {:?}", programming_language1, programming_language2));
                                        },
                                        achievements::AchievementLanguage::Either(programming_language1, programming_language2) => {
                                            ui.label(format!("{:?} OR {:?}", programming_language1, programming_language2));
                                        },
                                    }
                                    ui.label(format!("{:?}", achievement.sprint));
                                    match &achievement.comment {
                                        Some(comment) => {
                                            ui.label(comment);
                                        }
                                        None => {
                                            ui.label("");
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });
                let save_button = ui.button("Save");
                if save_button.clicked() {
                    save(self.achievements.clone());
                }
                let clear_done_button = ui.button("Clear Done");
                if clear_done_button.clicked() {
                    clear_done(&mut self.achievements);
                }
                let clear_present_soon_button = ui.button("Clear Present Soon");
                if clear_present_soon_button.clicked() {
                    clear_present_soon(&mut self.achievements);
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    let native_options = NativeOptions::default();

    eframe::run_native(
        "Achievements Enhanced",
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}
