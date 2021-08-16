use crate::document::{Document, NodeIter, Span, Node};
use crate::rules::Rule;
use std::rc::Rc;

/// Return a new node tree for the given document.
pub fn parse<'a, 'b>(document: &'b Document<'a>, edit: Span) -> Rc<Node<'a>> {
    let mut parser = Parser {
        edit, document,
        node: document.node_iter(),
        offset: 0,
    };

    return parser.parse(0).unwrap();
}

/// Updates the parse tree for a document.
pub struct Parser<'a, 'b> {
    /// The current position in the text in bytes.
    offset: usize,

    /// The document we want to update.
    document: &'b Document<'a>,

    /// Span covering the edits that were made to the document.
    edit: Span,

    /// An iterator of nodes to find memorized nodes.
    node: NodeIter<'a, 'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn get_node(&mut self, rule: &'a Box<dyn Rule>, index: usize) -> Option<Rc<Node<'a>>> {
        while let Some(node) = self.node.peek() {
            if node.span.0 < index {
                self.node.next();
                continue;
            }

            let right_index = node.span.0 == index;
            let right_rule = node.rule == rule;
            let unedited = self.edit.1 < node.span.0 || self.edit.0 > node.span.1;

            return if right_index && right_rule && unedited {
                Some(node.clone())
            } else {
                None
            };
        }

        return None;
    }
}

impl<'a> Parser<'a, '_> {
    /// Eats a character if it matches the given func.
    pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> bool {
        if let Some(chr) = self.document.text.read(self.offset) {
            if func(&chr) {
                self.offset += 1;
                return true;
            }
        }

        return false;
    }

    /// Eat as long as a Rule matches.
    pub fn parse(&mut self, rule_index: usize) -> Option<Rc<Node<'a>>> {
        let rule = &self.document.lang[rule_index];

        // Keep a copy of the current offset in the document.
        let offset = self.offset;

        // Check to see if we have this one memorized.
        if let Some(node) = self.get_node(rule, offset) {
            // If we do have one, then skip the cursor past it.
            self.offset = node.span.1;

            // Then return the old node.
            return Some(node.clone());
        }

        if let Some((kind, subs)) = rule.parse(self) {
            // If we havent advanced at all, then this is a failure 
            if self.offset == offset {
                return None;
            }

            // Succses! Make a new node and return it. 
            return Some(Rc::new(Node {
                span: (offset, self.offset),
                subs, kind, rule,
            }));
        }

        // We have failed :(. Return the offset to the original value.
        self.offset = offset;

        return None;
    }
}

