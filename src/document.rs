use std::rc::Rc;

use crate::language::{Cursor, Language, Node, parse};

pub type Span = (usize, usize);

pub struct Edit {
    pub span: Span,
    pub len: usize
}

pub struct Document<'a> {
    pub lang: &'a Language,
    pub root: Rc<Node<'a>>
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
                kind: &language[0],
                subs: vec! []
            })
        };
    }
}

impl<'a> Document<'a> {
    pub fn parse(&mut self, src: &str, edit: Edit) {
        // How much was removed? 
        let removed = edit.span.1 - edit.span.0;

        incrament_node(&mut self.root, removed, edit.len, edit.span.0);

        if src.len() == 0 {
            return
        }

        let mut cursor = Cursor::new(self, edit, src);
        self.root = parse(&self.lang[0], &mut cursor).unwrap();
    }
}
