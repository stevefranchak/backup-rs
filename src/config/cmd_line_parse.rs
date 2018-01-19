use ::utilities::iterator_list::IteratorList;
use regex::Regex;
use std::env;

#[derive(Debug)]
pub enum Argument {
    ShortOpt(String),
    LongOpt(String, usize), // usize element is # of leading dashes
    Positional(String),
}

impl Argument {
    #[allow(dead_code)]
    pub fn is_short_opt(&self) -> bool {
        use self::Argument::*;

        match self {
            &ShortOpt(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_long_opt(&self) -> bool {
        use self::Argument::*;

        match self {
            &LongOpt(_, _) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_positional(&self) -> bool {
        use self::Argument::*;

        match self {
            &Positional(_) => true,
            _ => false,
        }
    }
}

pub type ArgumentList = IteratorList<Argument>;

pub fn parse_command_line_arguments() -> ArgumentList {
    let mut list = ArgumentList::new();

    let command_args = env::args();
    let option_short_regex = Regex::new(r"^(\-+)([^-]+)$").unwrap();

    for (index, token) in command_args.enumerate() {
        if index == 0 { // skip path to binary
            continue
        }
        if token.starts_with("-") { // non-expensive check to avoid regex matching every iteration
            for group in option_short_regex.captures_iter(token.as_str()) {
                let dashes_group: String = group.get(1).unwrap().as_str().to_string();
                let opt_name: String = match group.get(2) {
                    Some(group_ele) => group_ele.as_str().to_string(),
                    None => "".to_string(),
                };

                let num_dashes = dashes_group.len();
                if num_dashes > 1 { // this token is a long option
                    list.push(Argument::LongOpt(opt_name, num_dashes));
                } else { // this token is at least one short option
                    for character in opt_name.chars() {
                        list.push(Argument::ShortOpt(character.to_string()));
                    }
                }
            }
        }
        else {
            list.push(Argument::Positional(token));
        }
    }

    list
}
