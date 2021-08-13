use crate::document::{Document, Edit, NodeIter, Span};
use std::{iter::Peekable, rc::Rc, str::CharIndices};
use tblit::screen::Color;

/* Rule */

pub trait Rule {
    fn parse<'a>(&self, cursor: &mut Cursor<'a, '_>) -> Option<Vec<Rc<Node<'a>>>>;

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

/* Node */

pub struct Node<'a> {
    pub span: (usize, usize),
    pub rule: &'a Box<dyn Rule>,
    pub subs: Vec<Rc<Node<'a>>>,
}

/* Cursor */

pub type Language = Vec<Box<dyn Rule>>;

type CursorIter<'a> = Peekable<CharIndices<'a>>;

pub struct Cursor<'a, 'b> {
    pub chars: CursorIter<'b>,

    pub text: &'b String,
    pub lang: &'a Language,

    pub edit: Span,
    pub node: NodeIter<'a, 'b>,
}

impl<'a, 'b> Cursor<'a, 'b> {
    pub fn new(doc: &'b Document<'a>, edit: Span) -> Cursor<'a, 'b> {
        return Cursor {
            edit,
            text: &doc.text,
            lang: doc.lang,
            node: doc.node_iter(),
            chars: doc.text.char_indices().peekable(),
        };
    }
}

impl<'a, 'b> Cursor<'a, 'b> {
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

impl<'a, 'b> Cursor<'a, 'b> {
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
        return (save.peek().unwrap().0, self.get_index());
    }

    pub fn get_index(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(i, _)| i.clone())
            .unwrap_or(self.text.len())
    }

    pub fn skip(&mut self, node: &Rc<Node>) {
        while self.chars.next_if(|(i, _)| i < &node.span.1).is_some() {}
    }
}

impl<'a> Cursor<'a, '_> {
    pub fn parse(&mut self, rule_index: &usize) -> Option<Rc<Node<'a>>> {
        // Get the rule
        let rule = &self.lang[*rule_index];

        // Get the current index
        let index = self.get_index();

        // Check to see if we have this one memorized.
        if let Some(node) = self.get_node(rule, index) {
            // If we do have one, then skip the cursor past it.
            self.skip(&node);

            // Then return the old node.
            return Some(node.clone());
        }

        let mut save = self.save();

        if let Some(subs) = rule.parse(self) {
            // 
            if !self.advanced_from(&mut save) {
                return None;
            }

            // 
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

pub struct Step<T>(pub Vec<(T, usize)>, pub bool);

impl<T> Step<T> {
    pub fn rules(&self) -> &Vec<(T, usize)> {
        return &self.0;
    }

    pub fn is_final(&self) -> bool {
        return self.1;
    }
}

/* Token */

pub struct Token {
    color: Option<Color>,
    steps: Vec<Step<usize>>,
}

impl Token {
    pub fn new(color: Option<Color>, steps: Vec<Step<usize>>) -> Box<dyn Rule> {
        return Box::new(Token { color, steps });
    }
}

impl Rule for Token {
    fn parse<'a>(&self, cursor: &mut Cursor<'a, '_>) -> Option<Vec<Rc<Node<'a>>>> {
        let mut subs = vec![];
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(rule, i)| {
            if let Some(node) = cursor.parse(rule) {
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

    fn get_color(&self) -> Option<Color> {
        return self.color.clone();
    }
}
