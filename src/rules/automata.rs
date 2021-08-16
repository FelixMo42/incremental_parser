use std::rc::Rc;
use tblit::RGB;
use crate::document::{Node, Parser};
use super::Rule;

/// A step in the automata.
pub struct Step<T>(pub Vec<(T, usize)>, pub bool);

impl<T> Step<T> {
    /// The edges of this step.
    pub fn rules(&self) -> &Vec<(T, usize)> {
        return &self.0;
    }

    /// Is the Step a possible end of the node?
    pub fn is_final(&self) -> bool {
        return self.1;
    }
}

/// A recusice definite finite automata rule.
pub struct Automata {
    /// A base symbol must have a color, as it never has any children.
    color: Option<RGB>,
    
    /// The Steps in the dfa.
    steps: Vec<Step<usize>>,
}

impl Automata {
    /// Constructor for the automata.
    pub fn new(color: Option<RGB>, steps: Vec<Step<usize>>) -> Box<dyn Rule> {
        return Box::new(Automata { color, steps });
    }
}

impl Rule for Automata {
    fn parse<'a>(&self, cursor: &mut Parser<'a, '_>) -> Option<Vec<Rc<Node<'a>>>> {
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

        let success = self.steps[step].is_final();

        if success {
            return Some(subs);
        } else {
            return None;
        }
    }

    fn get_color(&self) -> Option<RGB> {
        return self.color.clone();
    }
}

