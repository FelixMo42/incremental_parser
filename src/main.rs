pub mod token;

use std::fmt::Write;

use crate::token::{parse, Token, Node, Symbol};

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

fn out(symbols: &Vec<Symbol>, src: &str) {
    let mut chars = src.chars();

    for symbol in symbols {
        let mut value = "".to_string();

        for _ in 0..(symbol.span.1 - symbol.span.0) {
            value.write_char( chars.next().unwrap() ).unwrap();
        }

        if symbol.kind.name == "whitespace" {
            continue; 
        }
        
        println!("'{}' {}", value, symbol.kind.name);
    }
}

fn main() {
    let tokens = toks();
    let mut symbols: Vec<Symbol> = vec![];

    let src = "let name = abc + this_is_cool";
    parse(&tokens, &mut symbols, src, (0, src.len()));

    println!("=====");
    out(&symbols, src);
    
    let src = "let name = abc= + this_is_cool";
    parse(&tokens, &mut symbols, src, (14, 15));

    println!("=====");
    out(&symbols, src);
}
