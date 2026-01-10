mod calculator;
mod cli;
mod ui;

fn main() -> Result<(), String> {
    match cli::parse_command_or_exit() {
        None => ui::run(),
        Some(command) => cli::run(command),
    }
}
