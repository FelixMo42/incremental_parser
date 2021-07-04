pub mod token;

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

fn get_symbol(symbols: &Vec<Symbol>, cord: usize) -> usize {
    for index in 0..symbols.len() {
        if cord < symbols[index].span.1 {
            return index;
        }
    }
    return 0;
}

type Span = (usize, usize);

fn edit<'a>(tokens: &'a Vec<Token>, symbols: &mut Vec<Symbol<'a>>, src: &str, edit: Span) {
    // What is the index of the first symbol that could have been edited.
    let mut index = get_symbol(&symbols, edit.0 - 1);

    // How many characters were added?
    let offset = edit.1 - edit.0;
    
    // Increment the span of each Symbol after the beginning of the edit.
    for symbol in symbols.iter_mut().skip(index) {
        symbol.span = (symbol.span.0 + offset, symbol.span.1 + offset);
    }

    // Creat a cursor and skip to the cursor.
    let mut cursor = src.char_indices().skip(symbols[index].span.0).peekable();

    while cursor.peek().is_some() {
        for token in tokens {
            // Keep a copy of the Cursor in case the parse fails.
            let mut save = cursor.clone();

            // Try to parse the token.
            let mut success = token.parse(&mut cursor);

            // Make that at least one token has been parsed.
            if save.peek() == cursor.peek() {
                success = false;
            }

            // If the parse failed, restore the old cursor.
            if !success {
                cursor = save.clone();
            }

            if success {
                // Get the start and end point of the symbol.
                let start = save.peek().unwrap().0;
                let end = cursor.peek().map(|(i, _chr)| i.clone()).unwrap_or(src.len());

                let symbol = Symbol {
                    span: (start, end),
                    kind: token,
                };

                // If this is the same as the previously parsed symbol, then were done.
                if symbols[index] == symbol {
                    return
                }

                // Replace the old symbol if no longer needed, outherwise insert it.
                if symbols[index].span.0 < symbol.span.0 {
                    symbols[index] = symbol;
                } else {
                    symbols.insert(index, symbol);
                }

                // Move on the next symbol.
                index += 1;
            }
        }
    }


    println!("{}", index);
}

fn main() {
    let src = "let name = abc + this_is_cool";

    let tokens = toks();
    let mut symbols = parse(src, &tokens);

    println!("========");
    for symbol in &symbols {
        if symbol.kind != &tokens[0] {
            println!("({}, {}) {}", symbol.span.0, symbol.span.1, symbol.kind.name);
        }
    }

    let src = "let name = abc= + this_is_cool";
    edit(&tokens, &mut symbols, src, (14, 15));

    println!("========");
    for symbol in &symbols {
        if symbol.kind != &tokens[0] {
            println!("({}, {}) {}", symbol.span.0, symbol.span.1, symbol.kind.name);
        }
    }
}
