use crate::achievements::{
    Achievement, AchievementLanguage, AchievementPresention, PresentationType, ProgrammingLanguage,
    SerializableAchievement, Sprint,
};
use chrono::{DateTime, Local};

use csv;

pub fn read_achievements_from_gui() -> Result<Vec<Achievement>, csv::Error> {
    let mut rdr = csv::Reader::from_path("achievements.csv")?;
    let mut serialized_achievements = Vec::new();
    for result in rdr.deserialize() {
        let record: SerializableAchievement = result?;
        serialized_achievements.push(record.clone());
    }
    let mut achievements = Vec::new();
    for serialized_achievement in serialized_achievements {
        achievements.push(Achievement {
            id: serialized_achievement.id,
            link: serialized_achievement.link,
            title: serialized_achievement.title,
            deadline: if serialized_achievement.deadline == None {
                None
            } else {
                Some(
                    DateTime::parse_from_str(
                        &format!("{} 0:0:0 +0200", serialized_achievement.deadline.unwrap()),
                        "%b %d, %Y %H:%M:%S %z",
                    )
                    .unwrap()
                    .with_timezone(&Local),
                )
            },
            done: serialized_achievement.done,
            present_soon: serialized_achievement.present_soon,
            grade: serialized_achievement.grade,
            presenting_type: serialized_achievement.presenting_type,
            programming_language: serialized_achievement.programming_language,
            sprint: serialized_achievement.sprint,
            comment: serialized_achievement.comment,
        });
    }
    Ok(achievements)
}

pub fn read_achievements_from_google_sheets() -> Result<Vec<Achievement>, csv::Error> {
    let mut rdr = csv::Reader::from_path("achievements.csv")?;
    let mut achievements = Vec::new();

    for result in rdr.records() {
        let result = result.unwrap();
        let id = result[0].to_string();
        let link = result[1].to_string();
        let title = result[2].to_string();
        let deadline = match &result[3] {
            "" => None,
            _ => Some(
                DateTime::parse_from_str(
                    &format!("{} 0:0:0 +0200", &result[3]),
                    "%b %d, %Y %H:%M:%S %z",
                )
                .unwrap()
                .with_timezone(&Local),
            ),
        };
        let done = &result[4] == "TRUE";
        let present_soon = &result[5] == "TRUE";
        let grade = result[6].parse().expect("Grade is not a number!");
        let presenting_type = match &result[7] {
            "Lab" => AchievementPresention::Single(PresentationType::Lab),
            "Studium" => AchievementPresention::Single(PresentationType::Studium),
            "Special" => AchievementPresention::Single(PresentationType::Special),
            "Lab, Studium" => AchievementPresention::Either {
                first: PresentationType::Lab,
                second: PresentationType::Studium,
            },
            "Report" => AchievementPresention::Single(PresentationType::Report),
            _ => panic!("Unknown presentation type"),
        };
        let programming_language = match &result[8] {
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
            _ => panic!("Unknown programming language"),
        };
        let sprint = match &result[9] {
            "Sprint 1" => Sprint::Sprint1,
            "Sprint 2" => Sprint::Sprint2,
            "Sprint 3" => Sprint::Sprint3,
            "Sprint 4" => Sprint::Sprint4,
            "Project" => Sprint::Project,
            "Projekt" => Sprint::Project,
            "IDK" => Sprint::Unclear,
            v => panic!("Unknown sprint {}", v),
        };
        let comment = match &result[10] {
            "" => None,
            _ => Some(result[10].to_string()),
        };

        achievements.push(Achievement {
            id,
            link,
            title,
            deadline,
            done,
            present_soon,
            grade,
            presenting_type,
            programming_language,
            sprint,
            comment,
        });
    }

    Ok(achievements)
}
