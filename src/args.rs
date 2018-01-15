use std::env;
use regex::Regex;

#[derive(Debug)]
pub struct Args {
    filename: String,
    show_help: bool,
    invalid_option: Option<String>,
    truncate: bool,
}

impl Args {
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn should_show_help(&self) -> bool {
        self.show_help
    }

    pub fn has_invalid_option(&self) -> bool {
        self.invalid_option.is_some()
    }

    pub fn get_invalid_option(&self) -> &String {
        &self.invalid_option.as_ref().unwrap()
    }

    pub fn should_truncate(&self) -> bool {
        self.truncate
    }
}

fn init() -> Args {
    Args {
        filename: String::new(),
        show_help: false,
        invalid_option: None,
        truncate: false,
    }
}

pub fn get() -> Result<Args, String> {
    let command_args = env::args();

    // Parse options and operands out of the command_args iterator
    let mut options: Vec<String> = Vec::new();
    let mut operands: Vec<String> = Vec::new();

    let option_short_regex = Regex::new(r"^\-([^-]+)$").unwrap();
    for (index, token) in command_args.enumerate() {
        if index == 0 { // skip path to binary
            continue
        }
        if token.starts_with("-") {
            // Check if the token is a short option (which could be a group of them)
            let mut shorthand_option: Option<String> = None;

            for group in option_short_regex.captures_iter(token.as_str()) {
                match group.get(1) {
                    Some(group_ele) => shorthand_option = Some(group_ele.as_str().to_string()),
                    None => (),
                }
            }

            if shorthand_option.is_some() {
                for character in shorthand_option.as_ref().unwrap().chars() {
                    options.push(character.to_string());
                }
            } else {
                options.push(token);
            }

        }
        else {
            operands.push(token);
        }
    }

    //println!("{:?}", options);

    let mut parsed_args = init();

    // TODO: handle long option arguments since they appear in the same token (e.g. --opt=arg)?
    for option in options {
        match option.as_str() {
            "--help" => parsed_args.show_help = true,
            "--truncate" | "t" => parsed_args.truncate = true,
            _ => if parsed_args.invalid_option.is_none() {parsed_args.invalid_option = Some(option)},
        }
    }

    // If there is an invalid option present and --help is not present, err
    if !parsed_args.should_show_help() && parsed_args.has_invalid_option() {
        let invalid_option = parsed_args.get_invalid_option();
        match invalid_option.len() {
            1 => return Err(format!("invalid option -- {}", invalid_option)),
            _ => return Err(format!("unrecognized option '{}'", invalid_option)),
        }
    }

    // Expecting the filepath operand at this point
    match shift_vector(&mut operands) {
        Some(operand) => parsed_args.filename = operand,
        None => if !parsed_args.should_show_help() {return Err("missing file operand".to_string())},
    }

    //println!("{:?}", parsed_args);
    Ok(parsed_args)
}

// Removes and returns the first element of the vector if it exists
// "shift" is inspired by the JavaScript Array.prototype.shift() method
fn shift_vector<T>(vector: &mut Vec<T>) -> Option<T> {
    match vector.is_empty() {
        true => None,
        false => Some(vector.remove(0)),
    }
}
