use std::{iter::Peekable, ops::RangeInclusive, str::Chars};
use crate::{Language, Node};

// Source code
pub struct Source<'a> {
    chars: Peekable<Chars<'a>>,
    index: usize,
    rules: &'a Language
}

impl<'a> Source<'a> {
    pub fn new(lang: &'a Language, source: &'a str) -> Source<'a> {
        return Source {
            chars: source.chars().peekable(),
            index: 0,
            rules: lang
        }
    }
}


impl<'a> Source<'a> {
    fn next(&mut self) {
        self.index += 1;
        self.chars.next();
    }

    pub fn skip_white_space(&mut self) {
        let range = '\x00'..=' ';
        while self.eat_char_range(&range) {}
    }

    pub fn eat_rule(&mut self, rule_id: usize) -> Node {
        return self.rules.rules[rule_id].parse(self);
    }
    
    pub fn eat_char(&mut self, chr: char) -> bool {
        if Some(&chr) == self.chars.peek() {
            self.next();
            return true;
        }
        return false;
    }

    pub fn eat_char_range(&mut self, range: &RangeInclusive<char>) -> bool {
        if let Some(chr) = self.chars.peek() {
            if range.contains(chr) {
                self.next();
                return true;
            }
        }
        return false;
    }

    pub fn is_end_of_file(&mut self) -> bool {
        return self.chars.peek() == None;
    }
}
