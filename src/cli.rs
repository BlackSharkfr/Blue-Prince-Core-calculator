use clap::{Parser, Subcommand};

use crate::calculator::{
    decryptor::{DecryptInput, decrypt_numbers, decrypt_word},
    encryptor::encrypt_letter,
    num_to_char,
};

/**
    Blue Prince numeric core calculator

    A program to solve puzzles in the video game 'Blue Prince'.
    Usage without options runs the Terminal UI mode.
*/
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the app in Terminal-UI mode (default)
    UI,
    /// Computes every 4-letter word for a given letter
    Encode {
        /**
            Alphabetic letter in the range [A-Z] or [a-z]

            examples

            encode D

            encode N

            Tip : outputs tend to be long, it is recommended to pipe this command into a file

            encode L > file.txt
        */
        #[arg(value_name = "LETTER")]
        encode: char,
    },
    /// Computes numeric cores from a given cyphertext.
    Decode {
        /**
            Can be either :

            - WORDS : one or more 4-letter words (case insensitive) separated by spaces.

            - 4-NUMBERS : 4 numbers separated by spaces.

                examples

                decode "CLAM tell FIND"

                decode "156 21 9 7"
        */
        #[arg(value_name = "WORDS or 4-NUMBERS")]
        decode: String,
    },
}

pub fn parse_command() -> Option<Commands> {
    match Args::parse().command {
        None | Some(Commands::UI { .. }) => None,
        Some(other) => Some(other),
    }
}

pub fn run(command: Commands) -> Result<(), String> {
    match command {
        Commands::UI { .. } => Ok(()),
        Commands::Encode { encode } => encode_char(encode),
        Commands::Decode { decode } => decode_string(&decode),
    }
}

fn encode_char(c: char) -> Result<(), String> {
    let cores = encrypt_letter(c)?;
    for core in cores {
        println!("{}{}{}{}", core[0], core[1], core[2], core[3])
    }
    Ok(())
}

fn decode_string(input: &str) -> Result<(), String> {
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
