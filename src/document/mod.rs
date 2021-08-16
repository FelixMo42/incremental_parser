//! A code source file.

// Child modules
mod cursor;
mod parser;
mod nodeiter;

// Publish
pub use cursor::Cursor;
pub use parser::Parser;
pub use nodeiter::NodeIter;

use std::rc::Rc;
use crate::rules::{Language, Rule};

/// A span of the document in bytes.
pub type Span = (usize, usize);

/// A node in the document.
pub struct Node<'a> {
    /// The span of the node in the document.
    pub span: (usize, usize),

    /// What rule created the node.
    pub rule: &'a Box<dyn Rule>,

    /// The sub value of the nodes.
    pub subs: Vec<Rc<Node<'a>>>,
}



/// Updates the span of all the nodes when the document is changed.
fn incrament_node(node: &mut Rc<Node>, removed: usize, added: usize, start: usize) {
    let node = Rc::get_mut(node).expect("extra copy of node exists!");

    if node.span.0 >= start {
        if node.span.0 > start {
            node.span.0 = node.span.0 - removed + added;
            node.span.1 = node.span.1 - removed + added;
        } else {
            node.span.1 = node.span.1 - removed + added;
        }
    }

    for child in &mut node.subs {
        incrament_node(child, removed, added, start);
    }
}

/// A code document.
pub struct Document<'a> {
    /// The language used for parsing the document.
    pub lang: &'a Language,

    /// The root Node of the document.
    pub root: Rc<Node<'a>>,

    /// The actual String of the Document
    pub text: Text,
}

impl<'a> Document<'a> {
    /// Initializes a new document of the given language.
    pub fn new(language: &'a Language) -> Document<'a> {
        return Document {
            text: Text("".to_string()),
            lang: language,
            root: Rc::new(Node {
                span: (0, 0),
                rule: &language[0],
                subs: vec![],
            }),
        };
    }
}

impl<'a> Document<'a> {
    /// Reparse the document.
    fn parse<'b>(&'b mut self, span: Span, edit_len: usize) {
        // How much was removed?
        let removed = span.1 - span.0;

        incrament_node(&mut self.root, removed, edit_len, span.0);

        if self.text.byte_len() == 0 {
            return;
        }

        self.root = parser::parse(self, (span.0, span.0 + edit_len));
    }
}

impl<'a> Document<'a> {
    /// Replace the given span with the edit.
    pub fn edit(&mut self, span: Span, edit: &str) {
        self.text.edit(span, edit);
        self.parse(span, edit.len());
    }

    /// Iterate throught all the nodes in the document.
    pub fn node_iter<'b>(&'b self) -> NodeIter<'a, 'b> {
        return NodeIter::new(self);
    }
}

/// The Source for a document.
pub struct Text(String);

impl Text {
    /// Replace the given span of text with the edit.
    pub fn edit(&mut self, span: Span, edit: &str) {
        self.0.replace_range(span.0..span.1, edit);
    }

    /// Read the character at a given byte offset.
    pub fn read(&self, offset: usize) -> Option<char> {
        self.0[offset..].chars().nth(0)
    }

    /// Cheacks if the character at the given byte offset is a newline.
    pub fn is_newline(&self, offset: usize) -> bool {
        self.read(offset).map_or(true, |chr| chr == '\n')
    }

    /// Get the byte length of the text.
    pub fn byte_len(&self) -> usize {
        self.0.as_bytes().len()
    }
}

