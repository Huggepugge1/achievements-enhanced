use chrono::{DateTime, Local};
use serde_json;
use std::fs::File;

#[derive(Clone, Debug, PartialEq)]
pub enum ProgrammingLanguage {
    C,
    Java,
    Git,
    Bash,
    NoLanguage,
}

#[derive(Clone, Debug)]
pub enum AchievementLanguage {
    Single(ProgrammingLanguage),
    Both(ProgrammingLanguage, ProgrammingLanguage),
    Either(ProgrammingLanguage, ProgrammingLanguage),
}

#[derive(Clone, Debug)]
pub enum Sprint {
    Sprint1,
    Sprint2,
    Sprint3,
    Sprint4,
    Project,
    Unclear,
}

#[derive(Clone, Debug)]
pub enum PresentationType {
    Lab,
    Studium,
    Special,
    Report,
}

#[derive(Clone, Debug)]
pub enum AchievementPresention {
    Single(PresentationType),
    Either(PresentationType, PresentationType),
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

pub fn get_all_achievements() -> Vec<Achievement> {
    let mut achievements = Vec::new();

    let ids = get_ids();
    let links = get_links();
    let titles = get_titles();
    let deadlines = get_deadlines();
    let dones = get_dones();
    let present_soons = get_present_soons();
    let grades = get_grades();
    let presenting_types = get_presenting_types();
    let programming_languages = get_programming_languages();
    let sprints = get_sprints();
    let comments = get_comments();

    for i in 0..65 {
        achievements.push(Achievement {
            id: ids[i].clone(),
            link: links[i].clone(),
            title: titles[i].clone(),
            deadline: deadlines[i],
            done: dones[i],
            present_soon: present_soons[i],
            grade: grades[i],
            presenting_type: presenting_types[i].clone(),
            programming_language: programming_languages[i].clone(),
            sprint: sprints[i].clone(),
            comment: comments[i].clone(),
        });
    }
    achievements
}

fn get_ids() -> Vec<String> {
    vec![
        "A1", "A2", "A3", "A8", "B4", "B5", "B6", "C7", "D9", "E10", "E11", "E12", "F13", "F14",
        "G15", "G16", "G17", "H18", "H19", "H20", "H21", "I23", "I24", "I25", "J26", "J27", "J28",
        "J29", "K30", "K31", "K32", "M36", "M37", "M38", "M39", "N40", "N41", "O42", "O43", "O44",
        "P45", "P46", "P47", "Q49", "Q50", "R52", "T55", "T56", "U57", "V58", "X59", "X61", "X62",
        "X63", "Y60", "Y64", "Y65", "Y66", "Y67", "Y68", "Y69", "Z91", "Z92", "Z93", "Z94",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn get_links() -> Vec<String> {
    vec![
        "org6ec1409",
        "orgd101405",
        "orgb1801a1",
        "orgfd99011",
        "org19417f2",
        "org12c58c2",
        "org288b1bd",
        "org293b2b8",
        "org4c1e1fd",
        "org993e2af",
        "orgf4c46cf",
        "orgefde6c7",
        "org4c3b0c6",
        "org39be067",
        "org24bce0b",
        "orgfff5d3e",
        "orgaf9af75",
        "orge50cfdb",
        "org4846134",
        "org0d66f54",
        "org11fd79e",
        "orgf95ae01",
        "orga237fb3",
        "org83ecd2d",
        "org0064590",
        "org8e960d7",
        "org3a71579",
        "org403928a",
        "org8892d9b",
        "org4a30e09",
        "orgd680e6d",
        "orgc7bcddb",
        "org431aa6b",
        "org58f21cf",
        "org8518dcf",
        "orgc6c64c1",
        "org929d964",
        "orga0bfcc8",
        "org5c555fd",
        "org946de2f",
        "orgc8680b4",
        "46        ",
        "orga5e5337",
        "orgf94df90",
        "orga147443",
        "orgdcce3f7",
        "orgd8800a7",
        "orgf1e44ec",
        "orgcd9a1a0",
        "org5878382",
        "orgcf45958",
        "org1db71ec",
        "orgfab8b7b",
        "org2b17aed",
        "org44d2c87",
        "orgcd50ba7",
        "orgfcced79",
        "orgaf2ee5e",
        "org9423f97",
        "org0dbe28c",
        "org288479c",
        "org14ee6b8",
        "orgf69508b",
        "orgca4784f",
        "org0040891",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn get_titles() -> Vec<String> {
    vec![
        "Procedurell abstraktion",
        "Objektorienterad abstraktion",
        "Informationsgömning",
        "Gränssnitt",
        "Arv och subtypspolymorfism",
        "Liskov’s Substitution Principle",
        "Genomskärande åtaganden och arv",
        "Planering och uppföljning",
        "Dokumentation",
        "Implementera genericitet genom void-pekare",
        "Parametrisk polymorfism och typsäkerhet",
        "Designa med parametrisk polymorfism",
        "Iteration vs. rekursion",
        "Svansrekursion",
        "Aliasering",
        "Namn-baserad inkapsling",
        "Nästlade och inre klasser",
        "Jämförelsemetoden",
        "Skillnaden mellan identitet och ekvivalens",
        "Värdeöverföring",
        "Abstrakta klasser, metoder och interface",
        "Undantagshantering",
        "Olika metoder för felhantering",
        "Egendefinierade undantag",
        "Allokering på stacken vs. på heapen",
        "Manuell minneshantering",
        "Manuell vs. automatisk minneshantering",
        "Jämför två metoder för automatisk skräpsamling",
        "Gränssnitt mellan moduler",
        "Coupling & cohesion",
        "Separation of concerns",
        "C:s array-notation och pekararitmetik",
        "Använda pekare för att skapa länkade strukturer",
        "Värdeöverföring via pekare",
        "Pekare till pekare",
        "Kompilering, länkning och interpretering",
        "Bindning",
        "Profilering och optimering 1/3",
        "Profilering och optimering 2/3",
        "Profilering och optimering 3/3",
        "Gör en informell kodgranskning under fas 1",
        "Gör en informell kodgranskning under fas 2",
        "Åtgärda defekter efter en kodgranskning",
        "Enhetstestning",
        "Mät och resonera kring testkvalitet",
        "Debuggning med gdb",
        "Använda en utvecklingsmiljö på ett effektivt sätt",
        "Continuous Integration",
        "Byggverktyget Make",
        "Grundläggande terminalkommandon",
        "Essä",
        "Running Group Meetings Responsibly",
        "Kommunikation 1:1",
        "Kommuniktion 1:M",
        "Presentera projektet vid ett seminarium",
        "Använd en namngiven utvecklingsprocess och reflektera över utkomsten",
        "Skriv konsekvent bra kod",
        "Tillämpa kodgranskning löpande",
        "Delta aktivt i ett programmeringsprojekt",
        "Redovisa en fungerande projektuppgift",
        "Tillämpa testning under projektet",
        "Inlupp 1",
        "Inlupp 2",
        "Inlupp 3",
        "Inlupp 4",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn get_deadlines() -> Vec<Option<DateTime<Local>>> {
    let mut deadlines = Vec::new();
    let string_deadlines = vec![
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "Dec 12, 2024 00:00:00 +0200",
        "",
        "Dec 16, 2024 00:00:00 +0200",
        "Sep 20, 2024 00:00:00 +0200",
        "Jan 14, 2025 00:00:00 +0200",
        "",
        "",
        "",
        "",
        "",
        "",
        "Oct 18, 2024 00:00:00 +0200",
        "Nov 15, 2024 00:00:00 +0200",
        "Dec 6, 2024 00:00:00 +0200",
        "Jan 16, 2025 00:00:00 +0200",
    ];
    for string_deadline in string_deadlines {
        if string_deadline == "" {
            deadlines.push(None);
        } else {
            deadlines.push(Some(
                DateTime::parse_from_str(string_deadline, "%b %d, %Y %H:%M:%S %z")
                    .unwrap()
                    .with_timezone(&Local),
            ));
        }
    }

    deadlines
}

fn get_dones() -> Vec<bool> {
    let file = File::open("achievements.json").expect("File could not be opened");
    let json: serde_json::Value = serde_json::from_reader(file).expect("File is not proper JSON");
    json.get("done")
        .expect("The JSON does not contain the right keys")
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_bool().unwrap())
        .collect::<Vec<bool>>()
}

fn get_present_soons() -> Vec<bool> {
    let file = File::open("achievements.json").expect("File could not be opened");
    let json: serde_json::Value = serde_json::from_reader(file).expect("File is not proper JSON");
    json.get("present_soon")
        .expect("The JSON does not contain the right keys")
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_bool().unwrap())
        .collect::<Vec<bool>>()
}

fn get_grades() -> Vec<i8> {
    vec![
        3, 3, 4, 4, 3, 4, 5, 3, 3, 3, 3, 4, 3, 4, 3, 3, 5, 3, 3, 4, 5, 3, 4, 4, 3, 3, 4, 5, 3, 4,
        5, 3, 3, 3, 4, 3, 4, 3, 4, 5, 3, 3, 4, 3, 4, 3, 3, 5, 3, 3, 4, 3, 5, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 4,
    ]
}

fn get_presenting_types() -> Vec<AchievementPresention> {
    vec![
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Studium),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Either(PresentationType::Lab, PresentationType::Studium),
        AchievementPresention::Either(PresentationType::Lab, PresentationType::Studium),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Either(PresentationType::Lab, PresentationType::Studium),
        AchievementPresention::Either(PresentationType::Lab, PresentationType::Studium),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Special),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Report),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
        AchievementPresention::Single(PresentationType::Lab),
    ]
}

fn get_programming_languages() -> Vec<AchievementLanguage> {
    vec![
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Both(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Both(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Both(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Both(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Either(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Either(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Either(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Either(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::Git),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::Bash),
        AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
        AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
        AchievementLanguage::Either(ProgrammingLanguage::C, ProgrammingLanguage::Java),
        AchievementLanguage::Single(ProgrammingLanguage::NoLanguage),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
        AchievementLanguage::Single(ProgrammingLanguage::C),
    ]
}

fn get_sprints() -> Vec<Sprint> {
    vec![
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint2,
        Sprint::Sprint2,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Project,
        Sprint::Sprint2,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint2,
        Sprint::Sprint2,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Sprint3,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint3,
        Sprint::Project,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Sprint2,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Unclear,
        Sprint::Sprint1,
        Sprint::Sprint1,
        Sprint::Project,
        Sprint::Project,
        Sprint::Project,
        Sprint::Project,
        Sprint::Project,
        Sprint::Project,
        Sprint::Project,
        Sprint::Sprint1,
        Sprint::Sprint2,
        Sprint::Sprint3,
        Sprint::Sprint4,
    ]
}

fn get_comments() -> Vec<Option<String>> {
    vec![
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "Gärna under projekt",
        "Gärna under projekt",
        "Gärna under projekt",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "Examineras via seminarium",
        "Kom till möten",
        "Lek TA",
        "Presentation till gruppen i forumet",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ]
    .iter()
    .map(|s| if s == &"" { None } else { Some(s.to_string()) })
    .collect::<Vec<Option<String>>>()
}
