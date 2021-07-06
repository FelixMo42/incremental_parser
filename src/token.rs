use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};
use tblit::screen::Color;

pub type Cursor<'a> = Peekable<Skip<CharIndices<'a>>>;

#[derive(PartialEq, Eq)]
pub struct Node (pub Vec<(RangeInclusive<char>, usize)>, pub bool);

#[derive(PartialEq, Eq)]
pub struct Symbol<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
}

#[derive(PartialEq, Eq)]
pub struct Token {
    pub color: Color,
    pub nodes: Vec<Node>
}

impl Token {
    pub fn new(color: Color, nodes: Vec<Node>) -> Token {
        return Token {
            color,
            nodes
        }
    }

    pub fn parse(&self, cursor: &mut Cursor) -> bool {
        let mut index = 0;

        'main: loop {
            let node = &self.nodes[index];
            
            if let Some((_, chr)) = cursor.peek() {
                for (case, i) in &node.0 {
                    if case.contains(chr) {
                       index = i.clone();

                       cursor.next();

                       continue 'main;
                    }
                }
            }

            if node.1 {
                return true;
            }
            
            return false;
        }
    }
}

