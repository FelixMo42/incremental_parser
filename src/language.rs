use std::{iter::Peekable, ops::RangeInclusive, rc::Rc, str::CharIndices};
use tblit::screen::Color;

use crate::document::{Document, Edit};

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
    pub chars: CursorIter<'b>,

    pub lang: &'a Language,

    pub edit: Edit,
    pub node: Rc<Node<'a>>,
    pub sub: usize,
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn new(doc: &Document<'a>, edit: Edit, src: &'b str) -> Cursor<'a, 'b> {
        return Cursor {
            src,
            lang: doc.lang,
            node: doc.root.clone(),
            edit,
            sub: 0,
            chars: src.char_indices().peekable()
        };
    }
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn get_node(&mut self, token: &'a Token, index: usize) -> Option<Rc<Node<'a>>> {
        while self.sub < self.node.subs.len() {
            let child = &self.node.subs[self.sub];

            if child.span.0 == index && child.kind == token {
                if child.span.1 < self.edit.span.0 || child.span.0 > self.edit.span.0 + self.edit.len {
                    return Some(child.clone());
                }
            }

            if self.node.subs[self.sub].span.0 >= index {
                break
            }

            self.sub += 1;
        }

        return None;
    }

    pub fn set_node(&mut self, node: Rc<Node<'a>>) {
        self.node = node;
        self.sub = 0;
    }
}

impl<'a, 'b> Cursor<'a, 'b> {
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
        while self.chars.next_if(|(i, _)| i < &node.span.1).is_some() {}
    }
} 

/* Node */

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
    pub subs: Vec<Rc<Node<'a>>>
}

pub fn parse<'a>(kind: &'a Token, cursor: &mut Cursor<'a, '_>) -> Option<Rc<Node<'a>>> {
    let mut subs = vec![];
    let mut save = cursor.save();
    let mut step = 0;

    while kind.steps[step].rules().iter().any(|(rule, i)| {
        let success = match rule {
            Rule::Char(range) => {
                cursor.chars.next_if(|(_, chr)| range.contains(chr)).is_some()
            },
            Rule::Token(token_id) => {
                let token = &cursor.lang[*token_id];

                let index = cursor.get_index();
                if let Some(node) = cursor.get_node(token, index) {
                    cursor.skip(&node);
                    subs.push(node);
                    true
                } else if let Some(node) = parse(token, cursor) {
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
        kind.steps[step].is_final() &&
        cursor.advanced_from(&mut save);

    if success {
        return Some(Rc::new(Node::<'a> {
            span: cursor.get_span(&mut save),
            kind, subs
        }));
    } else {
        cursor.restore(save);
        return None;
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
