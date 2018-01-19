use std::cell::Cell;
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


pub struct IteratorList<T> {
    list: Vec<T>,
    current_index: Cell<usize>,
}

impl <T> IteratorList<T> {
    pub fn new() -> IteratorList<T> {
        IteratorList {
            list: Vec::new(),
            current_index: Cell::new(0),
        }
    }

    #[allow(dead_code)]
    pub fn with_vec(list: Vec<T>) -> IteratorList<T> {
        IteratorList {
            list: list,
            current_index: Cell::new(0),
        }
    }

    pub fn next(&self) -> Option<&T> {
        match self.list.get(self.current_index.get()) {
            Some(item) => {
                self.current_index.set(self.current_index.get() + 1);
                Some(item)
            },
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn peek(&self, window: usize) -> Option<&[T]> {
        let current_index = self.current_index.get();
        let to_index = current_index + window;
        self.list.get(current_index..to_index)
    }

    pub fn push(&mut self, item: T) {
        self.list.push(item);
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.current_index.set(0);
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    #[allow(dead_code)]
    pub fn get_current_position(&self) -> usize {
        self.current_index.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_iterator_list() {
        let list: IteratorList<String> = IteratorList::new();
        assert_eq!(list.list.len(), 0);
        assert_eq!(list.current_index.get(), 0);
    }

    #[test]
    fn test_iterator_list_next_iteration() {
        let list = IteratorList::with_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.len(), 5);

        assert_eq!(list.get_current_position(), 0);
        assert_eq!(list.next(), Some(&1));

        assert_eq!(list.get_current_position(), 1);
        assert_eq!(list.next(), Some(&2));

        assert_eq!(list.get_current_position(), 2);
        assert_eq!(list.next(), Some(&3));

        assert_eq!(list.get_current_position(), 3);
        assert_eq!(list.next(), Some(&4));

        assert_eq!(list.get_current_position(), 4);
        assert_eq!(list.next(), Some(&5));

        assert_eq!(list.next(), None);
        assert_eq!(list.get_current_position(), 5);
        assert_eq!(list.next(), None);
        assert_eq!(list.get_current_position(), 5);
    }

    #[test]
    fn test_iterator_list_reset() {
        let mut list = IteratorList::with_vec(vec![1, 2, 3]);

        // Doing something that a user of this should not do for the sake of testing
        list.current_index.set(3);
        assert_eq!(list.next(), None);
        assert_eq!(list.get_current_position(), 3);

        list.reset();
        assert_eq!(list.get_current_position(), 0);
        assert_eq!(list.next(), Some(&1));
        assert_eq!(list.get_current_position(), 1);
    }

    #[test]
    fn test_iterator_list_push() {
        let mut list: IteratorList<usize> = IteratorList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.get_current_position(), 0);

        assert_eq!(list.next(), Some(&1));
        assert_eq!(list.next(), Some(&2));
        assert_eq!(list.next(), Some(&3));
        assert_eq!(list.next(), None);
        assert_eq!(list.get_current_position(), 3);

        list.push(4);
        assert_eq!(list.len(), 4);
        assert_eq!(list.get_current_position(), 3);

        assert_eq!(list.next(), Some(&4));
        assert_eq!(list.next(), None);
        assert_eq!(list.get_current_position(), 4);
    }

    #[test]
    fn test_iterator_list_peek() {
        let list = IteratorList::with_vec(vec!["this", "is", "a", "test"]);

        let slice = list.peek(1);
        assert_eq!(slice, Some(&["this"][..]));

        let slice = list.peek(2);
        assert_eq!(slice, Some(&["this", "is"][..]));

        let slice = list.peek(4);
        assert_eq!(slice, Some(&["this", "is", "a", "test"][..]));

        let slice = list.peek(5);
        assert_eq!(slice, None);

        let item = list.next();
        assert_eq!(item, Some(&"this"));

        let slice = list.peek(1);
        assert_eq!(slice, Some(&["is"][..]));

        let slice = list.peek(4);
        assert_eq!(slice, None);

        let slice = list.peek(0);
        assert_eq!(slice, Some(&[][..]));
    }

    #[test]
    fn test_iterator_list_use_case1() {
        let list = IteratorList::with_vec(vec!["this", "is", "a", "test"]);
        let mut passed_items: Vec<&str> = Vec::new();

        while let Some(item) = list.next() {
            passed_items.push(item);
            if item == &"is" {
                let word = list.next().unwrap();
                println!("{}", word);
            }
        }

        assert_eq!(passed_items.join(" "), "this is test");
    }

}
