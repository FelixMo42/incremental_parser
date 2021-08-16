use std::{ops::RangeInclusive, rc::Rc};
use crate::document::{Kind, Node, Parser};
use super::{Rule, Step};

/// A simpled definite finite automata rule for creating base lexing.
pub struct Symbol {
    /// The Steps in the dfa.
    steps: Vec<Step<RangeInclusive<char>>>,
}

impl Symbol {
    /// Initializes a new Symbol.
    pub fn new(steps: Vec<Step<RangeInclusive<char>>>) -> Box<dyn Rule> {
        return Box::new(Symbol { steps });
    }
}

impl Rule for Symbol {
    fn parse<'a>(&self, cursor: &mut Parser<'a, '_>) -> Option<(Kind, Vec<Rc<Node<'a>>>)> {
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(range, i)| {
            if cursor.next_if(|chr| range.contains(chr)) {
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

