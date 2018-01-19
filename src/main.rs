extern crate regex;

// NOTE: Not using https://github.com/kbknapp/clap-rs because this is a learning-the-language project
mod config;
mod utilities;

use std::process;
use config::Config;

const SUCCESS_EXIT_CODE: i32 = 0;
const INVALID_ARG_EXIT_CODE: i32 = 22;

const PROG_NAME: &'static str = "backup";

fn main() {
    let config = get_config_or_exit();

    // --help trumps any other configuration fields
    if config.should_show_help() {
        print_help_and_exit();
    }

    println!("The source filename is: {}", config.get_filename());
}

// Returns config::Config if there are no issues parsing or validating command line arguments
// Else, prints errors to stderr and exits with code EINVAL
fn get_config_or_exit() -> Config {
    let config: Option<Config> = match config::get() {
        Ok(config) => Some(config),
        Err(err) => {
            print_prog_error(&err);
            print_try_help_error();
            None
        }
    };

    if config.is_none() {
        process::exit(INVALID_ARG_EXIT_CODE);
    }

    config.unwrap()
}

// Help message is loosely modeled after "cp --help"
fn print_help_and_exit() {
    println!("Usage: {PROG_NAME} [OPTION]... FILE_TO_BACKUP

Puts a copy of FILE_TO_BACKUP in the same directory as the original file.
If this is the only copy, this copy's filename has \".bak\" appended to it.
If other copies exist, this copy's filename has \".bak.n\" appended to it, where
n is the (nth - 1) copy. For example, if a copy of a.txt is being made and a.txt.bak
exists, then this copy is written to a.txt.bak.1.",
    PROG_NAME = PROG_NAME);
    process::exit(SUCCESS_EXIT_CODE);
}

fn print_prog_error(err: &String) {
    eprintln!("{}: {}", PROG_NAME, err);
}

fn print_try_help_error() {
    eprintln!("Try '{} --help' for more information.", PROG_NAME);
}
