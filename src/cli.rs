use clap::{Parser, Subcommand};

use crate::calculator::{
    decryptor::{DecryptInput, decrypt_numbers, decrypt_word},
    encryptor::encrypt_letter,
    num_to_char,
};

/// Blue Prince numeric core calculator
///
/// A program to solve puzzles in the video game 'Blue Prince'
/// Usage without command uses Terminal UI mode by default
#[derive(Parser, Debug)]
#[command(version, about, long_about, verbatim_doc_comment)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub fn parse_command() -> Option<Commands> {
    Args::parse().command
}

#[derive(Subcommand, Debug, Default)]

pub enum Commands {
    /// Runs the app in Terminal-UI mode. Used by default when no command is provided
    #[default]
    UI,
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
        #[arg(value_name = "LETTER", verbatim_doc_comment)]
        c: char,
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
        #[arg(value_name = "WORDS or 4-NUMBERS", verbatim_doc_comment)]
        str: String,
    },
}

pub fn run(command: Commands) -> Result<(), String> {
    match command {
        Commands::UI => Ok(()),
        Commands::Encrypt { c } => encrypt(c),
        Commands::Decrypt { str } => decrypt(&str),
    }
}

fn encrypt(c: char) -> Result<(), String> {
    let cores = encrypt_letter(c)?;
    for core in cores {
        println!("{}{}{}{}", core[0], core[1], core[2], core[3])
    }
    Ok(())
}

fn decrypt(input: &str) -> Result<(), String> {
    let input = DecryptInput::parse(input).map_err(|e| e.to_string())?;
    match input {
        DecryptInput::Numbers(numbers) => {
            let core = decrypt_numbers(numbers).map_err(|e| e.to_string())?;
            println!("{core}");
            Ok(())
        }
        DecryptInput::Words(words) => {
            let mut errors = Vec::new();
            for word in words {
                match decrypt_word(word) {
                    Ok(core) => println!("{} - {core}", num_to_char(core).unwrap_or('?')),
                    Err(e) => {
                        println!("{e}");
                        errors.push(word)
                    }
                }
            }
            match errors.is_empty() {
                true => Ok(()),
                false => Err(format!(
                    "failed to decode {} values. {}",
                    errors.len(),
                    errors.join(", ")
                )),
            }
        }
    }
}
