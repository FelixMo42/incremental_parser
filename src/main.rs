#![warn(missing_docs)]

//! An incramental parser, and a simple terminal ide for testing it.

// Publish the children modules
pub mod document;
pub mod rules;

use crate::rules::*;
use crate::document::*;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use tblit::vec2::Vec2;
use tblit::event::{Event, Key};
use tblit::screen::{Color, Screen};

/// Just the color off white.
pub const WHITE: Color = Color(200, 200, 200);

fn make_language() -> Vec<Box<dyn Rule>> {
    let word = Symbol::new(WHITE, vec![
        Step(vec![
            (('a'..='z'), 1),
            (('A'..='Z'), 1),
            (('_'..='_'), 1),
            (('\''..='\''), 1),
        ], true),
        Step(vec![
            (('a'..='z'), 1),
            (('A'..='Z'), 1),
            (('_'..='_'), 1),
            (('0'..='9'), 1),
            (('\''..='\''), 1),
        ], true),
    ]);

    let whitespace = Symbol::new(WHITE, vec![
        Step(vec![
             (('\t'..=' '), 0)
        ], true)
    ]);

    let punctuation = Symbol::new(Color(186, 108, 72), vec![
        Step(vec![
             (('!'..='/'), 0),
             ((':'..='@'), 0)
        ], true)
    ]);

    let number = Symbol::new(Color(1, 110, 115), vec![
        Step(vec![
             (('0'..='9'), 0),
             (('.'..='.'), 1)
        ], true),
        Step(vec![
             (('0'..='9'), 1)
        ], true),
    ]);

    let file = Automata::new(None, vec![
        Step(vec![
             (1, 0),
             (2, 0),
             (3, 0),
             (4, 0)
        ], true)
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

fn out(screen: &mut Screen, doc: &Document, node: &Rc<Node>, cord: &mut Vec2<usize>) {
    if let Some(color) = node.rule.get_color() {
        for i in node.span.0..node.span.1 {
            if let Some(chr) = doc.text.read(i) {
                if chr != '\n' {
                    screen.set(chr, color, &Vec2::new(cord.x, cord.y));
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

    let mut screen = Screen::new();
    let mut cursor = Cursor::new();

    screen.show_cursor();

    for event in screen.events() {
        match event.unwrap() {
            // input new text
            Event::Key(Key::Char('\t')) => cursor.write(&mut document, "   "),
            Event::Key(Key::Char(chr))  => cursor.write(&mut document, chr.to_string().as_str()),

            // delete text
            Event::Key(Key::Backspace) => {
                cursor.delete(&mut document);
                screen.set(' ', WHITE, &cursor.position);
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
        screen.out.flush().unwrap();
    }
}

