#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum Langs {
    English,
    Swedish,
}

impl std::fmt::Display for Langs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Langs::English => write!(f, "English"),
            Langs::Swedish => write!(f, "Swedish"),
        }
    }
}

pub struct Language {
    pub id: String,
    pub title: String,
    pub deadline: String,
    pub done: String,
    pub present_soon: String,
    pub grade: String,
    pub presenting_type: String,
    pub programming_language: String,
    pub sprint: String,
    pub comment: String,
    pub click_to_sort: String,
    pub right_click_to_filter: String,
    pub no_specific_language: String,

    pub settings: String,
    pub file: String,
    pub edit: String,
    pub clear_done: String,
    pub clear_present_soon: String,
    pub clear_filters: String,
    pub save: String,

    pub date: String,
    pub optimal: String,
    pub minimum: String,
    pub target: String,
    pub minimum_to_reach_target_grade: String,
    pub click_to_hide_passed_labs: String,
    pub click_to_show_passed_labs: String,
    pub click_to_show_achievements_done: String,
    pub click_to_show_achievements_left: String,
    pub lab: String,

    pub font_size: String,
    pub dark_mode: String,
    pub target_grade: String,
    pub max_per_lab: String,
    pub language: String,
    pub git: String,
}

pub fn get_english() -> Language {
    Language {
        id: String::from("ID"),
        title: String::from("Title"),
        deadline: String::from("Deadline"),
        done: String::from("Done"),
        present_soon: String::from("Present Soon"),
        grade: String::from("Grade"),
        presenting_type: String::from("Presentation type"),
        programming_language: String::from("Programming Language"),
        sprint: String::from("Sprint"),
        comment: String::from("Comment"),
        click_to_sort: String::from("Click to sort"),
        right_click_to_filter: String::from(
            "Right click to filter out\nShift Right click to only show this",
        ),
        no_specific_language: String::from("No Specific Language"),

        settings: String::from("Settings"),
        file: String::from("File"),
        edit: String::from("Edit"),
        clear_done: String::from("Clear Done"),
        clear_present_soon: String::from("Clear Present Soon"),
        clear_filters: String::from("Clear Filters"),
        save: String::from("Save"),

        date: String::from("Date"),
        optimal: String::from("Optimal"),
        minimum: String::from("Minimum"),
        target: String::from("Target"),
        minimum_to_reach_target_grade: String::from("Minimum to reach target grade"),
        click_to_hide_passed_labs: String::from("Click to hide passed labs"),
        click_to_show_passed_labs: String::from("Click to show passed labs"),
        click_to_show_achievements_done: String::from("Click to show achievements done"),
        click_to_show_achievements_left: String::from("Click to show achievements left"),
        lab: String::from("Lab"),

        font_size: String::from("Font Size"),
        dark_mode: String::from("Dark Mode"),
        target_grade: String::from("Target Grade"),
        max_per_lab: String::from("Max Achievements Per Lab"),
        language: String::from("Language"),
        git: String::from("Use Git To Sync"),
    }
}

pub fn get_swedish() -> Language {
    Language {
        id: String::from("ID"),
        title: String::from("Namn"),
        deadline: String::from("Deadline"),
        done: String::from("Färdig"),
        present_soon: String::from("Presentera snart"),
        grade: String::from("Betyg"),
        presenting_type: String::from("Typ av presentation"),
        programming_language: String::from("Programmeringsspråk"),
        sprint: String::from("Sprint"),
        comment: String::from("Kommentar"),
        click_to_sort: String::from("Klicka för att sortera"),
        right_click_to_filter: String::from(
            "Högerklicka för att filtrera bort\nSkift + Högerklicka för att visa endast denna",
        ),
        no_specific_language: String::from("Inget Specifikt Språk"),

        settings: String::from("Inställningar"),
        file: String::from("Arkiv"),
        edit: String::from("Redigera"),
        clear_done: String::from("Rensa Färdiga"),
        clear_present_soon: String::from("Rensa Presentera Snart"),
        clear_filters: String::from("Rensa Filter"),
        save: String::from("Spara"),

        date: String::from("Datum"),
        optimal: String::from("Optimalt"),
        minimum: String::from("Minimum"),
        target: String::from("Mål"),
        minimum_to_reach_target_grade: String::from("Minimum för att nå betyg"),
        click_to_hide_passed_labs: String::from("Klicka för att dölja passerade labbar"),
        click_to_show_passed_labs: String::from("Klicka för att visa passerade labbar"),
        click_to_show_achievements_done: String::from("Klicka för att visa färdiga achievements"),
        click_to_show_achievements_left: String::from(
            "Klicka för att visa kvarvarande achievements",
        ),
        lab: String::from("Lab"),

        font_size: String::from("Textstorlek"),
        dark_mode: String::from("Mörkt Tema"),
        target_grade: String::from("Målbetyg"),
        max_per_lab: String::from("Max Achievements Per Lab"),
        language: String::from("Språk"),
        git: String::from("Använd Git För Att Synka"),
    }
}