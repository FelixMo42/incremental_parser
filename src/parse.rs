use crate::token::{Symbol, Token};

pub struct Parse<'a> {
    pub tokens: &'a Vec<Token>,
    pub symbols: Vec<Symbol<'a>>
}

fn get_symbol(symbols: &Vec<Symbol>, cord: usize) -> usize {
    for index in 0..symbols.len() {
        if cord <= symbols[index].span.1 {
            return index;
        }
    }

    return 0;
}

impl<'a> Parse<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parse<'a> {
        return Parse {
            tokens,
            symbols: vec![]
        };
    }

    pub fn parse(&mut self, src: &str, edit: (usize, usize)) {
        // What is the index of the first symbol that could have been edited.
        let mut index = get_symbol(&self.symbols, edit.0);

        // How many characters were added?
        let offset = edit.1 - edit.0;
        
        // Increment the span of each Symbol after the beginning of the edit.
        for symbol in self.symbols.iter_mut().skip(index + 1) {
            symbol.span = (symbol.span.0 + offset, symbol.span.1 + offset);
        }

        // Creat a cursor and skip to the cursor.
        let mut cursor = if self.symbols.len() != 0 {
            src.char_indices().skip(self.symbols[index].span.0).peekable()
        } else {
            src.char_indices().skip(0).peekable()
        };

        while cursor.peek().is_some() {
            for token in self.tokens {
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
                        kind: &token,
                    };

                    if self.symbols.len() != index {
                        // If this is the same as the previously parsed symbol, then were done.
                        if self.symbols[index] == symbol && symbol.span.1 > edit.1 {
                            return
                        }

                        // Replace the old symbol if no longer needed, outherwise insert it.
                        if self.symbols[index].span.0 <= symbol.span.0 {
                            self.symbols[index] = symbol;
                        } else {
                            self.symbols.insert(index, symbol);
                        }
                    } else {
                        self.symbols.push(symbol);
                    }

                    // Move on the next symbol.
                    index += 1;
                }
            }
        }
    }
}

