use eframe::CreationContext;

use crate::achievement_csv;
use crate::achievements::*;
use crate::progress_tracker::ProgressTracker;

use std::fmt::Display;

use eframe::egui;

pub struct Settings {
    pub line_height: f32,
    pub font_size: f32,
}

#[derive(Debug)]
pub enum Direction {
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
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Ascending => Direction::Descending,
            Direction::Descending => Direction::Ascending,
            Direction::Default => Direction::Ascending,
        }
    }
}

#[derive(PartialEq)]
pub enum Fieled {
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

pub struct Sort {
    pub id: Direction,
    pub title: Direction,
    pub deadline: Direction,
    pub done: Direction,
    pub present_soon: Direction,
    pub grade: Direction,
    pub presenting_type: Direction,
    pub programming_language: Direction,
    pub sprint: Direction,
    pub comment: Direction,
    pub fieled: Fieled,
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

pub struct Filters {
    pub id: Vec<String>,
    pub title: Vec<String>,
    pub deadline: Vec<Option<chrono::DateTime<chrono::Local>>>,
    pub done: Vec<bool>,
    pub present_soon: Vec<bool>,
    pub grade: Vec<i8>,
    pub presenting_type: Vec<AchievementPresention>,
    pub programming_language: Vec<AchievementLanguage>,
    pub sprint: Vec<Sprint>,
    pub comment: Vec<Option<String>>,
}

impl Filters {
    pub fn new() -> Self {
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

pub enum ActiveWindow {
    Achievements,
    ProgressTracker,
}

pub struct Application {
    pub settings: Settings,
    pub achievements: Vec<Achievement>,
    pub progress_tracker: ProgressTracker,
    pub sorting: Sort,
    pub filters: Filters,
    pub active_window: ActiveWindow,
}

impl Application {
    pub fn new(_cc: &CreationContext) -> Self {
        let achievements = match achievement_csv::read_achievements_from_gui() {
            Ok(achievements) => achievements,
            Err(_) => match achievement_csv::read_achievements_from_google_sheets() {
                Ok(achievements) => achievements,
                Err(_) => achievement_csv::read_defaults(),
            },
        };

        let progress_tracker = ProgressTracker::new(4, 5, &achievements);

        Self {
            settings: Settings {
                line_height: 35.0,
                font_size: 14.0,
            },
            achievements,
            progress_tracker,
            sorting: Sort::new(),
            filters: Filters::new(),
            active_window: ActiveWindow::Achievements,
        }
    }

    pub fn save_achievements(&self) -> Result<(), csv::Error> {
        let mut wtr = csv::Writer::from_path("achievements.csv")?;
        for achievement in self.achievements.clone() {
            let serializable_achievement = SerializableAchievement {
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

    pub fn clear_done(&mut self) {
        for achievement in self.achievements.iter_mut() {
            achievement.done = false;
        }
    }

    pub fn clear_present_soon(&mut self) {
        for achievement in self.achievements.iter_mut() {
            achievement.present_soon = false;
        }
    }

    pub fn sort_achievements(&mut self, fieled: Fieled) {
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

    pub fn filtered_achievements(&mut self) -> Vec<(usize, Achievement)> {
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
            .collect::<Vec<(usize, Achievement)>>();

        filtered_achievements
    }
}

impl eframe::App for Application {
    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        if let Err(e) = self.save_achievements() {
            eprintln!("Error saving achievements: {}", e);
        };
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Q) && i.modifiers.ctrl) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close)
        }

        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            if let Err(e) = self.save_achievements() {
                eprintln!("Error saving achievements: {}", e);
            };
        }

        match self.active_window {
            ActiveWindow::Achievements => self.achievements_ui(ctx),
            ActiveWindow::ProgressTracker => self.progress_tracker_ui(ctx),
        }
    }
}