// NOTE: Not using https://github.com/kbknapp/clap-rs because this is a learning-the-language project
mod args;

use std::process;

const SUCCESS_EXIT_CODE: i32 = 0;
const INVALID_ARG_EXIT_CODE: i32 = 22;

const PROG_NAME: &'static str = "backup";

fn main() {
    let args = get_args_or_exit();

    // --help trumps any of the other command args
    if args.should_show_help() {
        print_help_and_exit();
    }

    println!("The source filename is: {}", args.get_filename());
}

// Returns args::Args if there are no issues parsing or validating command args
// Else, prints errors to stderr and exits with code EINVAL
fn get_args_or_exit() -> args::Args {
    let args: Option<args::Args> = match args::get() {
        Ok(args) => Some(args),
        Err(err) => {
            print_prog_error(&err);
            print_try_help_error();
            None
        }
    };

    if args.is_none() {
        process::exit(INVALID_ARG_EXIT_CODE);
    }

    args.unwrap()
}

// Help message is loosely modeled after "cp --help"
fn print_help_and_exit() {
    println!("Usage: {PROG_NAME} FILE_TO_BACKUP

Puts a copy of FILE_TO_BACKUP in the same directory as the original file.
If this is the only copy, this copy's filename has \".bak\" appended to it.
If other copies exist, this copy's filename has \".bak.n\" appended to it, where
n is the (nth - 1) copy. For example, if a copy of a.txt is being made and a.txt.bak
exists, then this copy is written to a.txt.bak.1.",
    PROG_NAME=PROG_NAME);
    process::exit(SUCCESS_EXIT_CODE);
}

fn print_prog_error(err: &String) {
    eprintln!("{}: {}", PROG_NAME, err);
}

fn print_try_help_error() {
    eprintln!("Try '{} --help' for more information.", PROG_NAME);
}
