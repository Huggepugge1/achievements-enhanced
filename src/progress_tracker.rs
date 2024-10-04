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
    pub optimal: u8,
    pub minimum: u8,
    pub target: u8,
    pub current_minimum: u8,
}

impl Lab {
    pub fn new(date: String) -> Self {
        let date =
            DateTime::parse_from_str(&format!("{date} 0:0:0 +0000"), "%b %d, %Y %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Local);

        let lab = Lab {
            date,
            optimal: 0,
            minimum: 0,
            target: 0,
            current_minimum: 0,
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
    pub optimal: u8,
    pub minimum: u8,
    pub target: u8,
    pub current_minimum: u8,
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
            optimal: 0,
            minimum: 0,
            target: 0,
            current_minimum: 0,
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

        self.optimal = match self.mode {
            ProgressTrackerMode::Left => max_achievements,
            ProgressTrackerMode::Done => 0,
        };

        self.minimum = self.optimal;
        self.target = self.optimal;
        self.current_minimum = self.optimal;

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
                .collect::<Vec<&Achievement>>()
                .len() as u8;

            let achievements_left = self
                .achievements
                .iter()
                .filter(|achievement| {
                    achievement.presenting_type
                        == AchievementPresention::Single(PresentationType::Lab)
                })
                .filter(|achievement| achievement.sprint.to_date() <= lab.date)
                .filter(|achievement| achievement.grade <= self.target_grade)
                .filter(|achievement| !achievement.done)
                .collect::<Vec<&Achievement>>()
                .len() as u8;

            match self.mode {
                ProgressTrackerMode::Left => {
                    self.optimal -= u8::min(
                        self.max_per_lab,
                        filtered_achievements - (max_achievements - self.optimal),
                    );
                    self.minimum -= u8::min(
                        self.max_per_lab,
                        u8::min(
                            filtered_achievements - (max_achievements - self.minimum),
                            u8::div_ceil(
                                max_achievements - (max_achievements - self.minimum),
                                (self.labs.len() - i) as u8,
                            ),
                        ),
                    );
                    if Local::now() > lab.date {
                        self.target -= u8::min(
                            4,
                            u8::min(
                                filtered_achievements - (max_achievements - self.target),
                                (filtered_achievements - achievements_left)
                                    - (max_achievements - self.target),
                            ),
                        );
                        self.current_minimum -= u8::min(
                            4,
                            u8::min(
                                filtered_achievements - (max_achievements - self.current_minimum),
                                (filtered_achievements - achievements_left)
                                    - (max_achievements - self.current_minimum),
                            ),
                        );
                    } else {
                        self.target -= u8::min(
                            self.max_per_lab,
                            filtered_achievements - (max_achievements - self.target),
                        );
                        self.current_minimum -= u8::min(
                            self.max_per_lab,
                            u8::min(
                                filtered_achievements - (max_achievements - self.current_minimum),
                                u8::div_ceil(
                                    max_achievements - (max_achievements - self.current_minimum),
                                    (self.labs.len() - i) as u8,
                                ),
                            ),
                        );
                    }
                }
                ProgressTrackerMode::Done => {
                    self.optimal += u8::min(self.max_per_lab, filtered_achievements - self.optimal);
                    self.minimum += u8::min(
                        self.max_per_lab,
                        u8::min(
                            filtered_achievements - self.minimum,
                            u8::div_ceil(
                                max_achievements - self.minimum,
                                (self.labs.len() - i) as u8,
                            ),
                        ),
                    );
                    if Local::now() > lab.date {
                        self.target += u8::min(
                            4,
                            u8::min(
                                filtered_achievements - self.target,
                                (filtered_achievements - achievements_left) - self.target,
                            ),
                        );
                        self.current_minimum += u8::min(
                            4,
                            u8::min(
                                filtered_achievements - self.current_minimum,
                                (filtered_achievements - achievements_left) - self.current_minimum,
                            ),
                        );
                    } else {
                        self.target +=
                            u8::min(self.max_per_lab, filtered_achievements - self.target);
                        self.current_minimum += u8::min(
                            self.max_per_lab,
                            u8::min(
                                filtered_achievements - self.current_minimum,
                                u8::div_ceil(
                                    max_achievements - self.current_minimum,
                                    (self.labs.len() - i) as u8,
                                ),
                            ),
                        );
                    }
                }
            }

            self.labs[i].optimal = self.optimal;
            self.labs[i].minimum = self.minimum;
            self.labs[i].target = self.target;
            self.labs[i].current_minimum = self.current_minimum;
        }
    }
}
