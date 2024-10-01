use crate::achievements::{Achievement, AchievementPresention, PresentationType};

use chrono::{DateTime, Local};

const LABS: [&str; 32] = [
    "Sep 2, 2024",
    "Sep 4, 2024",
    "Sep 5, 2024",
    "Sep 9, 2024",
    "Sep 11, 2024",
    "Sep 12, 2024",
    "Sep 16, 2024",
    "Sep 19, 2024",
    "Sep 23, 2024",
    "Sep 26, 2024",
    "Oct 1, 2024",
    "Oct 3, 2024",
    "Oct 7, 2024",
    "Oct 10, 2024",
    "Oct 14, 2024",
    "Oct 17, 2024",
    "Oct 21, 2024",
    "Oct 25, 2024",
    "Nov 1, 2024",
    "Nov 4, 2024",
    "Nov 6, 2024",
    "Nov 11, 2024",
    "Nov 14, 2024",
    "Nov 19, 2024",
    "Nov 21, 2024",
    "Nov 25, 2024",
    "Nov 28, 2024",
    "Dec 2, 2024",
    "Dec 5, 2024",
    "Dec 9, 2024",
    "Dec 12, 2024",
    "Dec 16, 2024",
];

#[derive(Clone, Debug)]
pub struct Lab {
    pub date: DateTime<Local>,
    pub done: bool,
    pub perfect_student: u8,
    pub target_student: u8,
    pub minimum_student: u8,
}

impl Lab {
    pub fn new(date: String) -> Self {
        let date =
            DateTime::parse_from_str(&format!("{date} 0:0:0 +0000"), "%b %d, %Y %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Local);

        let done = date < Local::now();

        let lab = Lab {
            date,
            done,
            perfect_student: 0,
            target_student: 0,
            minimum_student: 0,
        };

        lab
    }
}

pub enum ProgressTrackerMode {
    Left,
    Done,
}

impl ProgressTrackerMode {
    pub fn toggle(&self) -> Self {
        match self {
            ProgressTrackerMode::Left => ProgressTrackerMode::Done,
            ProgressTrackerMode::Done => ProgressTrackerMode::Left,
        }
    }
}

pub struct ProgressTracker {
    pub achievements: Vec<Achievement>,
    pub mode: ProgressTrackerMode,
    pub labs: Vec<Lab>,
    pub max_per_lab: u8,
    pub target_grade: i8,
    pub perfect_student: u8,
    pub target_student: u8,
    pub minimum_student: u8,
}

impl ProgressTracker {
    pub fn new(max_per_lab: u8, target_grade: i8, achievements: &Vec<Achievement>) -> Self {
        let mut labs = Vec::new();
        for lab in LABS.iter() {
            labs.push(Lab::new(lab.to_string()));
        }

        let mut progress_tracker = ProgressTracker {
            achievements: achievements.clone(),
            mode: ProgressTrackerMode::Left,
            labs,
            max_per_lab,
            target_grade,
            perfect_student: 0,
            target_student: 0,
            minimum_student: 0,
        };

        progress_tracker.update();
        progress_tracker
    }

    pub fn update(&mut self) {
        let max_achievements = self
            .achievements
            .iter()
            .filter(|achievement| {
                achievement.presenting_type == AchievementPresention::Single(PresentationType::Lab)
            })
            .filter(|achievement| achievement.grade <= self.target_grade)
            .collect::<Vec<&Achievement>>()
            .len() as u8;

        self.perfect_student = match self.mode {
            ProgressTrackerMode::Left => max_achievements,
            ProgressTrackerMode::Done => 0,
        };

        for (i, lab) in self.labs.clone().into_iter().enumerate() {
            let filtered_achievements = self
                .achievements
                .iter()
                .filter(|achievement| {
                    achievement.presenting_type
                        == AchievementPresention::Single(PresentationType::Lab)
                })
                .filter(|achievement| achievement.sprint.to_date() <= lab.date)
                .filter(|achievement| achievement.grade <= self.target_grade)
                .collect::<Vec<&Achievement>>();

            match self.mode {
                ProgressTrackerMode::Left => {
                    self.perfect_student -= u8::min(
                        self.max_per_lab,
                        filtered_achievements.len() as u8
                            - (max_achievements - self.perfect_student),
                    );
                }
                ProgressTrackerMode::Done => {
                    self.perfect_student += u8::min(
                        self.max_per_lab,
                        filtered_achievements.len() as u8 - self.perfect_student,
                    );
                }
            }

            self.labs[i].perfect_student = self.perfect_student;
        }
    }
}
