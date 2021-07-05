pub mod token;
pub mod parse;

use std::fmt::Write;

use tblit::event::{Event, Key};
use tblit::screen::Screen;
use tblit::vec2::Vec2;

use crate::token::{Token, Node, Symbol};
use crate::parse::Parse;

fn toks() -> Vec<Token> {
    let const_let = Token::new("let".to_string(), vec![
        Node(vec![('l'..='l', 1)], false),
        Node(vec![('e'..='e', 2)], false),
        Node(vec![('t'..='t', 3)], false),
        Node(vec![], true),
    ]);

    let ident = Token::new("ident".to_string(), vec![
        Node(vec![
            ('a'..='z', 0),
            ('A'..='Z', 0),
            ('_'..='_', 0),
            ('\''..='\'', 0),
        ], true),
    ]);

    let whitespace = Token::new("whitespace".to_string(), vec![
        Node(vec![
            ('\t'..=' ', 0)
        ], true)
    ]);

    let punctuation = Token::new("punctuation".to_string(), vec![
        Node(vec![
            ('!'..='/', 0),
            (':'..='@', 0),
        ], true)
    ]);

    return vec![
        whitespace,
        const_let,
        punctuation,
        ident,
    ];
}

fn out(screen: &mut Screen, symbols: &Vec<Symbol>, src: &str) {
    let mut chars = src.chars();

    for (y, symbol) in symbols.iter().enumerate() {
        for x in symbol.span.0..symbol.span.1 {
            screen.set(chars.next().unwrap(), &Vec2::new(x, y));
        }
    }
}

fn run() {
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

fn print_symbols(symbols: &Parse) {
    for symbol in &symbols.symbols {
        print!("({}, {}) {}, ", symbol.span.0, symbol.span.1, symbol.kind.name);
    }
    println!("\n")
}

fn main() {
    let debug = false;

    if !debug {
        run();
    } else {
        let tokens = toks();
        let mut symbols = Parse::new(&tokens);

        let chars = "abc = ";
        let one_at_a_time = true;

        if one_at_a_time {
            for i in 0..chars.len() {
                println!("= pass {} =", i);
                let src = chars.get(0..=chars.char_indices().nth(i).unwrap().0).unwrap();
                println!("src: {}", &src);
                symbols.parse(src, (i, src.len()));
                print_symbols(&symbols);
            }
        } else {
            symbols.parse(chars, (0, chars.len()));
            print_symbols(&symbols);
        }
    }
}
