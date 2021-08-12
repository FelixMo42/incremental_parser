use crate::language::{Cursor, Language, Node};
use log::info;
use std::rc::Rc;

pub type Span = (usize, usize);

#[derive(PartialEq)]
struct Parens {
    innter: String,
}

pub struct Edit {
    pub span: Span,
    pub len: usize,
}

pub struct Step<'a, 'b> {
    node: &'b Rc<Node<'a>>,
    index: usize,
}

pub struct NodeIter<'a, 'b> {
    nodes: Vec<Step<'a, 'b>>,
}

impl<'a, 'b> Iterator for NodeIter<'a, 'b> {
    type Item = &'b Rc<Node<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(step) = self.nodes.last() {
            let node = &step.node.subs[step.index.clone()];

            if step.index < step.node.subs.len() && node.subs.len() > 0 {
                self.nodes.push(Step { node, index: 0 })
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

impl<'a, 'b> NodeIter<'a, 'b> {
    pub fn new(document: &'b Document<'a>) -> NodeIter<'a, 'b> {
        if document.root.subs.len() == 0 {
            return NodeIter { nodes: vec![] };
        }

        return NodeIter {
            nodes: vec![Step {
                node: &document.root,
                index: 0,
            }],
        };
    }

    pub fn peek(&self) -> Option<&'b Rc<Node<'a>>> {
        if let Some(step) = self.nodes.last() {
            return Some(&step.node.subs[step.index]);
        } else {
            return None;
        }
    }
}

pub struct Document<'a> {
    pub lang: &'a Language,
    pub root: Rc<Node<'a>>,
}

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

impl<'a> Document<'a> {
    pub fn new(language: &'a Language) -> Document<'a> {
        return Document {
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
    pub fn parse<'b>(&'b mut self, src: &str, edit: Edit) {
        // How much was removed?
        let removed = edit.span.1 - edit.span.0;

        info!("parsing");

        incrament_node(&mut self.root, removed, edit.len, edit.span.0);

        if src.len() == 0 {
            return;
        }

        let mut cursor = Cursor::new(self, edit, src);
        self.root = cursor.parse(&0).unwrap();
    }

    pub fn node_iter<'b>(&'b self) -> NodeIter<'a, 'b> {
        return NodeIter::new(self);
    }
}
