struct Lexer<'a> {
   source: String,
   pub cur_char: char,
   cur_pos: usize,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Self {
        Self {
            source.to_string(),
            cur_char: '\0',
            cur_pos: 0,
        }
    }

    fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos > self.source.len() {
            self.cur_char = '\0' // EOF
        } else {
            self.cur_char = self.source[self.cur_pos]
        }
    }

    fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.cur_pos + 1]
        }
    }
}

fn main() {
    let source = "LET foobar = 123\0";
    let mut lexer = Lexer::new(source);

    while lexer.peek() != '\0' {
        println!("{}", lexer.cur_char);
        lexer.next_char();
    }

}
