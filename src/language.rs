use std::{iter::Peekable, ops::RangeInclusive, str::CharIndices};
use tblit::screen::Color;

use crate::document::Document;


/* Rule */

#[derive(PartialEq, Eq)]
pub enum Rule {
    Char(RangeInclusive<char>),
    Token(usize)
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

type CursorIter<'a> = Peekable<CharIndices<'a>>;

pub struct Cursor<'a, 'b> {
    pub src: &'b str,
    pub lang: &'a Language,
    pub chars: CursorIter<'b> 
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn new(doc: &Document<'a>, src: &'b str) -> Cursor<'a, 'b> {
        return Cursor {
            src,
            lang: doc.lang,
            chars: src.char_indices().peekable()
        };
    }

    pub fn done(&mut self) -> bool {
        return self.chars.peek().is_none();
    }

    pub fn save(&self) -> CursorIter<'b> {
        return self.chars.clone();
    }

    pub fn restore(&mut self, save: CursorIter<'b>) {
        self.chars = save;
    }

    pub fn advanced_from(&mut self, save: &mut CursorIter<'b>) -> bool {
        self.chars.peek() != save.peek()
    }

    pub fn get_span(&mut self, save: &mut CursorIter<'b>) -> (usize, usize) {
        return (
            save.peek().unwrap().0,
            self.get_index()
        )
    }

    pub fn get_index(&mut self) -> usize {
        self.chars.peek().map(|(i, _)| i.clone()).unwrap_or(self.src.len())
    }

    pub fn skip(&mut self, node: &Node) {
        while let Some(_) = self.chars.next_if(|(i, _)| i < &node.span.1) {}
    }
} 

/* Node */

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
    pub subs: Vec<Node<'a>>
}

impl<'a> Node<'a> {
    pub fn parse(&mut self, cursor: &mut Cursor<'a, '_>) -> bool {
        let mut subs = vec![];
        let mut save = cursor.save();
        let mut step = 0;
        let mut index = 0;

        while self.kind.steps[step].rules().iter().any(|(rule, i)| {
            let success = match rule {
                Rule::Char(range) => {
                    cursor.chars.next_if(|(_, chr)| range.contains(chr)).is_some()
                },
                Rule::Token(token_id) => {
                    let token = &cursor.lang[*token_id];

                    let mut node = Node {
                        span: (0, 0),
                        kind: token,
                        subs: vec![]
                    };

                    if node.parse(cursor) {
                        subs.push(node);
                        true
                    } else {
                        false
                    }
                }
            };

            if success {
                step = *i;
            }

            return success;
        }) {}

        let success =
            self.kind.steps[step].is_final() &&
            cursor.advanced_from(&mut save);

        if success {
            self.span = cursor.get_span(&mut save);
            self.kind = self.kind;
            self.subs = subs;
        } else {
            cursor.restore(save);
        }

        return success;
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
}

