use std::env;
pub mod cat_translator;

const CAT_TO_TEXT_SUB_COMMAND_NAME: &str = "decrypt";
const TEXT_TO_CAT_SUB_COMMAND_NAME: &str = "crypt";

fn show_help(program_name: &str) {
    println!(
        "some basic command examples:\n{} {} meoww meow mrow purrr mrowwww mrrrp mrowww meoww purr mrow mrow mroww meoww meowww mrrrrrp :3 :3c :3\n{} {} it works!\n\nalternatively to crypt or decrypt multiple times you can use the -d flag putting the depth afterward (if you want to crypt it two times -d 2)\n{} {} -d 2\n\nyou can use the --help or -h to get this message",
        program_name,
        CAT_TO_TEXT_SUB_COMMAND_NAME,
        program_name,
        TEXT_TO_CAT_SUB_COMMAND_NAME,
        program_name,
        TEXT_TO_CAT_SUB_COMMAND_NAME
    );
}

fn main() {
    let mut depth = 1;
    let mut text_starting_index = 2;

    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    // if not sub command in second place nor help command
    if args.len() <= 1
        || (&args[1] != CAT_TO_TEXT_SUB_COMMAND_NAME
            && &args[1] != TEXT_TO_CAT_SUB_COMMAND_NAME
            && &args[1] != "-h"
            && &args[1] != "--help")
    {
        println!(
            "the comamnd must include one of those two sub commands {}, {}\n",
            CAT_TO_TEXT_SUB_COMMAND_NAME, TEXT_TO_CAT_SUB_COMMAND_NAME,
        );
        show_help(program_name);
        return;
    }

    let subcommand = &args[1];

    if subcommand == "-h" || subcommand == "--help" {
        show_help(program_name);
        return;
    }

    // depth flag
    if args.len() >= 4 && &args[2] == "-d" && args[3].parse::<u16>().is_ok() {
        depth = args[3].parse::<u16>().unwrap();
        text_starting_index = 4;
    }

    if args.len() < text_starting_index + 1 {
        println!("the comamnd must include something to decrypt/crypt\n");
        show_help(program_name);
        return;
    }

    // decrypt
    if subcommand == CAT_TO_TEXT_SUB_COMMAND_NAME {
        let mut decrypted_message = args[text_starting_index..].join(" ");
        (0..depth).for_each(|_| {
            decrypted_message = cat_translator::cat_noises_to_text(&decrypted_message)
        });
        println!("{}", decrypted_message);
        // crypt
    } else {
        let mut crypted_message = args[text_starting_index..].join(" ");
        for _ in 0..depth {
            match cat_translator::text_to_cat(&crypted_message) {
                Ok(x) => crypted_message = x,
                Err(x) => {
                    println!("{}", x);
                    return;
                }
            }
        }
        println!("{}", crypted_message);
    }
}
