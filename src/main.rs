pub mod document;
pub mod language;

use crate::document::Document;
use crate::language::{Node, Rule, Step, Token};
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::io::Write;
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

const WHITE: Color = Color(200, 200, 200);

fn make_language() -> Vec<Box<dyn Rule>> {
    let word = Symbol::new(
        Some(WHITE),
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
        Some(WHITE),
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

struct Cursor<'a> {
    position: Vec2<usize>,
    document: Document<'a>,
    sticky_x: usize,
    index: usize,
}

impl<'a> Cursor<'a> {
    fn write(&mut self, input: &str) {
        self.document.edit((self.index, self.index), input);
        self.position.x += input.len(); 
        self.index += input.len();
    }

    fn new_line(&mut self) {
        self.document.edit((self.index, self.index), "\n");
        self.position.y += 1;
        self.position.x = 0;
        self.index += 1;
    }

    fn delete(&mut self) {
        if self.index == 0 {
            return
        }

        self.document.edit((self.index - 1, self.index), "");

        self.prev_char();
    }
}

impl<'a> Cursor<'a> {
    fn next(&mut self) {
        if let Some(chr) = self.document.read(self.index) {
            if chr == '\n' {
                self.position.y += 1;
                self.position.x = 0;
            } else {
                self.position.x += 1;
            }

            self.index += 1;
        }
    }

    fn prev(&mut self) {
        if self.index == 0 {
            return
        }

        self.index -= 1;

        if self.position.x == 0 {
            self.position.y -= 1;
            self.position.x = 0;
            
            if self.position.y == 0 {
                self.position.x = self.index;
                return
            }
            
            while self.is_newline(self.index - self.position.x - 1) {
                self.position.x += 1;
            }
        } else {
            self.position.x -= 1;
        }
    }

    fn next_char(&mut self) {
        self.next();
        self.sticky_x = self.position.x;
    }

    fn prev_char(&mut self) {
        self.prev();
        self.sticky_x = self.position.x;
    }

    fn next_line(&mut self) {
        // move to next line
        self.next();
        while self.position.x != 0 && self.index < self.document.text.len() {
            self.next();
        }

        // move back to the previous x value, or to the end of line
        while self.sticky_x != self.position.x && self.is_newline(self.index) {
            self.next();
        }
    }

    fn prev_line(&mut self) {
        // we cant go to previous line, if were at the first line
        if self.position.y == 0 {
            return
        }

        // move down to previous line
        self.prev();
        while self.is_newline(self.index) {
            self.prev()
        }

        // move down to correct position in line
        while self.position.x > self.sticky_x {
            self.prev()
        }
    }

    fn is_newline(&mut self, index: usize) -> bool {
        return self.document.read(index).map_or(false, |c| c != '\n');
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
    let document = Document::new(&language);

    let mut screen = Screen::new();

    let mut cursor = Cursor {
        position: Vec2::new(0, 0),
        sticky_x: 0,
        index: 0,
        document,
    };

    screen.show_cursor();

    for event in screen.events() {
        match event.unwrap() {
            // input new text
            Event::Key(Key::Char('\n')) => cursor.new_line(),
            Event::Key(Key::Char('\t')) => cursor.write("   "),
            Event::Key(Key::Char(chr)) => cursor.write(chr.to_string().as_str()),

            // delete text
            Event::Key(Key::Backspace) => {
                cursor.delete();
                screen.set(' ', WHITE, &cursor.position);
            },

            // move the cursor
            Event::Key(Key::Left)  => cursor.prev_char(),
            Event::Key(Key::Right) => cursor.next_char(),
            Event::Key(Key::Up)    => cursor.prev_line(),
            Event::Key(Key::Down)  => cursor.next_line(),

            // quit if unexpected input
            _ => break,
        }

        out(
            &mut screen,
            &cursor.document.root,
            &mut cursor.document.text.chars(),
            &mut Vec2::new(0, 0),
        );

        screen.blit();

        screen.move_cursor(&cursor.position);
        screen.out.flush().unwrap();
    }
}

