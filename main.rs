extern crate dialoguer;
extern crate reqwest;
extern crate zip;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn downloadFile(url) {
    let response = reqwest::blocking::get(url);

    if response.status().is_success() {
        let content = response.bytes();

        let mut file = File::create("{}.zip", url);

        file.write_all(&content);
    }
}

fn main() {
    let theme = ColorfulTheme::default();

    // Question 1: Rust or TypeScript
    let language = Select::with_theme(&theme)
        .with_prompt("Choose a language:")
        .item("Rust")
        .item("TypeScript")
        .interact()
        .unwrap();

    // Question 2: Include a frontend
    let includeFrontend = Confirm::with_theme(&theme)
        .with_prompt("Include a frontend?")
        .interact()
        .unwrap();

    // Question 3: Public or private Git repo
    let repoType = Select::with_theme(&theme)
        .with_prompt("Choose Git repository type:")
        .item("Public")
        .item("Private")
        .interact()
        .unwrap();

    match language {
        0 => {Command::new("git")
            .arg("clone")
            .arg("--depth=1")
            .arg("--single-branch")
            .arg("https://github.com/OscarRatcliffe/Oscar-template-script.git")
            .arg("typescript")
            .status()
            .expect("Failed to execute Git command");
        }

        1 => {Command::new("git")
            .arg("clone")
            .arg("--depth=1")
            .arg("--single-branch")
            .arg("https://github.com/OscarRatcliffe/Oscar-template-script.git")
            .arg("rust")
            .status()
            .expect("Failed to execute Git command");
        }

        _ => println!("Invalid choice."),
    }

    if includeFrontend {
        // Command::new("git")
        //     .arg("clone")
        //     .arg("--depth=1")
        //     .arg("--single-branch")
        //     .arg("https://github.com/OscarRatcliffe/Oscar-template-script.git")
        //     .arg("frontend")
        //     .status()
        //     .expect("Failed to execute Git command")
        
        downloadFile("https://raw.githubusercontent.com/OscarRatcliffe/Oscar-template-script/master/frontend.zip")
    } 

    Command::new("git")
            .arg("init");
    

    Command::new("git")
            .arg("add")
            .arg("*");

    Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("inital commit");

    match repoType {
        0 => println!("You chose a public Git repository."),
        1 => println!("You chose a private Git repository."),
        _ => println!("Invalid choice."),
    }
}
