use crate::language::{Cursor, Language, Node, Token};

pub type Span = (usize, usize);

pub struct Edit {
    pub span: Span,
    pub len: usize
}

pub struct Document<'a> {
    pub tokens: &'a Vec<Token>,
    pub nodes: Vec<Node<'a>>
}

fn get_node(nodes: &Vec<Node>, cord: usize) -> usize {
    for index in 0..nodes.len() {
        if cord <= nodes[index].span.1 {
            return index;
        }
    }

    return 0;
}

impl<'a> Document<'a> {
    pub fn new(tokens: &'a Language) -> Document<'a> {
        return Document {
            tokens,
            nodes: vec![]
        };
    }
}

impl<'a> Document<'a> {
    pub fn parse(&mut self, src: &str, edit: Edit) {
        // What is the index of the first node that could have been edited.
        let mut index = get_node(&self.nodes, edit.span.0);

        // How much was removed? 
        let removed = edit.span.1 - edit.span.0;

        // Increment the span of each Symbol after the beginning of the edit.
        for node in self.nodes.iter_mut().skip(index + 1) {
            node.span = (
                node.span.0 - removed + edit.len,
                node.span.1 - removed + edit.len
            );
        }

        // Creat a cursor and skip to the cursor.
        let mut cursor = Cursor::new(if self.nodes.len() != 0 {
            src.char_indices().skip(self.nodes[index].span.0).peekable()
        } else {
            src.char_indices().skip(0).peekable()
        });
    
        while !cursor.done() {
            for token in self.tokens {
                // Keep a copy of the Cursor in case the parse fails.
                let mut save = cursor.save();

                // Try to parse the token.
                let success = token.parse(self.tokens, &mut cursor);

                if success {
                    // Get the start and end point of the node.
                    let start = save.peek().unwrap().0;
                    let end = cursor.chars.peek().map(|(i, _)| i.clone()).unwrap_or(src.len());

                    let node = Node {
                        span: (start, end),
                        kind: &token,
                        subs: vec![],
                    };

                    if self.nodes.len() != index {
                        // If this is the same as the previously parsed node, then were done.
                        if self.nodes[index] == node && node.span.1 > edit.span.1 {
                            return
                        }

                        if self.nodes[index].span.0 > node.span.0 {
                            self.nodes.insert(index, node);
                        } else if self.nodes[index].span.0 == node.span.0 {
                            self.nodes[index] = node
                        } else {
                            while self.nodes[index].span.0 < node.span.0 {
                                self.nodes.remove(index);        
                            }
                            if self.nodes[index].span.0 > node.span.0 {
                                self.nodes.insert(index, node);
                            } else if node.span.0 == self.nodes[index].span.0 {
                                self.nodes[index] = node
                            }
                        }
                    } else {
                        self.nodes.push(node);
                    }

                    // Move on the next node.
                    index += 1;

                    break;
                }
            }
        }
        
        while self.nodes.len() > index {
            self.nodes.remove(index);        
        }
    }
}

