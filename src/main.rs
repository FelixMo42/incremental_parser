//! An incramental parser, and a simple terminal ide for testing it.

// Publish the children modules
pub mod document;
pub mod rules;

use crate::rules::*;
use crate::document::*;

use simplelog::{Config, WriteLogger};

use std::fs::File;

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
    let lexer = Lexer::new(vec![
        Step(vec![
            // Whitespace
            (('\t'..=' '), 1),

            // Word
            (('a'..='z'), 2),
            (('A'..='Z'), 2),
            (('_'..='_'), 2),
            (('\''..='\''), 2),

            // Punctuation
            (('!'..='/'), 3),
            ((':'..='@'), 3),
            (('{'..='~'), 3),

            // Number
            (('0'..='9'), 4),
        ], None),

        // Whitespace
        Step(vec![
             (('\t'..=' '), 1)
        ], Some(Kind::Whitespace)),

        // Word
        Step(vec![
            (('a'..='z'), 2),
            (('A'..='Z'), 2),
            (('_'..='_'), 2),
            (('0'..='9'), 2),
            (('\''..='\''), 2),
        ], Some(Kind::Name)),

        // Punctuation
        Step(vec![
        ], Some(Kind::Punctuation)),

        // Number
        Step(vec![
             (('0'..='9'), 4),
             (('.'..='.'), 5)
        ], Some(Kind::Number)),
        Step(vec![
             (('0'..='9'), 5)
        ], Some(Kind::Number)),
    ]);

    let file = Automata::new(vec![
        Step(vec![
             ((2, Kind::EqualExpression), 0),
             ((2, Kind::Error), 0)
        ], Some(Kind::File))
    ]);

    let assign = Automata::new(vec![
        Step(vec![
             ((1, Kind::Name), 1),
        ], Some(Kind::Error)),
        Step(vec![
             ((1, Kind::Punctuation), 2),
        ], Some(Kind::Error)),
        Step(vec![
             ((1, Kind::Number), 3),
        ], Some(Kind::Error)),
        Step(vec![
        ], Some(Kind::EqualExpression)),
    ]);

    return vec![
        file,        // 0
        lexer,       // 1

        // Expressions
        assign,      // 2
    ];
}

fn color(doc: &Document, index: usize) -> Option<RGB> {
    doc.get_filter(index, |node| {
        match node.kind {
            Kind::EqualExpression |
            Kind::File => None,

            Kind::Whitespace  => Some(WHITE),
            Kind::Name        => Some(WHITE),
            Kind::Number      => Some(BLUE),
            Kind::Punctuation => Some(ORANGE),

            Kind::Error => Some(ORANGE),
        }
    })
}

fn out(screen: &mut Screen<Color>, doc: &Document, cord: &mut Vec2<usize>) {
    for (i, chr) in doc.text.chars_indices() {
        if let Some(color) = color(doc, i) {
            screen.set(&cord, chr, Color {
                fg: color,
                bg: GRAY,
            });
        }

        if chr == '\n' {
            cord.y += 1;
            cord.x = 0;
        } else {
            cord.x += 1;
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
            &mut Vec2::new(0, 0),
        );

        screen.blit();
        screen.move_cursor(&cursor.position.into());
    }
}

