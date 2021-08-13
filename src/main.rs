pub mod document;
pub mod language;

use crate::document::{Document, Edit};
use crate::language::{Node, Rule, Step, Token};
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::str::Chars;
use tblit::event::{Event, Key};
use tblit::screen::{Color, Screen};
use tblit::vec2::Vec2;

//

struct Symbol {
    color: Option<Color>,
    steps: Vec<Step<RangeInclusive<char>>>,
}

impl Symbol {
    fn new(color: Option<Color>, steps: Vec<Step<RangeInclusive<char>>>) -> Box<dyn Rule> {
        return Box::new(Symbol { color, steps });
    }
}

impl Rule for Symbol {
    fn parse<'a>(&self, cursor: &mut language::Cursor<'a, '_>) -> Option<Vec<Rc<Node<'a>>>> {
        let mut step = 0;

        while self.steps[step].rules().iter().any(|(range, i)| {
            if let Some(_) = cursor.chars.next_if(|(_, chr)| range.contains(chr)) {
                step = *i;

                true
            } else {
                false
            }
        }) {}

        let success = self.steps[step].is_final();

        if success {
            return Some(vec![]);
        } else {
            return None;
        }
    }

    fn get_color(&self) -> Option<Color> {
        return self.color;
    }
}

fn make_language() -> Vec<Box<dyn Rule>> {
    let word = Symbol::new(
        Some(Color(200, 200, 200)),
        vec![
            Step(
                vec![
                    (('a'..='z'), 1),
                    (('A'..='Z'), 1),
                    (('_'..='_'), 1),
                    (('\''..='\''), 1),
                ],
                true,
            ),
            Step(
                vec![
                    (('a'..='z'), 1),
                    (('A'..='Z'), 1),
                    (('_'..='_'), 1),
                    (('0'..='9'), 1),
                    (('\''..='\''), 1),
                ],
                true,
            ),
        ],
    );

    let whitespace = Symbol::new(
        Some(Color(0, 0, 0)),
        vec![Step(vec![(('\t'..=' '), 0)], true)],
    );

    let punctuation = Symbol::new(
        Some(Color(186, 108, 72)),
        vec![Step(vec![(('!'..='/'), 0), ((':'..='@'), 0)], true)],
    );

    let number = Symbol::new(
        Some(Color(1, 110, 115)),
        vec![
            Step(vec![(('0'..='9'), 0), (('.'..='.'), 1)], true),
            Step(vec![(('0'..='9'), 1)], true),
        ],
    );

    let file = Token::new(None, vec![Step(vec![(1, 0), (2, 0), (3, 0), (4, 0)], true)]);

    return vec![
        // Root rule
        file,        // 0

        // Lexer bits
        whitespace,  // 1
        punctuation, // 2
        word,        // 3
        number,      // 4
    ];
}

fn out(screen: &mut Screen, node: &Rc<Node>, chars: &mut Chars, cord: &mut Vec2<usize>) {
    if let Some(color) = node.rule.get_color() {
        for _ in node.span.0..node.span.1 {
            let chr = chars.next().unwrap();

            if chr == '\n' {
                cord.y += 1;
                cord.x = 0;
            } else {
                screen.set(chr, color, cord);

                cord.x += 1;
            }
        }
    } else {
        for node in node.subs.iter() {
            out(screen, node, chars, cord);
        }
    }
}

fn main() {
    WriteLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        File::create("lang.log").unwrap(),
    )
    .unwrap();

    let language = make_language();
    let mut doc = Document::new(&language);

    let mut screen = Screen::new();
    let mut index = 0;

    for event in screen.events() {
        match event.unwrap() {
            Event::Key(Key::Char('\t')) => {
                doc.edit((index, index), "    ");
                index += 4;

            },
            Event::Key(Key::Char(chr)) => {
                doc.edit((index, index), chr.to_string().as_str());
                index += 1;
            }
            Event::Key(Key::Left) => {
                if index != 0 {
                    index -= 1;
                }
            }
            Event::Key(Key::Right) => {
                if index != doc.text.len() {
                    index += 1;
                }
            }
            Event::Key(Key::Backspace) => {
                if index != 0 {
                    doc.edit((index - 1, index), "");

                    screen.set(' ', Color(0, 0, 0), &Vec2::new(doc.text.len(), 0));

                    index -= 1;
                }
            }
            _ => break,
        }

        out(
            &mut screen,
            &doc.root,
            &mut doc.text.chars(),
            &mut Vec2::new(0, 0),
        );

        screen.blit();
    }
}
