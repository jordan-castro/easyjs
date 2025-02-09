use super::token;

/// Our Lexer structure.
pub struct Lex {
    input: String,        // Input is the code for this specific Lexer
    position: usize,      // Current position of the Lexer in input
    read_position: usize, // Current ReadPosition, i.e., one after where the position is at
    current_char: char,   // Current char being read
    /// The current line our lexer is on.
    pub current_line: i64,
    /// The exact column number.
    pub current_col: i64,
    /// The file being parsed.
    pub current_file: String,
    /// A vector of chars to not .chars() every read_char,
    input_chars: Vec<char>,
}

/// Allowed chars in ident (other than letters of course)
pub const ALLOWED_IN_IDENT: &str = "0123456789_";

impl Lex {
    /// Create a new Lex instance.
    pub fn new(input: String) -> Self {
        Lex {
            input: input.clone(),
            position: 0,
            read_position: 0,
            current_char: ' ', // Initialize with null character
            current_line: 1,
            current_col: 1,
            current_file: String::new(),
            input_chars: input.chars().collect(),
        }
    }

    /// Create a new lex instance and send in file name.
    pub fn new_with_file(input: String, file: String) -> Self {
        let mut l = Lex::new(input);
        l.current_file = file;

        l
    }

    /// Create a String object with the current and peek char.
    fn cc_pp(&mut self) -> String {
        let s = format!("{}{}", self.current_char, self.peek_char());
        self.read_char();
        s
    }

    /// checks if at the end of the file.
    fn is_eof(&self) -> bool {
        self.current_char == '\0'
    }

    /// Read the current Input[ReadPosition] character.
    /// Will update `position` and `read_position`.
    fn read_char(&mut self) {
        if (self.current_char == '\n') {
            self.current_line += 1;
            self.current_col = 1;
        } else {
            self.current_col += 1;
        }

        if self.read_position >= self.input_chars.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input_chars[self.read_position];
        }
        self.position = self.read_position; // Update position
        self.read_position += 1; // Move to the next character
    }

    /// Skip the whitespace in input.
    fn skip_whitespace(&mut self) {
        while (self.current_char == ' '
            || self.current_char == '\r'
            || self.current_char == '\t'
            || self.current_char == '\n')
            && self.current_char != '\0'
        {
            self.read_char();
        }
    }

    /// peek the next character without changing position
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input_chars[self.read_position]
        }
    }

    /// Read a string from '' to ""
    fn read_string(&mut self, cs: char) -> String {
        // Go to next char to not be stuck in the "
        self.read_char();

        let mut result = String::new();

        // go until end of statement.
        while self.current_char != cs && !self.is_eof() {
            // check for an escaped quote.
            if self.current_char == '\\' && (self.peek_char() == '"' || self.peek_char() == '\'') {
                result.push(self.current_char);
                // move pass the backslash
                self.read_char();
            }

            // add the current character to the result string
            result.push(self.current_char);
            self.read_char();
        }

        result
    }

    /// read anything after the // token
    fn read_comment(&mut self) -> (&str, String) {
        // go next to not get stuck in the /
        self.read_char();
        self.read_char();
        let is_doc = self.current_char == '/';
        
        if is_doc {
            self.read_char();
        }
        
        let mut res: String = String::new();

        while self.current_char != '\n' && !self.is_eof() {
            res.push(self.current_char);
            self.read_char();
        }

        if is_doc {
            (token::DOC_COMMENT, res)
        } else {
            (token::COMMENT, res)
        }
    }

    /// read the identifier
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while (self.current_char.is_alphabetic() || ALLOWED_IN_IDENT.contains(self.current_char))
            && !self.is_eof()
        {
            // read the ident
            ident.push(self.current_char);
            self.read_char();
        }
        self.read_position = self.position;
        ident
    }

    /// Get the full number.
    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while (self.current_char.is_numeric() || self.current_char == '.') && !self.is_eof() {
            if self.current_char == '.' {
                if self.peek_char().is_numeric() {
                    number.push(self.current_char);
                    self.read_char();
                } else {
                    break;
                }
            }

            number.push(self.current_char);
            self.read_char();
        }

        self.read_position = self.position;

        number
    }

    /// Read the javascript token.
    fn read_javascript(&mut self) -> String {
        let mut javascript = String::new();
        // move pass the javascript and the first {
        self.read_char(); // javascript
        self.read_char(); // {

        let mut braces = 1;

        while !self.is_eof() {
            if self.current_char == '{' {
                braces += 1;
            } else if self.current_char == '}' {
                braces -= 1;
                if braces == 0 {
                    self.read_char(); // consume it as the literal.
                    break;
                }
            }

            javascript.push(self.current_char);
            self.read_char();
        }

        javascript
    }

    /// Get the current char as a &str
    fn current_char_str(&self) -> String {
        self.current_char.to_string()
    }

    /// Read the next token via the lexer.
    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        if self.is_eof() {
            return token::new_token(token::EOF, &self.current_char_str());
        }

        let token = match self.current_char {
            '=' => {
                if self.peek_char() == '=' {
                    token::new_token(token::EQ, &self.cc_pp())
                } else {
                    token::new_token(token::ASSIGN, &self.current_char_str())
                }
            }
            '.' => {
                if self.peek_char() == '.' {
                    // let fs = format!("{}{}", self.current_char, self.peek_char());
                    token::new_token(token::DOTDOT, &self.cc_pp())
                } else {
                    token::new_token(token::DOT, &self.current_char_str())
                }
            }
            '+' => {
                if self.peek_char() == '=' {
                    token::new_token(token::PLUS_EQUALS, &self.cc_pp())
                } else {
                    token::new_token(token::PLUS, &self.current_char_str())
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    token::new_token(token::MINUS_EQUALS, &self.cc_pp())
                } else {
                    token::new_token(token::MINUS, &self.current_char_str())
                }
            }
            '*' => {
                if self.peek_char() == '=' {
                    token::new_token(token::ASTERISK_EQUALS, &self.cc_pp())
                } else {
                    token::new_token(token::ASTERISK, &self.current_char_str())
                }
            }
            '{' => token::new_token(token::L_BRACE, &self.current_char_str()),
            '}' => token::new_token(token::R_BRACE, &self.current_char_str()),
            '(' => token::new_token(token::L_PAREN, &self.current_char_str()),
            ')' => token::new_token(token::R_PAREN, &self.current_char_str()),
            ',' => token::new_token(token::COMMA, &self.current_char_str()),
            ';' => token::new_token(token::SEMICOLON, &self.current_char_str()),
            '\n' => token::new_token(token::EOL, &self.current_char_str()),
            '[' => token::new_token(token::L_BRACKET, &self.current_char_str()),
            ']' => token::new_token(token::R_BRACKET, &self.current_char_str()),
            '%' => token::new_token(token::MODULUS, &self.current_char_str()),
            '!' => {
                if self.peek_char() == '=' {
                    token::new_token(token::NOT_EQ, &self.cc_pp())
                } else {
                    token::new_token(token::BANG, &self.current_char_str())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    token::new_token(token::GT_OR_EQ, &self.cc_pp())
                } else {
                    token::new_token(token::GT, &self.current_char_str())
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    token::new_token(token::LT_OR_EQ, &self.cc_pp())
                } else {
                    token::new_token(token::LT, &self.current_char_str())
                }
            }
            ':' => {
                let next_char = self.peek_char();
                let token_type = token::lookup_colon_special(
                    format!("{}{}", self.current_char, next_char).as_str(),
                );
                if token_type != token::COLON {
                    token::new_token(&token_type, &self.cc_pp())
                } else {
                    token::new_token(token_type, &self.current_char_str())
                }
            }
            '/' => {
                if self.peek_char() == '/' {
                    let (tk, comment) = self.read_comment();
                    token::new_token(tk, &comment)
                } else if self.peek_char() == '=' {
                    token::new_token(token::SLASH_EQUALS, &self.cc_pp())
                } else {
                    token::new_token(token::SLASH, &self.current_char_str())
                }
            }
            '\"' => token::new_token(token::STRING, &self.read_string('\"')),
            '\'' => token::new_token(token::STRING, &self.read_string('\'')),
            '|' => {
                if self.peek_char() == '|' {
                    token::new_token(token::OR_SYMBOL, &self.cc_pp())
                } else {
                    token::new_token(token::BITWISE_OR, &self.current_char_str())
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    token::new_token(token::AND_SYMBOL, &self.cc_pp())
                } else {
                    token::new_token(token::BITWISE_AND, &self.current_char_str())
                }
            }
            '?' => {
                if self.peek_char() == '?' {
                    token::new_token(token::DOUBLE_QUESTION_MARK, &self.cc_pp())
                } else {
                    token::new_token(token::QUESTION_MARK, &self.current_char_str())
                }
            }
            // '$' => token::new_token(token::MACRO_SYMBOL, &self.current_char_str()),
            // '@' => token::new_token(token::DECORATOR, &self.current_char_str()),
            _ => {
                // check for identifier
                if self.current_char.is_alphabetic() || self.current_char == '_' {
                    let literal = &self.read_identifier();

                    // probably a identifier
                    let ident = token::lookup_ident(literal);

                    // if this a JS?
                    if ident == token::JAVASCRIPT {
                        let t = token::new_token(token::JAVASCRIPT, &self.read_javascript());
                        self.read_char();
                        return t;
                    }

                    // return the identifier
                    token::new_token(ident, literal)
                } else if self.current_char.is_numeric() {
                    // probably a integer
                    let int = self.read_number();
                    if int.contains('.') {
                        token::new_token(token::FLOAT, int.as_str())
                    } else {
                        token::new_token(token::INT, int.as_str())
                    }
                } else {
                    token::new_token(token::ILLEGAL, &self.current_char_str())
                }
            }
        };

        // read next char!
        self.read_char();

        if token.typ == token::COMMENT {
            return self.next_token();
        }

        // return the token
        token
    }
}

/// Used for testing only.
pub fn read_all_tokens(input: String) -> Vec<token::Token> {
    let mut tokens: Vec<token::Token> = vec![];
    let mut lexer = Lex::new(input);

    while !lexer.is_eof() {
        tokens.push(lexer.next_token());
    }

    tokens
}
