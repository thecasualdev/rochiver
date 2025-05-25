use std::{io::{self, Write}, process::exit};

use colored::Colorize;

mod components;
mod config;
mod macros;

fn main() {
    
    let title = format!(
        "{} {}",
        "ROCHIVER".yellow().bold(),
        env!("CARGO_PKG_VERSION").yellow().bold()
    );
    
    let mut group_id = String::new();

    rprintln!(
        "{}",
        format!("Weclome to {}, a simple tool to allow the archiving of Group Walls!", title).blue()
    );
    rprintln!(
        "{} {}",
        "This tool is still in early stages, you can find out more here:".blue(),
        "https://go.thecasual.dev/X5eURX".green().bold()
    );
    rprint!(
        "{}",
        "Insert the group ID here: ".blue()
    );

    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut group_id)
        .expect("Failed to read line");

    let group_id = group_id.trim();

    rprintln!(
        "{}",
        "Verifying group ID".green()
    );

    if components::check_group::verify(group_id) {
        components::fetch_wall::fetch();
    } else {
        rprintln!(
            "{}",
            "Could not verify group, closing app".red().bold()
        );
        exit(0)
    }

}

// project made by thecasualdev ⚡

