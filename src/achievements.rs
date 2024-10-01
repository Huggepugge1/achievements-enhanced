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

impl std::fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgrammingLanguage::C => write!(f, "C"),
            ProgrammingLanguage::Java => write!(f, "Java"),
            ProgrammingLanguage::Git => write!(f, "Git"),
            ProgrammingLanguage::Bash => write!(f, "Terminal"),
            ProgrammingLanguage::NoLanguage => write!(f, "Essä"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
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

impl std::fmt::Display for AchievementLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AchievementLanguage::Single(lang) => write!(f, "{}", lang),
            AchievementLanguage::Both { first, second } => write!(f, "{}&{}", first, second),
            AchievementLanguage::Either { first, second } => write!(f, "{}/{}", first, second),
        }
    }
}

impl AchievementLanguage {
    pub fn from_string(string: String) -> AchievementLanguage {
        match string.as_str() {
            "C" => AchievementLanguage::Single(ProgrammingLanguage::C),
            "Java" => AchievementLanguage::Single(ProgrammingLanguage::Java),
            "Terminal" => AchievementLanguage::Single(ProgrammingLanguage::Bash),
            "Git" => AchievementLanguage::Single(ProgrammingLanguage::Git),
            "Essä" => AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
            "Möte" => AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
            "C&Java" => AchievementLanguage::Both {
                first: ProgrammingLanguage::C,
                second: ProgrammingLanguage::Java,
            },
            "C/Java" => AchievementLanguage::Either {
                first: ProgrammingLanguage::C,
                second: ProgrammingLanguage::Java,
            },
            "NoLanguage" => AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
            e => panic!("Unknown programming language {e}"),
        }
    }
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

impl std::fmt::Display for PresentationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PresentationType::Lab => write!(f, "Lab"),
            PresentationType::Studium => write!(f, "Studium"),
            PresentationType::Special => write!(f, "Special"),
            PresentationType::Report => write!(f, "Report"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum AchievementPresention {
    Single(PresentationType),
    Either {
        first: PresentationType,
        second: PresentationType,
    },
}

impl std::fmt::Display for AchievementPresention {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AchievementPresention::Single(presentation) => write!(f, "{}", presentation),
            AchievementPresention::Either { first, second } => write!(f, "{}, {}", first, second),
        }
    }
}

impl AchievementPresention {
    pub fn from_string(string: String) -> AchievementPresention {
        match string.as_str() {
            "Lab" => AchievementPresention::Single(PresentationType::Lab),
            "Studium" => AchievementPresention::Single(PresentationType::Studium),
            "Special" => AchievementPresention::Single(PresentationType::Special),
            "Lab, Studium" => AchievementPresention::Either {
                first: PresentationType::Lab,
                second: PresentationType::Studium,
            },
            "Report" => AchievementPresention::Single(PresentationType::Report),
            _ => panic!("Unknown presentation type"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Achievement {
    pub id: String,
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
    pub title: String,
    pub deadline: Option<String>,
    pub done: bool,
    pub present_soon: bool,
    pub grade: i8,
    pub presenting_type: String,
    pub programming_language: String,
    pub sprint: Sprint,
    pub comment: Option<String>,
}
