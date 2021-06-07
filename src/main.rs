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
impl Rule for String {
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

impl Lots {
    fn new(rule: RuleId) -> Box<Lots> {
        return Box::new( Lots {
            rule
        } );
    }
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
struct Union {
    rules: Vec<usize>
}

impl Union {
    fn new(rules: Vec<usize>) -> Box<Union> {
        return Box::new( Union {
            rules
        } );
    }
}

impl Rule for Union {
    fn parse(&self, source: &mut Source) -> Node {
        for rule in self.rules.iter() {
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

impl Opts {
    fn new(opts: Vec<RuleId>) -> Box<Opts> {
        return Box::new(Opts {
            opts
        })
    }
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

pub struct Language {
    pub rules: Vec<Box<dyn Rule>>
}

impl Language {
    pub fn new() -> Language {
        return Language {
            rules: vec![]
        }
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) -> usize {
        self.rules.push(rule);
        return self.rules.len() - 1;
    }
    
    pub fn set_rule(&mut self, index: usize, rule: Box<dyn Rule>) {
        self.rules[index] = rule;
    }
}

// Main
fn main() {

    let mut lang = Language::new();

    let placeholder = Box::new(PlaceholderRule {});

    // ident
    let ident = lang.add_rule(Box::new(Ident {}));

    // params
    let params = lang.add_rule(Lots::new(ident));

    // value
    let value = lang.add_rule(placeholder);

    // values
    let values = lang.add_rule(Lots::new(value));

    // call
    let open_paren = lang.add_rule(Box::new("(".to_string()));
    let close_paren = lang.add_rule(Box::new(")".to_string()));
    let call = lang.add_rule(Union::new( vec![
        open_paren,
        ident,
        values,
        close_paren
    ] ));
    
    // func
    let open_braket = lang.add_rule(Box::new("[".to_string()));
    let close_braket = lang.add_rule(Box::new("]".to_string()));
    let func = lang.add_rule(Union::new( vec![
        ident,
        open_braket,
        params,
        close_braket,
        value,
    ] ));
        
    // add value
    lang.set_rule(value, Opts::new(vec![
        ident,
        call
    ]));
    
    let funcs = lang.add_rule(Lots::new(func));
    let eof = lang.add_rule(Box::new(EndOfFile {}));
    let file = lang.add_rule(Union::new( vec![
        funcs,
        eof
    ] ));
    
    let src = &mut Source::new(&lang, r#"
        bla [ x y ] (add x y)
        main [args] (bla (unwrap args))
    "#);

    let prs = src.eat_rule(file);

    println!("{:?}", prs);
}
