mod source;

use crate::source::Source;

// Node struct
#[derive(Debug)]
pub struct Node {
    range: (usize, usize),
    info: NodeType,
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Error,
    Symbol,
}

impl Node {
    fn error() -> Node {
        Node {
            range: (0, 0),
            info: NodeType::Error
        }
    }

    fn symbol() -> Node {
        Node {
            range: (0, 0),
            info: NodeType::Symbol
        }
    }
}

// Rule trait
pub trait Rule {
    fn parse(&self, source: &mut Source) -> Node;
}

type RuleId = usize;

//
struct PlaceholderRule {}

impl Rule for PlaceholderRule {
    fn parse(&self, _source: &mut Source) -> Node {
        unreachable!();
    }
}

// Symbol Rule
impl Rule for &str {
    fn parse(&self, source: &mut Source) -> Node {
        source.skip_white_space();
        for chr in self.chars() {
            if !source.eat_char(chr) {
                return Node::error()
            }
        }
        return Node::symbol();
    }
}

// Loop Rule
struct Lots {
    rule: RuleId
}

impl Rule for Lots {
    fn parse(&self, source: &mut Source) -> Node {
        while let Node { range: _, info: NodeType::Symbol } = source.eat_rule(self.rule) {} 
        return Node::symbol();
    }
}

//
struct EndOfFile {}

impl Rule for EndOfFile {
    fn parse(&self, source: &mut Source) -> Node {
        source.skip_white_space();
        if source.is_end_of_file() {
            return Node::symbol();
        } else {
            return Node::error();
        }
    }
}

// 
struct Union (Vec<usize>);

impl Rule for Union {
    fn parse(&self, source: &mut Source) -> Node {
        for rule in self.0.iter() {
            source.skip_white_space();
            
            if source.eat_rule(*rule).info == NodeType::Error {
                return Node::error(); 
            }
        }

        return Node::symbol();
    }
}

struct Ident {}

impl Rule for Ident {
    fn parse(&self, source: &mut Source) -> Node {
        let range = 'a'..='z';

        source.skip_white_space();

        if !source.eat_char_range(&range) {
            return Node::error();
        }

        while source.eat_char_range(&range) {}
        
        return Node::symbol();
    }
}

struct Opts {
    opts: Vec<RuleId>
}

impl Rule for Opts {
    fn parse(&self, source: &mut Source) -> Node {
        for opt in self.opts.iter() {
            if source.eat_rule(*opt).info == NodeType::Symbol {
                return Node::symbol()
            }
        }
        return Node::error();
    }
}

// Main
fn main() {
    let src = &mut Source::new(r#"
        bla [ x y ] (add x y)
        main [args] (bla (unwrap args))
    "#);

    let placeholder = &PlaceholderRule {};

    // ident
    let ident = src.add_rule(&Ident {});

    // params
    let params = &Lots { rule: ident };
    let params = src.add_rule(params);

    // value
    let value = src.add_rule(placeholder);

    // values
    let values = &Lots { rule: value };
    let values = src.add_rule(values);

    // call
    let call = &Union( vec![
        src.add_rule(&"("),
        ident,
        values,
        src.add_rule(&")")
    ] );
    let call = src.add_rule(call);

    // func
    let func = &Union( vec![
        ident,
        src.add_rule(&"["),
        params,
        src.add_rule(&"]"),
        value,
    ] );
    let func = src.add_rule(func);
    
        
    // add value
    let v = &Opts { opts: vec![
        ident,
        call
    ] };
    src.set_rule(value, v);

    let eof = &EndOfFile {};
    let funcs = &Lots { rule: func };
    let file = &Union( vec![
        src.add_rule(funcs),
        src.add_rule(eof),
    ] );
    let file = src.add_rule(file);

    let prs = src.eat_rule(file);

    println!("{:?}", prs);
}
