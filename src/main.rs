mod calculator;
mod cli;
mod ui;

fn main() -> Result<(), String> {
    match cli::parse_command() {
        Some(command) => cli::run(command),
        None => ui::run().map_err(|e| e.to_string()),
    }
}
