use std::env;

#[derive(Debug)]
pub struct Args {
    filename: String,
    show_help: bool,
    invalid_option: Option<String>,
}

impl Args {
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn should_show_help(&self) -> bool {
        self.show_help
    }
}

fn init() -> Args {
    Args {
        filename: String::new(),
        show_help: false,
        invalid_option: None,
    }
}

pub fn get() -> Result<Args, String> {
    let command_args = env::args();

    // Parse options and operands out of the command_args iterator
    let mut options: Vec<String> = Vec::new();
    let mut operands: Vec<String> = Vec::new();
    for (index, token) in command_args.enumerate() {
        if index == 0 { // skip path to binary
            continue
        }
        if token.starts_with("-") {
            options.push(token);
        }
        else {
            operands.push(token);
        }
    }

    let mut parsed_args = init();

    // TODO: parse short options that appear in the same token (e.g. -dTe)
    // TODO: handle long option arguments since they appear in the same token (e.g. --opt=arg)?
    for option in options {
        match option.as_str() {
            "--help" => parsed_args.show_help = true,
            _ => if parsed_args.invalid_option.is_none() {parsed_args.invalid_option = Some(option)},
        }
    }

    if !parsed_args.should_show_help() && parsed_args.invalid_option.is_some() {
        return Err(format!("unrecognized option '{}'", parsed_args.invalid_option.as_ref().unwrap()));
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
