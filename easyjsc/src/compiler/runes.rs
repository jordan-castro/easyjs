use crate::parser::ast::Expression;

/// Parse $ in easyjs strings.
pub struct RuneParser {
    /// The found expressions.
    pub expressions: Vec<String>,

    /// The easyjs string
    input: String,
    /// Current position of parser in input
    position: usize,
    /// Current read position, one after where the position is at
    read_position: usize,
    /// The current char being read
    current_char: char,
    /// Input chars
    input_chars: Vec<char>,
    /// previos char
    pre_char: char,
}

impl RuneParser {
    pub fn new(input: String) -> RuneParser {
        let mut rp = RuneParser {
            expressions: vec![],
            input: input.clone(),
            position: 0,
            read_position: 0,
            current_char: ' ',
            input_chars: input.chars().collect(),
            pre_char: ' '
        };

        rp.read_char();
        rp.parse();

        rp
    }

    /// Get next char without changing state.
    fn peek_char(&self) -> char {
        if self.position >= self.input_chars.len() {
            '\0'
        } else {
            self.input_chars[self.read_position]
        }
    }

    /// Consume the current char.
    fn read_char(&mut self) {
        self.pre_char = self.current_char;
        if self.read_position >= self.input_chars.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input_chars[self.read_position];
        }
        self.position = self.read_position; // Update position
        self.read_position += 1; // Move to the next character
    }

    /// Read a rune. We don't read internal runes. that is handled seperately.
    fn read_rune(&mut self) {
        self.read_char(); // consume {
        let mut rune:String = String::new();
        let mut brace_count = 1;

        while self.current_char != '\0' {
            if self.current_char == '{' && self.pre_char != '\\' {
                brace_count += 1;
            }

            if self.current_char == '}' && self.pre_char != '\\' {
                brace_count -= 1;
                if brace_count == 0 {
                    self.read_char(); // consume }
                    break;
                }
            }

            // add char to rune
            rune.push(self.current_char);
            self.read_char();
        }

        // Add rune to expressions
        self.expressions.push(rune);
    }

    /// Parse the runed string.
    pub fn parse(&mut self) {
        while self.current_char != '\0' {
            if self.current_char == '$' && self.pre_char != '\\' && self.peek_char() == '{' {
                self.read_char(); // consume $
                // read the whole rune
                self.read_rune();
            }

            // at the end of the loop
            self.read_char();
        }
    }
}