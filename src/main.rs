mod achievements;

use eframe::{egui, CreationContext, NativeOptions};
use serde;
use serde_json;

use std::fs::File;
use std::io::prelude::*;

use std::fmt::Display;

struct Settings {
    line_height: f32,
    font_size: f32,
}

#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
    Default,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Ascending => write!(f, "ascending"),
            Direction::Descending => write!(f, "descending"),
            Direction::Default => write!(f, "default"),
        }
    }
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::Ascending => Direction::Descending,
            Direction::Descending => Direction::Ascending,
            Direction::Default => Direction::Ascending,
        }
    }
}

#[derive(PartialEq)]
enum Fieled {
    ID,
    Title,
    Deadline,
    Done,
    PresentSoon,
    Grade,
    PresentingType,
    ProgrammingLanguage,
    Sprint,
    Comment,
}

struct Sort {
    id: Direction,
    title: Direction,
    deadline: Direction,
    done: Direction,
    present_soon: Direction,
    grade: Direction,
    presenting_type: Direction,
    programming_language: Direction,
    sprint: Direction,
    comment: Direction,
    fieled: Fieled,
}

impl Sort {
    fn new() -> Self {
        Self {
            id: Direction::Default,
            title: Direction::Default,
            deadline: Direction::Default,
            done: Direction::Default,
            present_soon: Direction::Default,
            grade: Direction::Default,
            presenting_type: Direction::Default,
            programming_language: Direction::Default,
            sprint: Direction::Default,
            comment: Direction::Default,
            fieled: Fieled::ID,
        }
    }

    fn from(fieled: Fieled) -> Self {
        let mut sort = Self::new();
        match fieled {
            Fieled::ID => sort.id = Direction::Ascending,
            Fieled::Title => sort.title = Direction::Ascending,
            Fieled::Deadline => sort.deadline = Direction::Ascending,
            Fieled::Done => sort.done = Direction::Ascending,
            Fieled::PresentSoon => sort.present_soon = Direction::Ascending,
            Fieled::Grade => sort.grade = Direction::Ascending,
            Fieled::PresentingType => sort.presenting_type = Direction::Ascending,
            Fieled::ProgrammingLanguage => sort.programming_language = Direction::Ascending,
            Fieled::Sprint => sort.sprint = Direction::Ascending,
            Fieled::Comment => sort.comment = Direction::Ascending,
        }
        sort.fieled = fieled;
        sort
    }

    fn reverse(&mut self, fieled: Fieled) {
        if self.fieled == fieled {
            match self.fieled {
                Fieled::ID => self.id = self.id.reverse(),
                Fieled::Title => self.title = self.title.reverse(),
                Fieled::Deadline => self.deadline = self.deadline.reverse(),
                Fieled::Done => self.done = self.done.reverse(),
                Fieled::PresentSoon => self.present_soon = self.present_soon.reverse(),
                Fieled::Grade => self.grade = self.grade.reverse(),
                Fieled::PresentingType => self.presenting_type = self.presenting_type.reverse(),
                Fieled::ProgrammingLanguage => {
                    self.programming_language = self.programming_language.reverse()
                }
                Fieled::Sprint => self.sprint = self.sprint.reverse(),
                Fieled::Comment => self.comment = self.comment.reverse(),
            }
            return;
        }
        *self = Self::from(fieled);
    }
}

struct Filters {
    id: Vec<String>,
    title: Vec<String>,
    deadline: Vec<Option<chrono::DateTime<chrono::Local>>>,
    done: Vec<bool>,
    present_soon: Vec<bool>,
    grade: Vec<i8>,
    presenting_type: Vec<achievements::AchievementPresention>,
    programming_language: Vec<achievements::AchievementLanguage>,
    sprint: Vec<achievements::Sprint>,
    comment: Vec<Option<String>>,
}

impl Filters {
    fn new() -> Self {
        Self {
            id: Vec::new(),
            title: Vec::new(),
            deadline: Vec::new(),
            done: Vec::new(),
            present_soon: Vec::new(),
            grade: Vec::new(),
            presenting_type: Vec::new(),
            programming_language: Vec::new(),
            sprint: Vec::new(),
            comment: Vec::new(),
        }
    }
}

struct Application {
    settings: Settings,
    achievements: Vec<achievements::Achievement>,
    sorting: Sort,
    filters: Filters,
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
            sorting: Sort::new(),
            filters: Filters::new(),
        }
    }

    fn save_achievements(&self) {
        let mut done = Vec::new();
        let mut present_soon = Vec::new();
        let mut achievements = self.achievements.clone();
        achievements.sort_by(|a, b| a.id.cmp(&b.id));
        for achievement in achievements.iter() {
            done.push(achievement.done);
            present_soon.push(achievement.present_soon);
        }
        let json_save = JsonSave { done, present_soon };

        let json = serde_json::to_string(&json_save).unwrap();
        let mut file = File::create("achievements.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    fn clear_done(&mut self) {
        for achievement in self.achievements.iter_mut() {
            achievement.done = false;
        }
    }

    fn clear_present_soon(&mut self) {
        for achievement in self.achievements.iter_mut() {
            achievement.present_soon = false;
        }
    }

    fn sort_achievements(&mut self, fieled: Fieled) {
        self.sorting.reverse(fieled);
        match self.sorting.fieled {
            Fieled::ID => match self.sorting.id {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.id.cmp(&b.id)),
                Direction::Descending => self.achievements.sort_by(|a, b| b.id.cmp(&a.id)),
                Direction::Default => self.achievements.sort_by(|a, b| a.id.cmp(&b.id)),
            },
            Fieled::Title => match self.sorting.title {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.title.cmp(&b.title)),
                Direction::Descending => self.achievements.sort_by(|a, b| b.title.cmp(&a.title)),
                Direction::Default => self.achievements.sort_by(|a, b| a.title.cmp(&b.title)),
            },
            Fieled::Deadline => match self.sorting.deadline {
                Direction::Ascending => self
                    .achievements
                    .sort_by(|a, b| a.deadline.cmp(&b.deadline)),
                Direction::Descending => self
                    .achievements
                    .sort_by(|a, b| b.deadline.cmp(&a.deadline)),
                Direction::Default => self
                    .achievements
                    .sort_by(|a, b| a.deadline.cmp(&b.deadline)),
            },
            Fieled::Done => match self.sorting.done {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.done.cmp(&b.done)),
                Direction::Descending => self.achievements.sort_by(|a, b| b.done.cmp(&a.done)),
                Direction::Default => self.achievements.sort_by(|a, b| a.done.cmp(&b.done)),
            },
            Fieled::PresentSoon => match self.sorting.present_soon {
                Direction::Ascending => self
                    .achievements
                    .sort_by(|a, b| a.present_soon.cmp(&b.present_soon)),
                Direction::Descending => self
                    .achievements
                    .sort_by(|a, b| b.present_soon.cmp(&a.present_soon)),
                Direction::Default => self
                    .achievements
                    .sort_by(|a, b| a.present_soon.cmp(&b.present_soon)),
            },
            Fieled::Grade => match self.sorting.grade {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.grade.cmp(&b.grade)),
                Direction::Descending => self.achievements.sort_by(|a, b| b.grade.cmp(&a.grade)),
                Direction::Default => self.achievements.sort_by(|a, b| a.grade.cmp(&b.grade)),
            },
            Fieled::PresentingType => match self.sorting.presenting_type {
                Direction::Ascending => self
                    .achievements
                    .sort_by(|a, b| a.presenting_type.cmp(&b.presenting_type)),
                Direction::Descending => self
                    .achievements
                    .sort_by(|a, b| b.presenting_type.cmp(&a.presenting_type)),
                Direction::Default => self
                    .achievements
                    .sort_by(|a, b| a.presenting_type.cmp(&b.presenting_type)),
            },
            Fieled::ProgrammingLanguage => match self.sorting.programming_language {
                Direction::Ascending => self
                    .achievements
                    .sort_by(|a, b| a.programming_language.cmp(&b.programming_language)),
                Direction::Descending => self
                    .achievements
                    .sort_by(|a, b| b.programming_language.cmp(&a.programming_language)),
                Direction::Default => self
                    .achievements
                    .sort_by(|a, b| a.programming_language.cmp(&b.programming_language)),
            },
            Fieled::Sprint => match self.sorting.sprint {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.sprint.cmp(&b.sprint)),
                Direction::Descending => self.achievements.sort_by(|a, b| b.sprint.cmp(&a.sprint)),
                Direction::Default => self.achievements.sort_by(|a, b| a.sprint.cmp(&b.sprint)),
            },
            Fieled::Comment => match self.sorting.comment {
                Direction::Ascending => self.achievements.sort_by(|a, b| a.comment.cmp(&b.comment)),
                Direction::Descending => {
                    self.achievements.sort_by(|a, b| b.comment.cmp(&a.comment))
                }
                Direction::Default => self.achievements.sort_by(|a, b| a.comment.cmp(&b.comment)),
            },
        }
    }

    fn filtered_achievements(&mut self) -> Vec<(usize, achievements::Achievement)> {
        let filtered_achievements = self
            .achievements
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, x)| !self.filters.id.contains(&x.id))
            .filter(|(_, x)| !self.filters.title.contains(&x.title))
            .filter(|(_, x)| !self.filters.deadline.contains(&x.deadline))
            .filter(|(_, x)| !self.filters.done.contains(&x.done))
            .filter(|(_, x)| !self.filters.present_soon.contains(&x.present_soon))
            .filter(|(_, x)| !self.filters.grade.contains(&x.grade))
            .filter(|(_, x)| !self.filters.presenting_type.contains(&x.presenting_type))
            .filter(|(_, x)| {
                !self
                    .filters
                    .programming_language
                    .contains(&x.programming_language)
            })
            .filter(|(_, x)| !self.filters.sprint.contains(&x.sprint))
            .filter(|(_, x)| !self.filters.comment.contains(&x.comment))
            .collect::<Vec<(usize, achievements::Achievement)>>();

        filtered_achievements
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonSave {
    done: Vec<bool>,
    present_soon: Vec<bool>,
}

impl eframe::App for Application {
    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        self.save_achievements();
    }
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            self.save_achievements();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
        }

        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save_achievements();
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

                let max_width = ui.available_width();
                let max_height = ui.available_height();
                egui::ScrollArea::both().scroll_bar_rect(egui::Rect::from_min_max(egui::Pos2::new(max_width - 5.0, 0.0), egui::Pos2::new(max_width, max_height))).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        let achievements = self.filtered_achievements()
                            .into_iter()
                            .map(|(_, achievement)| achievement)
                            .collect::<Vec<achievements::Achievement>>();

                        egui::Grid::new(0)
                            .spacing(egui::vec2(50.0, 15.0))
                            .min_row_height(self.settings.line_height)
                            .num_columns(11)
                            .with_row_color(move |row_index, _style| {
                                if row_index > 0
                                    && row_index < achievements.len() + 1
                                    && achievements[row_index - 1].done
                                {
                                    let green = egui::Color32::from_rgba_unmultiplied(0, 255, 0, 50);
                                    Some(green)
                                } else if row_index > 0
                                    && row_index < achievements.len() + 1
                                    && achievements[row_index - 1].present_soon
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
                                let id = ui.heading("ID").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.id.reverse()));
                                });
                                let title = ui.heading("Title").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.title.reverse()));
                                });
                                let deadline = ui.heading("Deadline").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.deadline.reverse()));
                                });
                                let done = ui.heading("Done").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.done.reverse()));
                                });
                                let present_soon = ui.heading("Present Soon").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.present_soon.reverse()));
                                });
                                let grade = ui.heading("Grade").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.grade.reverse()));
                                });
                                let presenting_type = ui.heading("Presenting Type").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.presenting_type.reverse()));
                                });
                                let programming_language = ui.heading("Programming Language").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.programming_language.reverse()));
                                });
                                let sprint = ui.heading("Sprint").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.sprint.reverse()));
                                });
                                let comment = ui.heading("Comment").on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_ui(|ui| {
                                    ui.label(format!("Click to sort {}", self.sorting.comment.reverse()));
                                });
                                if id.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::ID);
                                }
                                if title.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Title);
                                }
                                if deadline.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Deadline);
                                }
                                if done.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Done);
                                }
                                if present_soon.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::PresentSoon);
                                }
                                if grade.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Grade);
                                }
                                if presenting_type.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::PresentingType);
                                }
                                if programming_language.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::ProgrammingLanguage);
                                }
                                if sprint.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Sprint);
                                }
                                if comment.clicked_by(egui::PointerButton::Primary) {
                                    self.sort_achievements(Fieled::Comment);
                                }
                                ui.end_row();

                                for (i, achievement) in self.filtered_achievements() {
                                    let id = ui.add(egui::Hyperlink::from_label_and_url(
                                        egui::RichText::new(format!("{}", achievement.id))
                                            .font(egui::FontId::new(
                                                self.settings.font_size,
                                                egui::FontFamily::Monospace,
                                            ))
                                            .text_style(egui::TextStyle::Monospace),
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.link),
                                    ))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let title = ui.add(egui::Hyperlink::from_label_and_url(
                                        egui::RichText::new(format!("{}", achievement.title))
                                            .font(egui::FontId::new(
                                                self.settings.font_size,
                                                egui::FontFamily::Monospace,
                                            ))
                                            .color(egui::Color32::LIGHT_GRAY)
                                            .text_style(egui::TextStyle::Monospace),
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.link),
                                    ))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let deadline = match achievement.deadline {
                                        Some(deadline) => {
                                            ui.label(deadline.format("%a %b %d, %Y").to_string())
                                        }
                                        None => {
                                            ui.label("")
                                        }
                                    }
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let done = ui.centered_and_justified(|ui| {
                                        ui.checkbox(&mut self.achievements[i].done, "")
                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                            .on_hover_text("Right click to filter out")
                                    });
                                    let present_soon = ui.centered_and_justified(|ui| {
                                        ui.checkbox(&mut self.achievements[i].present_soon, "")
                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                            .on_hover_text("Right click to filter out")
                                    });
                                    let grade = ui.label(achievement.grade.to_string())
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let presenting_type = match &achievement.presenting_type {
                                        achievements::AchievementPresention::Single(presenting_type) => {
                                            ui.label(format!("{:?}", presenting_type))
                                        },
                                        achievements::AchievementPresention::Either(presenting_type1, presenting_type2) => {
                                            ui.label(format!("{:?} OR {:?}", presenting_type1, presenting_type2))
                                        },
                                    }
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let programming_language = match &achievement.programming_language {
                                        achievements::AchievementLanguage::Single(programming_language) => {
                                            if programming_language == &achievements::ProgrammingLanguage::NoLanguage {
                                                ui.label("No Specific Language")
                                            } else {
                                                ui.label(format!("{:?}", programming_language))
                                            }
                                        },
                                        achievements::AchievementLanguage::Both(programming_language1, programming_language2) => {
                                            ui.label(format!("{:?} AND {:?}", programming_language1, programming_language2))
                                        },
                                        achievements::AchievementLanguage::Either(programming_language1, programming_language2) => {
                                            ui.label(format!("{:?} OR {:?}", programming_language1, programming_language2))
                                        },
                                    }
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let sprint = ui.label(format!("{:?}", achievement.sprint))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    let comment = match &achievement.comment {
                                        Some(comment) => {
                                            ui.label(comment)
                                        }
                                        None => {
                                            ui.label("")
                                        }
                                    }
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .on_hover_text("Right click to filter out");

                                    if id.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.id.push(achievement.id.clone());
                                    }
                                    if title.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.title.push(achievement.title.clone());
                                    }
                                    if deadline.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.deadline.push(achievement.deadline);
                                    }
                                    if done.inner.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.done.push(achievement.done);
                                    }
                                    if present_soon.inner.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.present_soon.push(achievement.present_soon);
                                    }
                                    if grade.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.grade.push(achievement.grade);
                                    }
                                    if presenting_type.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.presenting_type.push(achievement.presenting_type.clone());
                                    }
                                    if programming_language.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.programming_language.push(achievement.programming_language.clone());
                                    }
                                    if sprint.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.sprint.push(achievement.sprint.clone());
                                    }
                                    if comment.clicked_by(egui::PointerButton::Secondary) {
                                        self.filters.comment.push(achievement.comment.clone());
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });
                let save_button = ui.button("Save");
                if save_button.clicked() {
                    self.save_achievements();
                }
                let clear_done_button = ui.button("Clear Done");
                if clear_done_button.clicked() {
                    self.clear_done();
                }
                let clear_present_soon_button = ui.button("Clear Present Soon");
                if clear_present_soon_button.clicked() {
                    self.clear_present_soon();
                }
                let clear_filters_button = ui.button("Clear Filters");
                if clear_filters_button.clicked() {
                    self.filters = Filters::new();
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
