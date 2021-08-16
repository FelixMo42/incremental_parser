use tblit::vec2::Vec2;
use crate::document::Document;

/// Document user cursor.
pub struct Cursor {
    /// The xy position of the cursor in the document. 
    pub position: Vec2<usize>,

    /// The current byte position of the cursor in the document.
    offset: usize,

    /// Memorizes the x position for when using next_line or prev_line.
    sticky_x: usize,
}

impl Cursor {
    /// Initializes a cursor at the start of a document.
    pub fn new() -> Cursor {
        Cursor {
            position: Vec2::new(0, 0),
            offset: 0,
            sticky_x: 0,
        }
    }
}

impl Cursor {
    /// Inserts the input into the current cursor position, and move it forwards.
    pub fn write(&mut self, document: &mut Document, input: &str) {
        document.edit((self.offset, self.offset), input);

        for _ in 0..input.len() {
            self.next_char(document);
        }
    }

    /// Remove the character beffor the cursor position.
    pub fn delete(&mut self, document: &mut Document) {
        if self.offset == 0 {
            return
        }

        self.prev_char(document);

        document.edit((self.offset, self.offset + 1), "");
    }
}

impl Cursor {
    /// Moves to next character, without bound checking.
    fn next(&mut self, document: &Document) {
        if document.text.is_newline(self.offset) {
            self.position.y += 1;
            self.position.x = 0;
        } else {
            self.position.x += 1;
        }

        self.offset += 1;
    }

    /// Moves to previous character, without bound checking.
    fn prev(&mut self, document: &Document) {
        self.offset -= 1;

        if document.text.is_newline(self.offset) {
            self.position.y -= 1;
            self.position.x = 0;

            while
                self.position.x != self.offset &&
                !document.text.is_newline(self.offset - self.position.x - 1)
            {
                self.position.x += 1;
            }
        } else {
            self.position.x -= 1;
        }
    }
}

impl Cursor {
    /// Moves the cursor to the next character. Also updates the sticky x position.
    pub fn next_char(&mut self, document: &Document) {
        // Go to the next line if were not at the end.
        if self.offset != document.text.byte_len() { 
            self.next(&document);
        }

        self.sticky_x = self.position.x;
    }

    /// Moves the cursor to the previous character. Also updates the sticky x position.
    pub fn prev_char(&mut self, document: &Document) {
        if self.offset != 0 {
            self.prev(&document);
        }
        
        self.sticky_x = self.position.x;
    }

    /// Moves the cursor to the next line at the current sticky x.
    pub fn next_line(&mut self, document: &Document) {
        // Dont do anything if at the end of the document
        if self.offset == document.text.byte_len() {
            return
        }

        // move to next line
        self.next(document);
        while self.position.x != 0 && self.offset < document.text.byte_len() {
            self.next(document);
        }

        // move back to the previous x value, or to the end of line
        while self.position.x < self.sticky_x && !document.text.is_newline(self.offset) {
            self.next(document);
        }
    }

    /// Moves the cursor to the previous line at the current sticky x.
    pub fn prev_line(&mut self, document: &Document) {
        // we cant go to previous line, if were at the first line
        if self.position.y == 0 {
            return
        }

        // move down to previous line
        self.prev(document);
        while !document.text.is_newline(self.offset) {
            self.prev(document);
        }

        // move down to correct position in line
        while self.position.x > self.sticky_x {
            self.prev(document);
        }
    }
}

