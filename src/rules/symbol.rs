use std::{ops::RangeInclusive, rc::Rc};
use crate::document::{Node, Parser};
use tblit::RGB;
use super::{Rule, Step};

/// A simpled definite finite automata rule for creating base lexing.
pub struct Symbol {
    /// A base symbol must have a color, as it never has any children.
    color: RGB,

    /// The Steps in the dfa.
    steps: Vec<Step<RangeInclusive<char>>>,
}

impl Symbol {
    /// Initializes a new Symbol.
    pub fn new(color: RGB, steps: Vec<Step<RangeInclusive<char>>>) -> Box<dyn Rule> {
        return Box::new(Symbol { color, steps });
    }
}

impl Rule for Symbol {
    fn parse<'a>(&self, cursor: &mut Parser<'a, '_>) -> Option<Vec<Rc<Node<'a>>>> {
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(range, i)| {
            if cursor.next_if(|chr| range.contains(chr)) {
                step = *i;

                true
            } else {
                false
            }
        }) {}

        let success = self.steps[step].is_final();

        if success {
            return Some(vec![]);
        } else {
            return None;
        }
    }

    fn get_color(&self) -> Option<RGB> {
        return Some(self.color);
    }
}

