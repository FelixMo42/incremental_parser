use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::document::{Kind, Node, Parser};
use crate::rules::Rule;

use super::Step;

pub struct Lexer {
    pub steps: Vec<Step<RangeInclusive<char>>>
}

impl Lexer {
    pub fn new(steps: Vec<Step<RangeInclusive<char>>>) -> Box<dyn Rule> {
        return Box::new(Lexer { steps });
    }
}

impl Rule for Lexer {
    fn parse<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<(Kind, Vec<Rc<Node<'a>>>)> {
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(range, i)| {
            if parser.next_if(|chr| range.contains(chr)) {
                step = *i;

                true
            } else {
                false
            }
        }) {}

        if let Some(kind) = self.steps[step].kind() {
            return Some((kind, vec![]));
        } else {
            return None;
        }
    }
}

