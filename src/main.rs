#![warn(missing_docs)]

//! An incramental parser, and a simple terminal ide for testing it.

// Publish the children modules
pub mod document;
pub mod rules;

use crate::rules::*;
use crate::document::*;

use simplelog::{Config, WriteLogger};

use std::fs::File;
use std::rc::Rc;

use tblit::*;
use tblit::event::*;

/// Gray
pub const WHITE: RGB = RGB(153, 174, 168);

/// Black
pub const GRAY: RGB = RGB(10, 10, 10);

/// Blue
pub const BLUE: RGB = RGB(0, 110, 114);

/// Orange
pub const ORANGE: RGB = RGB(186, 107, 71);

fn make_language() -> Vec<Box<dyn Rule>> {
    let word = Symbol::new(vec![
        Step(vec![
            (('a'..='z'), 1),
            (('A'..='Z'), 1),
            (('_'..='_'), 1),
            (('\''..='\''), 1),
        ], Some(Kind::Name)),
        Step(vec![
            (('a'..='z'), 1),
            (('A'..='Z'), 1),
            (('_'..='_'), 1),
            (('0'..='9'), 1),
            (('\''..='\''), 1),
        ], Some(Kind::Name)),
    ]);

    let whitespace = Symbol::new(vec![
        Step(vec![
             (('\t'..=' '), 0)
        ], Some(Kind::Whitespace))
    ]);

    let punctuation = Symbol::new(vec![
        Step(vec![
             (('!'..='/'), 0),
             ((':'..='@'), 0),
             (('{'..='~'), 0),
        ], Some(Kind::Punctuation))
    ]);

    let number = Symbol::new(vec![
        Step(vec![
             (('0'..='9'), 0),
             (('.'..='.'), 1)
        ], Some(Kind::Number)),
        Step(vec![
             (('0'..='9'), 1)
        ], Some(Kind::Number)),
    ]);

    let file = Automata::new(vec![
        Step(vec![
             (1, 0),
             (2, 0),
             (3, 0),
             (4, 0),
        ], Some(Kind::File))
    ]);

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

fn color(kind: Kind) -> Option<RGB> {
    match kind {
        Kind::File => None,
        Kind::Whitespace => Some(WHITE),
        Kind::Name => Some(WHITE),
        Kind::Number => Some(BLUE),
        Kind::Punctuation => Some(ORANGE),
    }
}

fn out(screen: &mut Screen<Color>, doc: &Document, node: &Rc<Node>, cord: &mut Vec2<usize>) {
    if let Some(color) = color(node.kind) {
        for i in node.span.0..node.span.1 {
            if let Some(chr) = doc.text.read(i) {
                if chr != '\n' {
                    screen.set(&cord, chr, Color {
                        fg: color,
                        bg: GRAY,
                    });

                    cord.x += 1;
                } else {
                    cord.y += 1;
                    cord.x = 0;
                }
            }
        }
    } else {
        for node in node.subs.iter() {
            out(screen, doc, node, cord);
        }
    }
}

fn main() {
    WriteLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        File::create("lang.log").unwrap(),
    ).unwrap();

    let language = make_language();
    let mut document = Document::new(&language);

    let mut cursor = Cursor::new();

    let mut screen = Screen::new(Color {
        fg: WHITE,
        bg: GRAY,
    });

    screen.blit();
    screen.show_cursor();
    screen.move_cursor(&cursor.position.into());

    for event in screen.events() {
        match event.unwrap() {
            // input new text
            Event::Key(Key::Char('\t')) => cursor.write(&mut document, "   "),
            Event::Key(Key::Char(chr))  => cursor.write(&mut document, chr.to_string().as_str()),

            // delete text
            Event::Key(Key::Backspace) => {
                cursor.delete(&mut document);
            },

            // move the cursor
            Event::Key(Key::Left)  => cursor.prev_char(&document),
            Event::Key(Key::Right) => cursor.next_char(&document),
            Event::Key(Key::Up)    => cursor.prev_line(&document),
            Event::Key(Key::Down)  => cursor.next_line(&document),

            // quit if unexpected input
            _ => break,
        }

        out(
            &mut screen,
            &document,
            &document.root,
            &mut Vec2::new(0, 0),
        );

        screen.blit();
        screen.move_cursor(&cursor.position.into());
    }
}

