pub mod token;
pub mod parse;

use std::fmt::Write;

use tblit::event::{Event, Key};
use tblit::screen::{Screen, Color};
use tblit::vec2::Vec2;

use crate::token::{Token, Node, Symbol};
use crate::parse::Parse;

fn toks() -> Vec<Token> {
    let const_let = Token::new(Color(100, 0, 0), vec![
        Node(vec![('l'..='l', 1)], false),
        Node(vec![('e'..='e', 2)], false),
        Node(vec![('t'..='t', 3)], false),
        Node(vec![], true),
    ]);

    let ident = Token::new(Color(0, 100, 0), vec![
        Node(vec![
            ('a'..='z', 0),
            ('A'..='Z', 0),
            ('_'..='_', 0),
            ('\''..='\'', 0),
        ], true),
    ]);

    let whitespace = Token::new(Color(0, 0, 0), vec![
        Node(vec![
            ('\t'..=' ', 0)
        ], true)
    ]);

    let punctuation = Token::new(Color(40, 40, 40), vec![
        Node(vec![
            ('!'..='/', 0),
            (':'..='@', 0),
        ], true)
    ]);

    let number = Token::new(Color(0, 0, 100), vec![
        Node(vec![
            ('0'..='9', 0),
            ('.'..='.', 1),
        ], true),
        Node(vec![
            ('0'..='9', 1),
        ], true)
    ]);

    return vec![
        whitespace,
        const_let,
        number,
        punctuation,
        ident,
    ];
}

fn out(screen: &mut Screen, symbols: &Vec<Symbol>, src: &str) {
    let mut chars = src.chars();

    for symbol in symbols.iter() {
        for x in symbol.span.0..symbol.span.1 {
            screen.set(chars.next().unwrap(), symbol.kind.color, &Vec2::new(x, 0));
        }
    }
}

fn main() {
    let mut src = "".to_string();

    let mut screen = Screen::new();

    let tokens = toks();
    let mut symbols = Parse::new(&tokens);

    for event in screen.events() {
        match event.unwrap() {
            Event::Key(Key::Char(c)) => {
                src.write_char(c).unwrap();     
                symbols.parse(src.as_str(), (src.len() - 1, src.len()));
                out(&mut screen, &symbols.symbols, &src);
            }
            _ => break
            
        }
        screen.blit()
    }
}

