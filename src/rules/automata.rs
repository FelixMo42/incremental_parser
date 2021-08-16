use std::rc::Rc;
use crate::document::{Kind, Node, Parser};
use super::Rule;

/// A step in the automata.
pub struct Step<T>(pub Vec<(T, usize)>, pub Option<Kind>);

impl<T> Step<T> {
    /// The edges of this step.
    pub fn rules(&self) -> &Vec<(T, usize)> {
        return &self.0;
    }

    /// Is the Step a possible end of the node?
    pub fn kind(&self) -> Option<Kind> {
        return self.1;
    }
}

/// A recusice definite finite automata rule.
pub struct Automata {
    /// The Steps in the dfa.
    steps: Vec<Step<usize>>,
}

impl Automata {
    /// Constructor for the automata.
    pub fn new(steps: Vec<Step<usize>>) -> Box<dyn Rule> {
        return Box::new(Automata { steps });
    }
}

impl Rule for Automata {
    fn parse<'a>(&self, cursor: &mut Parser<'a, '_>) -> Option<(Kind, Vec<Rc<Node<'a>>>)> {
        let mut subs = vec![];
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(rule, i)| {
            if let Some(node) = cursor.parse(*rule) {
                subs.push(node);

                step = *i;

                true
            } else {
                false
            }
        }) {}

        if let Some(kind) = self.steps[step].kind() {
            return Some((kind, subs));
        } else {
            return None;
        }
    }
}

