pub mod language;
pub mod document;

use std::rc::Rc;
use std::str::Chars;

use tblit::event::{Event, Key};
use tblit::screen::{Screen, Color};
use tblit::vec2::Vec2;

use crate::language::{Node, Rule, Step, Token};
use crate::document::{Document, Edit};

fn toks() -> Vec<Token> {
    let word = Token::new(Some(Color(200, 200, 200)), vec![
        Step(vec![
            (Rule::Char('a'..='z'), 1),
            (Rule::Char('A'..='Z'), 1),
            (Rule::Char('_'..='_'), 1),
            (Rule::Char('\''..='\''), 1),
        ], true),

        Step(vec![
            (Rule::Char('a'..='z'), 0),
            (Rule::Char('A'..='Z'), 0),
            (Rule::Char('_'..='_'), 0),
            (Rule::Char('0'..='9'), 0),
            (Rule::Char('\''..='\''), 0),
        ], true),
    ]);

    let whitespace = Token::new(Some(Color(0, 0, 0)), vec![
        Step(vec![
            (Rule::Char('\t'..=' '), 0)
        ], true)
    ]);

    let punctuation = Token::new(Some(Color(186,108,72)), vec![
        Step(vec![
            (Rule::Char('!'..='/'), 0),
            (Rule::Char(':'..='@'), 0),
        ], true)
    ]);

    let number = Token::new(Some(Color(1,110,115)), vec![
        Step(vec![
            (Rule::Char('0'..='9'), 0),
            (Rule::Char('.'..='.'), 1),
        ], true),
        Step(vec![
            (Rule::Char('0'..='9'), 1),
        ], true)
    ]);

    let value = Token::new(None, vec![
        Step(vec![
             (Rule::Token(2), 0),
             (Rule::Token(3), 0)
        ], true)
    ]);

    let error = Token::new(Some(Color(200, 0, 0)), vec![
        Step(vec![
            (Rule::Char('\x00'..='~'), 1),
        ], false),
        Step(vec![
        ], true)
    ]);

    let file = Token::new(None, vec![
        Step(vec![
            (Rule::Token(5), 0),
        ], true)
    ]);

    return vec![
        file,        // 0

        whitespace,  // 1
        word,        // 2
        number,      // 3
        punctuation, // 4
        value,       // 5
        error,       // 6

    ];
}

fn out(screen: &mut Screen, node: &Rc<Node>, chars: &mut Chars, cord: &mut Vec2<usize>) {
    if let Some(color) = node.kind.color {
        for _ in node.span.0..node.span.1 {
            let chr = chars.next().unwrap();
            
            if chr == '\n' {
                cord.y += 1;
                cord.x  = 0;
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
    let mut src = "".to_string();
    let tokens = toks();
    let mut doc = Document::new(&tokens);

    let mut screen = Screen::new();
    let mut index = 0;

    for event in screen.events() {
        match event.unwrap() {
            Event::Key(Key::Char(chr)) => {
                src.insert(index, chr);
                
                doc.parse(src.as_str(), Edit {
                    span: (index, index),
                    len: 1,
                });

                index += 1;
                
                out(&mut screen, &doc.root, &mut src.chars(), &mut Vec2::new(0, 0));
            }
            Event::Key(Key::Left) => {
                if index != 0 {
                    index -= 1;
                }
            },
            Event::Key(Key::Right) => {
                if index != src.len() {
                    index += 1;
                }
            },
            Event::Key(Key::Backspace) => {
                if index != 0 {
                    src.remove(index - 1);

                    doc.parse(src.as_str(), Edit {
                        span: (index - 1, index),
                        len: 0,
                    });

                    screen.set(' ', Color(0, 0, 0), &Vec2::new(src.len(), 0));

                    index -= 1;

                    out(&mut screen, &doc.root, &mut src.chars(), &mut Vec2::new(0, 0));
                }
            }
            _ => break
        }

        screen.blit();
    }
}

