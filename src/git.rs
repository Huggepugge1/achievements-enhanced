use chrono::Local;
use std::process::Command;

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
