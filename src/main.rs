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

    rprintln!(
        "{}",
        format!("Weclome to {}, a simple tool to allow the archiving of Group Walls!", title).blue()
    );

    rprintln!(
        "{} {}",
        "This tool is still in early stages, you can find out more here:".blue(),
        "https://go.thecasual.dev/X5eURX".green().bold()
    );

    components::fetch_wall::fetch();

}

// project made by thecasualdev ⚡

