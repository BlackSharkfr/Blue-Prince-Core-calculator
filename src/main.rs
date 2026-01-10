use crate::cli::Commands;

mod calculator;
mod cli;
mod ui;

fn main() -> Result<(), String> {
    match cli::parse_command() {
        None | Some(Commands::UI { .. }) => ui::run().map_err(|e| e.to_string()),
        Some(command) => cli::run(command),
    }
}
