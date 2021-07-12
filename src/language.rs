use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};
use tblit::screen::Color;

/* Node */

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
    pub subs: Vec<Node<'a>>
}

/* Rule */

#[derive(PartialEq, Eq)]
pub enum Rule {
    Char(RangeInclusive<char>),
    Token(usize)
}

impl Rule {
    fn parse<'a>(&self, lang: &'a Language, cursor: &mut Cursor) -> Option<Node<'a>> {
        match self {
            Rule::Char(range) => {
                cursor.chars.next_if(|(_, chr)| range.contains(chr)).map(|(i, _)| Node {
                    span: (i, i + 1),
                    kind: &lang[0],
                    subs: vec![]
                })
            },
            Rule::Token(token) => {
                lang[*token].parse(lang, cursor)
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
    pub src: &'a str,
    pub chars: CursorIter<'a> 
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str, start: usize) -> Cursor<'a> {
        return Cursor {
            src,
            chars: src.char_indices().skip(start).peekable()
        };
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

    pub fn get_span(&mut self, save: &mut CursorIter<'a>) -> (usize, usize) {
        return (
            save.peek().unwrap().0,
            self.chars.peek().map(|(i, _)| i.clone()).unwrap_or(self.src.len())
        )
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

    pub fn parse<'a>(&'a self, lang: &'a Language, cursor: &mut Cursor) -> Option<Node<'a>> {
        let mut index = 0;
        let mut save = cursor.save();
        let mut subs: Vec<Node> = vec![];

        while self.steps[index].rules().iter().any(|(rule, i)| {
            if let Some(node) = rule.parse(lang, cursor) {
                subs.push(node);
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
            return None;
        }

        if success {
            return Some(Node::<'a> {
                span: cursor.get_span(&mut save),
                kind: &self,
                subs,
            });
        } else {
            return None
        } 
    }
}

