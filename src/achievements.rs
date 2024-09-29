use chrono::{DateTime, Local};

use serde;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum ProgrammingLanguage {
    C,
    Java,
    Git,
    Bash,
    NoLanguage,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum AchievementLanguage {
    Single(ProgrammingLanguage),
    Both {
        first: ProgrammingLanguage,
        second: ProgrammingLanguage,
    },
    Either {
        first: ProgrammingLanguage,
        second: ProgrammingLanguage,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Sprint {
    Sprint1,
    Sprint2,
    Sprint3,
    Sprint4,
    Project,
    Unclear,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum PresentationType {
    Lab,
    Studium,
    Special,
    Report,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum AchievementPresention {
    Single(PresentationType),
    Either {
        first: PresentationType,
        second: PresentationType,
    },
}

#[derive(Clone, Debug)]
pub struct Achievement {
    pub id: String,
    pub link: String,
    pub title: String,
    pub deadline: Option<DateTime<Local>>,
    pub done: bool,
    pub present_soon: bool,
    pub grade: i8,
    pub presenting_type: AchievementPresention,
    pub programming_language: AchievementLanguage,
    pub sprint: Sprint,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SerializableAchievement {
    pub id: String,
    pub link: String,
    pub title: String,
    pub deadline: Option<String>,
    pub done: bool,
    pub present_soon: bool,
    pub grade: i8,
    #[serde(flatten)]
    pub presenting_type: AchievementPresention,
    #[serde(flatten)]
    pub programming_language: AchievementLanguage,
    pub sprint: Sprint,
    pub comment: Option<String>,
}
