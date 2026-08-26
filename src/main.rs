use clap::{Parser, Subcommand};
use std::{io, io::Read};

use crate::cat_translator::{bin_to_cat_noises, bin_to_text, cat_noises_to_bin, text_to_bin};
use crate::error::Error;

pub mod cat_translator;
pub mod error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// turns text into cat noises
    Crypt {
        /// text to turn into cat_noises
        text: Option<String>,

        /// number of time you want it to be crypted
        #[arg(short, long, default_value_t = 1)]
        depth: u8,

        /// interprets the input text as binary
        #[arg(short, long)]
        from_bin: bool,

        /// turns the text into binary instead of cat noises
        #[arg(short, long)]
        to_bin: bool,
    },

    /// turns cat noises into text
    Decrypt {
        /// cat_noises to turn into text
        cat_noises: Option<String>,

        /// number of time you want it to be decrypted
        #[arg(short, long, default_value_t = 1)]
        depth: u8,

        /// interprets the input text as binary
        #[arg(short, long)]
        from_bin: bool,

        /// turns the cat noises into binary instead of text
        #[arg(short, long)]
        to_bin: bool,
    },
}

fn get_stdin_content() -> Result<String, Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn get_input(text: Option<String>) -> Result<String, Error> {
    match text {
        Some(text) => Ok(text),
        None => get_stdin_content(),
    }
}

fn crypt(mut text: String, depth: u8, from_bin: bool, to_bin: bool) -> Result<String, Error> {
    for _ in 0..depth {
        if !from_bin {
            text = text_to_bin(&text)?;
        }
        if !to_bin {
            text = bin_to_cat_noises(&text);
        }
    }
    Ok(text)
}

fn decrypt(mut cat_noises: String, depth: u8, from_bin: bool, to_bin: bool) -> String {
    for _ in 0..depth {
        if !from_bin {
            cat_noises = cat_noises_to_bin(&cat_noises);
        }
        if !to_bin {
            cat_noises = bin_to_text(&cat_noises);
        }
    }
    cat_noises
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let result = match args.command {
        Commands::Crypt {
            text,
            depth,
            from_bin,
            to_bin,
        } => crypt(get_input(text)?, depth, from_bin, to_bin)?,
        Commands::Decrypt {
            cat_noises,
            depth,
            from_bin,
            to_bin,
        } => decrypt(get_input(cat_noises)?, depth, from_bin, to_bin),
    };
    println!("{result}");
    Ok(())
}
