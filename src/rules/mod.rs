//! A collection of rules to make a language.

// Child modules
mod symbol;
mod automata; 

// Publish
pub use symbol::*;
pub use automata::*;

use std::rc::Rc;
use tblit::screen::Color;
use crate::document::{Parser, Node};

/// A rule for parsing.
pub trait Rule {
    /// Parse the rule.
    fn parse<'a>(&self, cursor: &mut Parser<'a, '_>) -> Option<Vec<Rc<Node<'a>>>>;

    /// The color that this node should be displayed as.
    /// If None, then it will display each of the child nodes in their colors.
    fn get_color(&self) -> Option<Color>;
}

impl PartialEq for dyn Rule {
    fn eq(&self, outher: &dyn Rule) -> bool {
        // get the raw pointers
        let a = self as *const _;
        let b = outher as *const _;

        // see if the two point at the same thing
        return a == b;
    }
}

/// A programming language is just a list of Rules. The first one should be the file rule.
pub type Language = Vec<Box<dyn Rule>>;

