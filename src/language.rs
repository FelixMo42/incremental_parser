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
                cursor.next_if(|(i, chr)| range.contains(chr)).is_some()
            },
            _ => false  
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
    pub fn done(&self) -> bool {
        return self.1;
    }
}

/* Cursor */

pub type Language = Vec<Token>;

pub type Cursor<'a> = Peekable<Skip<CharIndices<'a>>>;

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

        'main: loop {
            let step = &self.steps[index];
            
            for (rule, i) in step.rules() {
                if rule.parse(lang, cursor) {
                    index = i.clone();
                    
                    continue 'main;
                }
            }

            if step.done() {
                return true;
            }
            
            return false;
        }
    }
}

