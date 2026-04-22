use clap::{Parser, Subcommand};

use crate::cat_translator::{bin_to_cat_noises, bin_to_text, cat_noises_to_bin, text_to_bin};

pub mod cat_translator;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// turns text into cat noises
    Crypt {
        /// text to turn into cat_noises
        text: String,

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
        cat_noises: String,

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

fn crypt(mut text: String, depth: u8, from_bin: bool, to_bin: bool) -> String{
    for _ in 0..depth{
        if !from_bin{
            text = text_to_bin(&text).unwrap_or_else(|x| panic!("{x}"));
        }
        if !to_bin{
            text = bin_to_cat_noises(&text);
        }
    }
    text
}

fn decrypt(mut cat_noises: String, depth: u8, from_bin: bool, to_bin: bool) -> String{
    for _ in 0..depth{
        if !from_bin{
            cat_noises = cat_noises_to_bin(&cat_noises);
        }
        if !to_bin{
            cat_noises = bin_to_text(&cat_noises);
        }
    }
    cat_noises
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Commands::Crypt { text, depth, from_bin, to_bin } => crypt(text, depth, from_bin, to_bin),
        Commands::Decrypt { cat_noises, depth, from_bin, to_bin } => decrypt(cat_noises, depth, from_bin, to_bin),
    };
    println!("{result}");
}
