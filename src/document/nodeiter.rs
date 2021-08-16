use crate::document::{Node, Document};
use std::rc::Rc;

/// A step along the iteration of the nodes in a document.
struct NodeIterStep<'a, 'b> {
    node: &'b Rc<Node<'a>>,
    index: usize,
}

/// An iterator of the nodes in a document.
pub struct NodeIter<'a, 'b> {
    nodes: Vec<NodeIterStep<'a, 'b>>,
}

impl<'a, 'b> NodeIter<'a, 'b> {
    /// Initializes a new node iter.
    pub fn new(document: &'b Document<'a>) -> NodeIter<'a, 'b> {
        if document.root.subs.len() == 0 {
            return NodeIter { nodes: vec![] };
        }

        return NodeIter {
            nodes: vec![NodeIterStep {
                node: &document.root,
                index: 0,
            }],
        };
    }

    /// Basically Peekable.
    pub fn peek(&self) -> Option<&'b Rc<Node<'a>>> {
        if let Some(step) = self.nodes.last() {
            return Some(&step.node.subs[step.index]);
        } else {
            return None;
        }
    }
}

impl<'a, 'b> Iterator for NodeIter<'a, 'b> {
    type Item = &'b Rc<Node<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(step) = self.nodes.last() {
            let node = &step.node.subs[step.index.clone()];

            if step.index < step.node.subs.len() && node.subs.len() > 0 {
                self.nodes.push(NodeIterStep { node, index: 0 })
            } else {
                while let Some(step) = self.nodes.last_mut() {
                    step.index += 1;

                    if step.index == step.node.subs.len() {
                        self.nodes.pop();
                    } else {
                        break;
                    }
                }
            }

            return Some(node);
        } else {
            return None;
        }
    }
}

