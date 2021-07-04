use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};

pub type Cursor<'a> = Peekable<Skip<CharIndices<'a>>>;
pub type Span = (usize, usize);

#[derive(PartialEq, Eq)]
pub struct Node (pub Vec<(RangeInclusive<char>, usize)>, pub bool);

#[derive(PartialEq, Eq)]
pub struct Symbol<'a> {
    pub span: (usize, usize),
    pub kind: &'a Token,
}

#[derive(PartialEq, Eq)]
pub struct Token {
    pub name: String,
    pub nodes: Vec<Node>
}

impl Token {
    pub fn new(name: String, nodes: Vec<Node>) -> Token {
        return Token {
            name,
            nodes
        }
    }

    pub fn parse(&self, cursor: &mut Cursor) -> bool {
        let mut index = 0;

        'main: loop {
            let node = &self.nodes[index];
            
            if let Some((_, chr)) = cursor.peek() {
                for (case, i) in &node.0 {
                    if case.contains(chr) {
                       index = i.clone();

                       cursor.next();

                       continue 'main;
                    }
                }
            }

            if node.1 {
                return true;
            }
            
            return false;
        }
    }
}

fn get_symbol(symbols: &Vec<Symbol>, cord: usize) -> usize {
    for index in 0..symbols.len() {
        if cord <= symbols[index].span.1 {
            return index;
        }
    }

    return 0;
}

pub fn parse<'a>(tokens: &'a Vec<Token>, symbols: &mut Vec<Symbol<'a>>, src: &str, edit: Span) {
    // What is the index of the first symbol that could have been edited.
    let mut index = get_symbol(&symbols, edit.0);

    // How many characters were added?
    let offset = edit.1 - edit.0;
    
    // Increment the span of each Symbol after the beginning of the edit.
    for symbol in symbols.iter_mut().skip(index) {
        symbol.span = (symbol.span.0 + offset, symbol.span.1 + offset);
    }

    // Creat a cursor and skip to the cursor.
    let mut cursor = if symbols.len() != 0 {
        src.char_indices().skip(symbols[index].span.0 - 1).peekable()
    } else {
        src.char_indices().skip(0).peekable()
    };

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

                if symbols.len() != index {
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
                } else {
                    symbols.push(symbol);
                }

                // Move on the next symbol.
                index += 1;
            }
        }
    }
}
