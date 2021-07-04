use std::{iter::Peekable, ops::RangeInclusive, str::CharIndices};

struct Node (Vec<(RangeInclusive<char>, usize)>, bool);

type Cursor<'a> = Peekable<CharIndices<'a>>;

fn parse(nodes: &Vec<Node>, cursor: &mut Cursor) -> bool {
    let mut index = 0;

    'main: loop {
        let node = &nodes[index];
        
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

struct Symbol<'a> {
    span: (usize, usize),
    kind: &'a Token,
}

struct Token(String, Vec<Node>);

impl Token {
    pub fn name(&self) -> &String {
        return &self.0;
    } 

    pub fn nodes(&self) -> &Vec<Node> {
        return &self.1;
    } 
}

fn main() {
    let src = "let name = ";

    let const_let = Token("let".to_string(), vec![
        Node(vec![('l'..='l', 1)], false),
        Node(vec![('e'..='e', 2)], false),
        Node(vec![('t'..='t', 3)], false),
        Node(vec![], true),
    ]);

    let ident = Token("ident".to_string(), vec![
        Node(vec![
            ('a'..='z', 0),
            ('A'..='Z', 0),
            ('_'..='_', 0),
            ('\''..='\'', 0),
        ], true),
    ]);

    let whitespace = Token("whitespace".to_string(), vec![
        Node(vec![
            ('\t'..=' ', 0)
        ], true)
    ]);

    let punctuation = Token("punctuation".to_string(), vec![
        Node(vec![
            ('!'..='/', 0),
            (':'..='@', 0),
        ], true)
    ]);

    let tokens = vec![
        whitespace,
        const_let,
        punctuation,
        ident,
    ];

    let mut cursor = src.char_indices().peekable();

    let mut symbols: Vec<Symbol> = vec![];

    while cursor.peek().is_some() {
        for token in &tokens {
            // Keep a copy of the Cursor in case the parse fails.
            let mut save = cursor.clone();

            // Try to parse the token.
            let mut success = parse(token.nodes(), &mut cursor);

            // Make that at least one token has been parsed.
            if save.peek() == cursor.peek() {
                success = false;
            }

            // If the parse failed, restore the old cursor.
            if !success {
                cursor = save.clone();
            }


            if success {
                let start = save.peek().unwrap().0;
                let end = cursor.peek().map(|(i, _chr)| i.clone()).unwrap_or(src.len());

                symbols.push(Symbol {
                    span: (start, end),
                    kind: token,
                });

                break;
            }
        }
    }

    for symbol in symbols {
        println!("({}, {}) {}", symbol.span.0, symbol.span.1, symbol.kind.name());
    }
}
