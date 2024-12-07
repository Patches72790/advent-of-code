mod simpler;
pub use simpler::find_sum_regex;

use std::{fs::read_to_string, iter::Peekable, option::Option, str::Chars};

fn process_input() -> String {
    read_to_string("inputs/third/input.txt").expect("Error getting input")
}

trait Operator<T> {
    fn execute(&self) -> Option<T>;
}

#[derive(Debug)]
struct MultOp {
    num1: i32,
    num2: i32,
}

impl MultOp {
    fn mult(&self) -> i32 {
        self.num1 * self.num2
    }
}
impl Operator<i32> for MultOp {
    fn execute(&self) -> Option<i32> {
        Some(self.mult())
    }
}
impl Operator<()> for DoOp {
    fn execute(&self) -> Option<()> {
        Some(())
    }
}
impl Operator<()> for DontOp {
    fn execute(&self) -> Option<()> {
        Some(())
    }
}

#[derive(Debug)]
struct DoOp;

#[derive(Debug)]
struct DontOp;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenType {
    Mult,
    LeftParen,
    RightParen,
    Comma,
    Number(i32),
    Do,
    Dont,
}

fn check_for_do_or_dont(peek_chars: &mut Peekable<Chars<'_>>) -> Option<TokenType> {
    peek_chars.next_if(|c| *c == 'o')?;

    if peek_chars.next_if(|c| *c == '(').is_some() {
        check_for_do(peek_chars)
    } else if peek_chars.next_if(|c| *c == 'n').is_some() {
        check_for_dont(peek_chars)
    } else {
        None
    }
}

fn check_for_do(peek_chars: &mut Peekable<Chars<'_>>) -> Option<TokenType> {
    peek_chars.next_if(|c| *c == ')')?;

    Some(TokenType::Do)
}

fn check_for_dont(peek_chars: &mut Peekable<Chars<'_>>) -> Option<TokenType> {
    peek_chars.next_if(|c| *c == '\'')?;
    peek_chars.next_if(|c| *c == 't')?;
    peek_chars.next_if(|c| *c == '(')?;
    peek_chars.next_if(|c| *c == ')')?;

    Some(TokenType::Dont)
}

fn check_for_mul(peek_chars: &mut Peekable<Chars<'_>>) -> Option<TokenType> {
    peek_chars.next_if(|c| *c == 'u')?;
    peek_chars.next_if(|c| *c == 'l')?;

    Some(TokenType::Mult)
}

fn take_digits(ch: &char, peek_chars: &mut Peekable<Chars<'_>>) -> Option<TokenType> {
    let mut str = ch.to_string();
    while let Some(s) = peek_chars.next_if(|c| c.is_numeric()) {
        str.push(s);
    }

    str.parse::<i32>()
        .map_or(None, |n| Some(TokenType::Number(n)))
}

fn scan() -> Vec<TokenType> {
    let text = process_input();
    let mut tokens = vec![];

    let mut peekable_chars = text.chars().peekable();
    while let Some(ch) = peekable_chars.next() {
        let token_type = match ch {
            'm' => check_for_mul(&mut peekable_chars),
            'd' => check_for_do_or_dont(&mut peekable_chars),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            ',' => Some(TokenType::Comma),
            ('0'..='9') => take_digits(&ch, &mut peekable_chars),
            _ => None,
        };

        if let Some(token) = token_type {
            tokens.push(token);
        }
    }

    tokens
}

struct Parser<T> {
    token_idx: usize,
    tokens: Vec<TokenType>,
    ops: Vec<Box<dyn Operator<T>>>,
}

impl<T> Parser<T>
where
    T: Sized,
{
    fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            token_idx: 0,
            tokens,
            ops: vec![],
        }
    }

    fn previous(&self) -> &TokenType {
        &self.tokens[self.token_idx - 1]
    }

    fn at_end(&self) -> bool {
        self.token_idx == self.tokens.len() - 1
    }

    fn advance(&mut self) -> &TokenType {
        if !self.at_end() {
            self.token_idx += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &TokenType {
        &self.tokens[self.token_idx]
    }

    fn match_token(&mut self, token: &TokenType) -> bool {
        std::mem::discriminant(self.advance()) == std::mem::discriminant(token)
    }

    fn check_token(&self, token: &TokenType) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn mult(&mut self) {
        if self.match_token(&TokenType::Mult) && self.match_token(&TokenType::LeftParen) {
            self.binary();
        } else if self.match_token(&TokenType::Do) {
            //self.emit_do();
        } else if self.match_token(&TokenType::Dont) {
            //self.emit_dont();
        }
    }

    fn binary(&mut self) {
        let num1 = if self.match_token(&TokenType::Number(0)) {
            Some(self.number())
        } else {
            None
        };

        if !self.match_token(&TokenType::Comma) || num1.is_none() {
            return;
        };

        let num2 = if self.match_token(&TokenType::Number(0)) {
            Some(self.number())
        } else {
            None
        };
        if !self.match_token(&TokenType::RightParen) || num2.is_none() {
            return;
        }

        if let Some(num1) = num1 {
            if let Some(num2) = num2 {
                self.emit_op(num1, num2);
            }
        }
    }

    fn number(&mut self) -> i32 {
        match self.previous() {
            TokenType::Number(n) => *n,
            _ => panic!("Shouldn't have gotten here: {:?}", self.previous()),
        }
    }

    fn emit_op(&mut self, num1: i32, num2: i32) {
        let op = MultOp { num1, num2 };
        self.ops.push(Box::new(op));
    }

    fn parse(tokens: &[TokenType]) -> Vec<Box<dyn Operator<T>>> {
        let mut parser = Parser::new(tokens.to_vec());

        while !parser.at_end() {
            parser.mult();
        }

        parser.ops
    }
}

struct VM<T> {
    can_execute: bool,
    ops: Vec<Box<dyn Operator<T>>>,
}

impl<T> VM<T> {
    fn new() -> Self {
        Self {
            can_execute: true,
            ops: vec![],
        }
    }

    fn evaluate(&self) {
        for op in &self.ops {}
    }
}

pub fn find_sum() {
    let tokens = scan();
    println!("{:?}", tokens);
    let ops = Parser::parse(&tokens);

    println!("Num ops: {}", ops.len());
}
