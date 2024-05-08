use core::{fmt, panic};
use std::io::{Read, Write};

struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    fn new(input: String) -> Tokenizer {
        Tokenizer { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.position += 1;
        }
        ch
    }

    fn eof(&self) -> bool {
        self.position >= self.input.len() - 1
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    fn parse_heading(tokenizer: &mut Tokenizer) -> Token {
        let mut hash_count = 1;
        while tokenizer.peek() == Some('#') {
            tokenizer.next();
            hash_count += 1;
        }

        let heading = match hash_count {
            1 => TOKEN::HEADING1,
            2 => TOKEN::HEADING2,
            3 => TOKEN::HEADING3,
            4 => TOKEN::HEADING4,
            5 => TOKEN::HEADING5,
            6 => TOKEN::HEADING6,
            _ => TOKEN::TEXT,
        };

        // grab the input after the hash and before the newline
        let mut input = String::new();
        tokenizer.skip_whitespace();

        while let Some(ch) = tokenizer.peek() {
            if ch == '\n' {
                break;
            }
            input.push(ch);
            tokenizer.next();
        }

        Token {
            token: heading,
            value: input.trim().to_string(),
        }
    }

    fn parse_paragraph(tokenizer: &mut Tokenizer) -> Token {
        let mut input = String::new();
        while let Some(ch) = tokenizer.peek() {
            if ch == '\n' {
                break;
            }
            input.push(ch);
            tokenizer.next();
        }

        Token {
            token: TOKEN::PARAGRAPH,
            value: input.trim().to_string(),
        }
    }

    fn parse_list(tokenizer: &mut Tokenizer) -> Token {
        let mut input = String::new();
        while let Some(ch) = tokenizer.peek() {
            if ch == '\n' {
                break;
            }
            input.push(ch);
            tokenizer.next();
        }

        Token {
            token: TOKEN::LIST,
            value: input.trim().to_string(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.eof() {
            return None;
        }

        let token = match self.peek() {
            Some('#') => Self::parse_heading(self),
            Some('-') => Self::parse_list(self),
            Some(_) => Self::parse_paragraph(self),
            None => panic!("STOP BEING DUMb STUPID"),
        };

        Some(token)
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TOKEN {
    HEADING1,
    HEADING2,
    HEADING3,
    HEADING4,
    HEADING5,
    HEADING6,
    PARAGRAPH,
    TEXT,
    LIST,
}

#[derive(Clone, Debug)]
struct Token {
    token: TOKEN,
    value: String,
}

fn renderMDToHTML(input: String) -> String {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    let mut output = String::new();

    let mut i = 0;

    while (i < tokens.len()) {
        let token = &tokens[i];
        match tokens[i].token {
            TOKEN::HEADING1 => output.push_str(&format!("<h1>{}</h1>", token.value)),
            TOKEN::HEADING2 => output.push_str(&format!("<h2>{}</h2>", token.value)),
            TOKEN::HEADING3 => output.push_str(&format!("<h3>{}</h3>", token.value)),
            TOKEN::HEADING4 => output.push_str(&format!("<h4>{}</h4>", token.value)),
            TOKEN::HEADING5 => output.push_str(&format!("<h5>{}</h5>", token.value)),
            TOKEN::HEADING6 => output.push_str(&format!("<h6>{}</h6>", token.value)),
            TOKEN::PARAGRAPH => output.push_str(&format!("<p>{}</p>", token.value)),
            TOKEN::TEXT => output.push_str(&format!("{}", token.value)),
            TOKEN::LIST => {
                output.push_str("<ul>");
                while (i < tokens.len() && tokens[i].token == TOKEN::LIST) {
                    output.push_str(&format!("<li>{}</li>", tokens[i].value));
                    i += 1;
                }
                output.push_str("</ul>");
            }
            _ => {}
        }
        i += 1;
    }

    output
}

fn main() {
    let listener = std::net::TcpListener::bind("0.0.0.0:3001").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer);
        let path = request.split_whitespace().nth(1).unwrap();

        println!("Request: {}", path);

        if !path.ends_with(".md") {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            continue;
        }

        if !std::path::Path::new(&path[1..]).exists() {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            continue;
        }

        let mut file = std::fs::File::open(&path[1..]).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let output = renderMDToHTML(input.clone());
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            output.len(),
            output
        );

        stream.write(response.as_bytes()).unwrap();
    }
}
