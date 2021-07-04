use std::{iter::{Peekable, Skip}, ops::RangeInclusive, str::CharIndices};

pub type Cursor<'a> = Peekable<Skip<CharIndices<'a>>>;

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

pub fn parse<'a>(src: &'a str, tokens: &'a Vec<Token>) -> Vec<Symbol<'a>> {
    let mut cursor = src.char_indices().skip(0).peekable();

    let mut symbols: Vec<Symbol> = vec![];

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

            // The parse succeded, add it to symbols.
            if success {
                // Get the start and end point of the symbol.
                let start = save.peek().unwrap().0;
                let end = cursor.peek().map(|(i, _chr)| i.clone()).unwrap_or(src.len());

                // Create the symbol, and push it to the list.
                symbols.push(Symbol {
                    span: (start, end),
                    kind: token,
                });

                // Weve found the matching token, so we can exit now.
                break;
            }
        }
    }

    return symbols;
}
