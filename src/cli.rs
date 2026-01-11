use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::calculator::{
    Letter,
    decryptor::{DecryptInput, decrypt_numbers, decrypt_word},
    encryptor::encrypt_letter,
};

/// Blue Prince numeric core calculator
///
/// A program to solve puzzles in the video game 'Blue Prince'
/// Usage without a command runs the Terminal UI mode
#[derive(Parser, Debug)]
#[command(version, about, verbatim_doc_comment)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Parse application arguments using [`clap`]
///
/// # Exits
/// [`clap`] will exit the program if parsing fails
pub fn parse_command_or_exit() -> Option<Command> {
    Args::parse().command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Computes every 4-letter word for a given letter
    #[command(name = "encode")]
    Encrypt {
        /// Alphabetic letter in the range [A-Z] or [a-z]
        /// Examples:
        ///     encode D   
        ///     encode N
        ///
        /// Tip: output tend to be long (2000~6000 lines), it is recommended to pipe the output into a file
        ///     encode L > file.txt
        #[arg(value_name = "LETTER", value_parser = Letter::from_str, verbatim_doc_comment)]
        letter: Letter,
    },
    /// Computes numeric cores from a given cyphertext
    #[command(name = "decode")]
    Decrypt {
        /// Can be either:
        ///     <WORDS>       one or more 4-letter words (case insensitive) separated by spaces
        ///     <4-NUMBERS>   4 numbers separated by spaces
        ///
        /// Examples:
        ///     decode "CLAM tell FIND"
        ///     decode "156 21 9 7
        #[arg(value_name = "WORDS or 4-NUMBERS", value_parser = DecryptInput::from_str, verbatim_doc_comment)]
        input: DecryptInput,
    },
}

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Encrypt { letter } => encrypt(letter),
        Command::Decrypt { input } => decrypt(input),
    }
}

fn encrypt(letter: Letter) -> Result<(), String> {
    let cores = encrypt_letter(letter);
    for core in cores {
        println!("{}{}{}{}", core[0], core[1], core[2], core[3])
    }
    Ok(())
}

fn decrypt(input: DecryptInput) -> Result<(), String> {
    match input {
        DecryptInput::Numbers(numbers) => {
            let core = decrypt_numbers(numbers).map_err(|e| e.to_string())?;
            println!("{core}");
            Ok(())
        }
        DecryptInput::Words(words) => {
            let mut errors = Vec::new();
            for word in words {
                match decrypt_word(&word) {
                    Ok(core) => println!(
                        "{} - {core}",
                        Letter::try_from_num(core)
                            .map(Letter::to_char)
                            .unwrap_or('?')
                    ),
                    Err(e) => {
                        println!("{e}");
                        errors.push(word)
                    }
                }
            }
            match errors.is_empty() {
                true => Ok(()),
                false => Err(format!(
                    "Failed to decode {} word{}: `{}`",
                    errors.len(),
                    if errors.len() > 1 { "s" } else { "" },
                    errors.join(", ")
                )),
            }
        }
    }
}
