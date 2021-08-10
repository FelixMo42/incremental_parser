use std::{iter::Peekable, rc::Rc, str::CharIndices};

use tblit::screen::Color;

use crate::document::{Document, Edit};

/* Rule */

pub trait Rule {
    fn parse<'a>(&self, cursor: &mut Cursor<'a, '_>) -> Option<Vec<Rc<Node<'a>>>>;
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

/* Node */

pub struct Node<'a> {
    pub span: (usize, usize),
    pub rule: &'a Box<dyn Rule>,
    pub subs: Vec<Rc<Node<'a>>>
}


/* Cursor */

pub type Language = Vec<Box<dyn Rule>>;

type CursorIter<'a> = Peekable<CharIndices<'a>>;

pub struct Cursor<'a, 'b> {
    pub src: &'b str,
    pub chars: CursorIter<'b>,

    pub lang: &'a Language,

    pub edit: Edit,
    pub node: Rc<Node<'a>>,
    pub sub: usize,
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn new(doc: &Document<'a>, edit: Edit, src: &'b str) -> Cursor<'a, 'b> {
        return Cursor {
            src,
            lang: doc.lang,
            node: doc.root.clone(),
            edit,
            sub: 0,
            chars: src.char_indices().peekable()
        };
    }
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn get_node(&self, rule: &'a Box<dyn Rule>, index: usize) -> Option<Rc<Node<'a>>> {
        while self.sub < self.node.subs.len() {
            let child = &self.node.subs[self.sub];

            if child.span.0 == index && child.rule == rule {
                if child.span.1 < self.edit.span.0 || child.span.0 > self.edit.span.0 + self.edit.len {
                    return Some(child.clone());
                }
            }

            if self.node.subs[self.sub].span.0 >= index {
                break
            }

            self.sub += 1;
        }

        return None;
    }

    pub fn set_node(&mut self, node: Rc<Node<'a>>) {
        self.node = node;
        self.sub = 0;
    }
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn done(&mut self) -> bool {
        return self.chars.peek().is_none();
    }

    pub fn save(&self) -> CursorIter<'b> {
        return self.chars.clone();
    }

    pub fn restore(&mut self, save: CursorIter<'b>) {
        self.chars = save;
    }

    pub fn advanced_from(&mut self, save: &mut CursorIter<'b>) -> bool {
        self.chars.peek() != save.peek()
    }

    pub fn get_span(&mut self, save: &mut CursorIter<'b>) -> (usize, usize) {
        return (
            save.peek().unwrap().0,
            self.get_index()
        )
    }

    pub fn get_index(&mut self) -> usize {
        self.chars.peek().map(|(i, _)| i.clone()).unwrap_or(self.src.len())
    }

    pub fn skip(&mut self, node: &Rc<Node>) {
        while self.chars.next_if(|(i, _)| i < &node.span.1).is_some() {}
    }
} 

impl<'a> Cursor<'a, '_> {
    pub fn parse(&mut self, rule: &'a Box<dyn Rule>) -> Option<Rc<Node<'a>>> {
        let index = self.get_index();

        // Check to see if we have this one memorized.
        if let Some(node) = self.get_node(rule, index) {
            // If we do have one, then skip the cursor past it.
            self.skip(&node);

            // Then return the old node.
            return Some(node);
        }

        let save = self.save();

        if let Some(subs) = rule.parse(self) {
            return Some(Rc::new(Node {
                span: self.get_span(&mut save),
                subs,
                rule,
            }));
        }

        self.restore(save);
        
        return None;
    }
}

/* Step */

pub struct Step (pub Vec<(Box<dyn Rule>, usize)>, pub bool);

impl Step {
    #[inline]
    pub fn rules(&self) -> &Vec<(Box<dyn Rule>, usize)> {
        return &self.0;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        return self.1;
    }
}


/* Token */

pub struct Token {
    steps: Vec<Step>
}

impl Token {
    pub fn new(color: Option<Color>, steps: Vec<Step>) -> Token {
        return Token {
            steps
        };
    }
}

impl Rule for Token {
    fn parse<'a>(&self, cursor: &mut Cursor<'a, '_>) -> Option<Vec<Rc<Node<'a>>>> {
        let mut subs = vec![];
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(rule, i)| {
            if let Some(node) = rule.parse(cursor) {
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
}
