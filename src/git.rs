use std::process::Command;

use chrono::{DateTime, Local};

pub fn git_pull() {
    println!("Pulling changes from git");

    println!("command: git pull");

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "pull"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git pull")
            .output()
            .expect("failed to execute process")
    };

    println!(
        "output: {}",
        String::from_utf8(output.stdout).unwrap() + &String::from_utf8(output.stderr).unwrap()
    );
}

pub fn git_add() {
    println!("Adding changes to git");

    println!("command: git add .");
    println!("{:?}", Command::new("cmd").args(&["/C", "git", "add", "."]));

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "add", "."])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git add .")
            .output()
            .expect("failed to execute process")
    };

    println!(
        "output: {}",
        String::from_utf8(output.stdout).unwrap() + &String::from_utf8(output.stderr).unwrap()
    );
}

pub fn git_commit() {
    println!("Committing changes to git");

    let message = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let command = format!("git commit -m \"{}\"", message);

    println!("command: {}", command);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "commit", "-m", &message])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    };

    println!(
        "output: {}",
        String::from_utf8(output.stdout).unwrap() + &String::from_utf8(output.stderr).unwrap()
    );
}

pub fn git_push() {
    println!("Pushing changes to git");

    println!("command: git push");

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "push"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git push")
            .output()
            .expect("failed to execute process")
    };

    println!(
        "output: {}",
        String::from_utf8(output.stdout).unwrap() + &String::from_utf8(output.stderr).unwrap()
    );
}

pub fn git_get_commits() -> Vec<(String, DateTime<Local>)> {
    println!("Getting commits from git");

    println!("command: git log --format=\"%h %as\"");

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "log", "--format=\"%h %ai\""])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git log --format=\"%h %ai\"")
            .output()
            .expect("failed to execute process")
    };

    let output = String::from_utf8(output.stdout).unwrap();

    let commits = output
        .trim()
        .split("\n")
        .map(|s| {
            let splitted = s.split_once(" ").unwrap();
            println!("splitted: {:?}", splitted);
            (
                splitted.0.to_string(),
                DateTime::parse_from_str(splitted.1, "%Y-%m-%d %H:%M:%S %z")
                    .unwrap()
                    .with_timezone(&Local),
            )
        })
        .collect::<Vec<(String, DateTime<Local>)>>();

    commits
}

pub fn git_get_achievements_from_commit(commit: String) -> String {
    println!("Getting achievements from commit {}", commit);

    println!("command: git show {}:achievements.csv", commit);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "git", "show", &format!("{}:achievements.csv", commit)])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&format!("git show {}:achievements.csv", commit))
            .output()
            .expect("failed to execute process")
    };

    let output = String::from_utf8(output.stdout).unwrap();

    output
}
