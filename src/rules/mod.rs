//! A collection of rules to make a language.

// Child modules
mod automata; 
mod symbol;
mod lexer;

// Publish
pub use automata::*;
pub use symbol::*;
pub use lexer::*;

use std::rc::Rc;
use crate::document::*;

/// A rule for parsing.
pub trait Rule {
    /// Parse the rule.
    fn parse<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<(Kind, Vec<Rc<Node<'a>>>)>;
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

