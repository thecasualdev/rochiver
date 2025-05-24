
#[macro_export]
macro_rules! rprintln {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        println!(
            "{} {}",
            "[ROCHIVER]".yellow().bold(),
            format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! rprint {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        print!(
            "{} {}",
            "[ROCHIVER]".yellow().bold(),
            format!($($arg)*))
    }};
}
