use crate::git;

use chrono::{DateTime, Local};
use csv;
use std::collections::HashMap;

struct Commit {
    date: DateTime<Local>,
    dones: u8,
    planned: u8,
}

pub fn generate() {
    let commits = git::git_get_commits();

    let mut dones: HashMap<DateTime<Local>, u8> = HashMap::new();
    let mut planned: HashMap<DateTime<Local>, u8> = HashMap::new();

    for (commit, date) in commits {
        let achievements = git::git_get_achievements_from_commit(commit.clone());

        let mut reader = csv::ReaderBuilder::new().from_reader(achievements.as_bytes());

        for result in reader.records() {
            let record = result.unwrap();
            dones.insert(
                date,
                dones.get(&date).unwrap_or(&0) + (record[4] == *"true") as u8,
            );
            planned.insert(
                date,
                planned.get(&date).unwrap_or(&0) + (record[5] == *"true") as u8,
            );
        }
    }

    let mut dones = dones
        .into_iter()
        .map(|(date, dones)| (date, 65 - dones))
        .collect::<Vec<(DateTime<Local>, u8)>>();
    dones.sort_by(|a, b| a.0.cmp(&b.0));

    let mut planned = planned.into_iter().collect::<Vec<(DateTime<Local>, u8)>>();
    planned.sort_by(|a, b| a.0.cmp(&b.0));

    planned = planned
        .into_iter()
        .enumerate()
        .map(|(i, (date, doing_soon))| (date, dones[i].1 - doing_soon))
        .collect::<Vec<(DateTime<Local>, u8)>>();

    let mut commits = Vec::new();
    for (i, (date, done)) in dones.iter().enumerate() {
        commits.push(Commit {
            date: *date,
            dones: *done,
            planned: planned[i].1,
        });
    }

    let mut csv = csv::Writer::from_path("burndown.csv").unwrap();
    csv.write_record(&["date", "done", "planned"]).unwrap();
    for commit in commits {
        csv.write_record(&[
            commit.date.format("%Y-%m-%d").to_string(),
            commit.dones.to_string(),
            commit.planned.to_string(),
        ])
        .unwrap();
    }
}
