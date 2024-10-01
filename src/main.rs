mod achievement_csv;
mod achievements;

use eframe::{egui, CreationContext, NativeOptions};
use serde;

use csv;

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
        let achievements = match achievement_csv::read_achievements_from_gui() {
            Ok(achievements) => achievements,
            Err(_) => match achievement_csv::read_achievements_from_google_sheets() {
                Ok(achievements) => achievements,
                Err(_) => panic!("Could not read achievements from csv"),
            },
        };

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

    fn save_achievements(&self) -> Result<(), csv::Error> {
        let mut wtr = csv::Writer::from_path("achievements.csv")?;
        for achievement in self.achievements.clone() {
            let serializable_achievement = achievements::SerializableAchievement {
                id: achievement.id.clone(),
                link: format!(
                    "https://uppsala.instructure.com/courses/97453/pages/achievements#{}",
                    achievement.id[1..].to_string()
                ),
                title: achievement.title.clone(),
                deadline: achievement
                    .deadline
                    .map(|x| x.format("%b %d, %Y").to_string()),
                done: achievement.done,
                present_soon: achievement.present_soon,
                grade: achievement.grade,
                presenting_type: achievement.presenting_type.to_string(),
                programming_language: achievement.programming_language.to_string(),
                sprint: achievement.sprint.clone(),
                comment: achievement.comment.clone(),
            };
            wtr.serialize(serializable_achievement)?;
        }
        Ok(())
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
        if let Err(e) = self.save_achievements() {
            eprintln!("Error saving achievements: {}", e);
        };
    }
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));

        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
        }

        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            if let Err(e) = self.save_achievements() {
                eprintln!("Error saving achievements: {}", e);
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let file_button = ui.button(egui::RichText::new("File").font(egui::FontId::new(
                    self.settings.font_size * 1.5,
                    egui::FontFamily::Proportional,
                )));
                let file_popup_id = ui.make_persistent_id("File popup");
                egui::containers::popup::popup_below_widget(ui, file_popup_id, &file_button, egui::containers::popup::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                    let save_button = ui.label("Save").on_hover_cursor(egui::CursorIcon::PointingHand);
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
                });
                let edit_button = ui.button(egui::RichText::new("Edit").font(egui::FontId::new(
                    self.settings.font_size * 1.5,
                    egui::FontFamily::Proportional,
                )));
                let edit_popup_id = ui.make_persistent_id("Edit popup");
                egui::containers::popup::popup_below_widget(ui, edit_popup_id, &edit_button, egui::containers::popup::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                    let clear_done_button = ui.label("Clear Done").on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_done_button.hovered() {
                        clear_done_button.clone().highlight();
                    }
                    if clear_done_button.clicked() {
                        self.clear_done();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let clear_present_soon_button = ui.label("Clear Present Soon").on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_present_soon_button.hovered() {
                        clear_present_soon_button.clone().highlight();
                    }
                    if clear_present_soon_button.clicked() {
                        self.clear_present_soon();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                    let clear_filters_button = ui.label("Clear Filters").on_hover_cursor(egui::CursorIcon::PointingHand);
                    if clear_filters_button.hovered() {
                        clear_filters_button.clone().highlight();
                    }
                    if clear_filters_button.clicked() {
                        self.filters = Filters::new();
                        ui.memory_mut(|memory| {
                            memory.close_popup();
                        });
                    }
                });
                if file_button.clicked() {
                    ui.memory_mut(|memory| {
                        memory.open_popup(file_popup_id);
                    });
                } else if edit_button.clicked() {
                    ui.memory_mut(|memory| {
                        memory.open_popup(edit_popup_id);
                    });
                }

            });
            ui.horizontal_centered(|ui| {
                ctx.style_mut(|ctx| {
                    ctx.wrap_mode = Some(egui::TextWrapMode::Extend);
                    ctx.override_font_id = Some(egui::FontId::new(
                        self.settings.font_size,
                        egui::FontFamily::Proportional,
                    ));
                });

                egui::ScrollArea::both().stick_to_right(true).show(ui, |ui| {
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
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.id[1..].to_string()),
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
                                        format!("https://uppsala.instructure.com/courses/97453/pages/achievements#{}", achievement.id[1..].to_string()),
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
                                        achievements::AchievementPresention::Either { first, second } => {
                                            ui.label(format!("{:?} OR {:?}", first, second))
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
                                        achievements::AchievementLanguage::Both { first, second } => {
                                            ui.label(format!("{:?} AND {:?}", first, second))
                                        },
                                        achievements::AchievementLanguage::Either { first, second } => {
                                            ui.label(format!("{:?} OR {:?}", first, second))
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
                                    ui.allocate_space(egui::vec2(10.0, 0.0));

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
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();

    eframe::run_native(
        "Achievements Enhanced",
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}
