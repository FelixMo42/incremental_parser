pub mod language;
pub mod document;

use tblit::event::{Event, Key};
use tblit::screen::{Screen, Color};
use tblit::vec2::Vec2;

use crate::language::{Node, Rule, Step, Token};
use crate::document::{Document, Edit};

fn toks() -> Vec<Token> {
    let const_let = Token::new(Color(186,108,72), vec![
        Step(vec![(Rule::Char('l'..='l'), 1)], false),
        Step(vec![(Rule::Char('e'..='e'), 2)], false),
        Step(vec![(Rule::Char('t'..='t'), 3)], false),
        Step(vec![], true),
    ]);

    let name = Token::new(Color(249, 245, 235), vec![
        Step(vec![
             (Rule::Token(6), 0)
        ], true),
        Step(vec![
             (Rule::Token(1), 1)
        ], false),
    ]);

    let word = Token::new(Color(10, 245, 0), vec![
        Step(vec![
            (Rule::Char('a'..='z'), 0),
            (Rule::Char('A'..='Z'), 0),
            (Rule::Char('_'..='_'), 0),
            (Rule::Char('\''..='\''), 0),
        ], true),
    ]);

    let whitespace = Token::new(Color(0, 0, 0), vec![
        Step(vec![
            (Rule::Char('\t'..=' '), 0)
        ], true)
    ]);

    let punctuation = Token::new(Color(186,108,72), vec![
        Step(vec![
            (Rule::Char('!'..='/'), 0),
            (Rule::Char(':'..='@'), 0),
        ], true)
    ]);

    let number = Token::new(Color(1,110,115), vec![
        Step(vec![
            (Rule::Char('0'..='9'), 0),
            (Rule::Char('.'..='.'), 1),
        ], true),
        Step(vec![
            (Rule::Char('0'..='9'), 1),
        ], true)
    ]);

    let error = Token::new(Color(200, 0, 0), vec![
        Step(vec![
            (Rule::Char('\x00'..='~'), 1),
        ], false),
        Step(vec![
        ], true)
    ]);

    let file = Token::new(Color(255, 255, 255), vec![
        Step(vec![
            (Rule::Token(1), 0),
            (Rule::Token(2), 0),
            (Rule::Token(3), 0),
            (Rule::Token(4), 0),
            (Rule::Token(5), 0),
            (Rule::Token(7), 0),
        ], true)
    ]);

    return vec![
        file,        // 0
        whitespace,  // 1
        const_let,   // 2
        number,      // 3
        punctuation, // 4
        name,        // 5
        word,        // 6
        error,       // 7
    ];
}

fn out(screen: &mut Screen, symbols: &Vec<Node>, src: &str) {
    let mut chars = src.chars();

    for symbol in symbols.iter() {
        for x in symbol.span.0..symbol.span.1 {
            screen.set(chars.next().unwrap(), symbol.kind.color, &Vec2::new(x, 0));
        }
    }
}

fn main() {
    let tokens = toks();
    let mut doc = Document::new(&tokens);

    let mut screen = Screen::new();

    let mut src = "".to_string();

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
                
                out(&mut screen, &doc.nodes, &src);
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

                    out(&mut screen, &doc.nodes, &src);
                }
            }
            _ => break
        }

        screen.blit();
    }
}

