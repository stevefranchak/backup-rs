mod parse;

use self::parse::Argument;

#[derive(Debug)]
pub struct Args {
    filename: String,
    show_help: bool,
    truncate: bool,
}

impl Args {
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn should_show_help(&self) -> bool {
        self.show_help
    }

    fn set_should_show_help(&mut self, show_help: bool) {
        self.show_help = show_help;
    }

    #[allow(dead_code)]
    pub fn should_truncate(&self) -> bool {
        self.truncate
    }

    fn set_should_truncate(&mut self, should_truncate: bool) {
        self.truncate = should_truncate;
    }

    pub fn new() -> Args {
        Args {
            filename: String::new(),
            show_help: false,
            truncate: false,
        }
    }

}

pub fn get() -> Result<Args, String> {
    let args_list = parse::parse_command_line_arguments();
    let mut parsed_args = Args::new();

    let mut invalid_option: Option<&Argument> = None;
    // Sadly, the operands vector is required since the program does not know in advance whether
    //  non-opt tokens are arguments to an option or positional arguments to the executable.
    // If this were to be designed such that the caller of the parser told it which options to
    //  expect (see: clap), then the operands vector *may not* be needed.
    let mut operands: Vec<&String> = Vec::new();

    while let Some(arg) = args_list.next() {
        use self::parse::Argument::*;
        match arg {
            &ShortOpt(ref name) => {
                match name.as_str() {
                    "t" => parsed_args.set_should_truncate(true),
                    _ => set_invalid_option(&mut invalid_option, Some(arg)),
                }
            }
            &LongOpt(ref name, _) => {
                match name.as_str() {
                    "help" => parsed_args.set_should_show_help(true),
                    "truncate" => parsed_args.set_should_truncate(true),
                    _ => set_invalid_option(&mut invalid_option, Some(arg)),
                }
            }
            // If an option requires arguments, they should be gotten by calling list.next()
            //  inside of the handler for that argument
            // Therefore, at this point, any positional arguments should be operands to the command
            &Positional(ref operand) => operands.push(operand),
        }
    }

    // If there is an invalid option present and --help is not present, err
    if !parsed_args.should_show_help() && invalid_option.is_some() {
        use self::parse::Argument::*;
        match invalid_option.unwrap() {
            &ShortOpt(ref name) => return Err(format!("invalid option -- {}", name)),
            &LongOpt(ref name, _) => return Err(format!("unrecognized option '{}'", name)),
            _ => (),
        }
    }

    // Expecting the filepath to be the first element in the operands vector at this point
    // If the operands list is empty, err
    // Extra operands are ignored
    if operands.len() > 0 {
        parsed_args.set_filename(operands.get(0).unwrap().to_string());
    } else {
        if !parsed_args.should_show_help() {
            return Err("missing file operand".to_string());
        }
    }

    //println!("{:?}", parsed_args);
    Ok(parsed_args)
}

fn set_invalid_option<'a>(invalid_option: &mut Option<&'a Argument>, option_to_set: Option<&'a Argument>) {
    if invalid_option.is_none() {
        *invalid_option = option_to_set;
    }
}
