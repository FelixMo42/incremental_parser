use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};
use tblit::screen::Color;

pub type Cursor<'a> = Peekable<Skip<CharIndices<'a>>>;

#[derive(PartialEq, Eq)]
pub struct Step (pub Vec<(RangeInclusive<char>, usize)>, pub bool);

impl Step {
    #[inline]
    pub fn rules(&self) -> &Vec<(RangeInclusive<char>, usize)> {
        return &self.0;
    }

    #[inline]
    pub fn done(&self) -> bool {
        return self.1;
    }
}

pub struct Language {
    pub rules: Vec<Token>
}

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
}

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

    pub fn parse(&self, cursor: &mut Cursor) -> bool {
        let mut index = 0;

        'main: loop {
            let step = &self.steps[index];
            
            if let Some((_, chr)) = cursor.peek() {
                for (rule, i) in step.rules() {
                    if rule.contains(chr) {
                       index = i.clone();

                       cursor.next();

                       continue 'main;
                    }
                }
            }

            if step.done() {
                return true;
            }
            
            return false;
        }
    }
}

