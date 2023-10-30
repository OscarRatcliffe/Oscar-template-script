extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

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
    let include_frontend = Confirm::with_theme(&theme)
        .with_prompt("Include a frontend?")
        .interact()
        .unwrap();

    // Question 3: Public or private Git repo
    let repo_type = Select::with_theme(&theme)
        .with_prompt("Choose Git repository type:")
        .item("Public")
        .item("Private")
        .interact()
        .unwrap();

    match language {
        0 => println!("You chose Rust."),
        1 => println!("You chose TypeScript.")
    }

    if include_frontend {
        println!("You chose to include a frontend.");
    } else {
        println!("You chose not to include a frontend.");
    }

    match repo_type {
        0 => println!("You chose a public Git repository."),
        1 => println!("You chose a private Git repository.")
    }
}
