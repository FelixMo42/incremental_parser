use crate::language::{Cursor, Language, Node};

pub type Span = (usize, usize);

pub struct Edit {
    pub span: Span,
    pub len: usize
}

pub struct Document<'a> {
    pub lang: &'a Language,
    pub node: Node<'a>
}

fn incrament_node(node: &mut Node, removed: usize, added: usize, start: usize) {
    if node.span.0 >= start {
        if node.span.0 > added {
            node.span.0 = node.span.0 - removed + added;
            node.span.1 = node.span.1 - removed + added;
        } else {
            node.span.1 = node.span.1 - removed + added;
        }

        for child in &mut node.subs {
            incrament_node(child, removed, added, start);
        }
    }
}

impl<'a> Document<'a> {
    pub fn new(language: &'a Language) -> Document<'a> {
        return Document {
            lang: language,
            node: Node {
                span: (0, 0),
                kind: &language[0],
                subs: vec! []
            }
        };
    }
}

impl<'a> Document<'a> {
    pub fn parse(&mut self, src: &str, edit: Edit) {
        // What is the index of the first node that could have been edited.

        // How much was removed? 
        let removed = edit.span.1 - edit.span.0;

        incrament_node(&mut self.node, removed, edit.len, edit.span.0);

        if src.len() == 0 {
            return
        }

        let mut cursor = Cursor::new(self, src);
        self.node.parse(&mut cursor);
    }
}

