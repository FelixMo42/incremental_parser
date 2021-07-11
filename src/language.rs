use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};
use tblit::screen::Color;

/* Node */

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
    pub subs: Vec<Node<'a>>
}

#[derive(PartialEq, Eq)]
pub enum Rule {
    Char(RangeInclusive<char>),
    Token(usize)
}

impl Rule {
    fn parse(&self, lang: &Language, cursor: &mut Cursor) -> bool {
        match self {
            Rule::Char(range) => {
                cursor.chars.next_if(|(_, chr)| range.contains(chr)).is_some()
            },
            Rule::Token(token) => {
                return lang[*token].parse(lang, cursor);
            }
        }
    }
}

/* Step */

#[derive(PartialEq, Eq)]
pub struct Step (pub Vec<(Rule, usize)>, pub bool);

impl Step {
    #[inline]
    pub fn rules(&self) -> &Vec<(Rule, usize)> {
        return &self.0;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return self.1;
    }
}

/* Cursor */

pub type Language = Vec<Token>;

type CursorIter<'a> = Peekable<Skip<CharIndices<'a>>>;

#[derive(Clone)]
pub struct Cursor<'a> {
    pub chars: CursorIter<'a> 
}

impl<'a> Cursor<'a> {
    pub fn new(chars: Peekable<Skip<CharIndices<'a>>>) -> Cursor<'a> {
        return Cursor { chars };
    }

    pub fn done(&mut self) -> bool {
        return self.chars.peek().is_none();
    }

    pub fn save(&self) -> Peekable<Skip<CharIndices<'a>>> {
        return self.chars.clone();
    }

    pub fn restore(&mut self, save: CursorIter<'a>) {
        self.chars = save;
    }

    pub fn advanced_from(&mut self, save: &mut CursorIter<'a>) -> bool {
        self.chars.peek() != save.peek()
    }
} 

/* Token */

#[derive(PartialEq, Eq)]
pub struct Token {
    pub color: Color,
    pub steps: Vec<Step>
}


impl Token {
    pub fn new(color: Color, steps: Vec<Step>) -> Token {
        return Token {
            color,
            steps
        }
    }

    pub fn parse(&self, lang: &Language, cursor: &mut Cursor) -> bool {
        let mut index = 0;
        let mut save = cursor.save();

        while self.steps[index].rules().iter().any(|(rule, i)| {
            if rule.parse(lang, cursor) {
                index = i.clone();
                return true;
            }

            return false
        }) {}

        let success =
            self.steps[index].is_final() &&
            cursor.advanced_from(&mut save);

        if !success {
            cursor.restore(save);
        }

        return success
    }
}

