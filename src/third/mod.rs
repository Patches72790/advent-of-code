mod simpler;
pub use simpler::find_sum_regex;

use std::{fs::read_to_string, iter::Peekable, option::Option, str::Chars};

fn process_input() -> String {
    read_to_string("inputs/third/input.txt").expect("Error getting input")
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mult(i32, i32),
    Do,
    Dont,
}

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

struct Parser {
    token_idx: usize,
    tokens: Vec<TokenType>,
    ops: Vec<Op>,
}

impl Parser {
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

    fn peek(&self) -> &TokenType {
        &self.tokens[self.token_idx]
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

    fn match_token(&mut self, token: &TokenType) -> bool {
        if self.check_current(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check_current(&mut self, token: &TokenType) -> bool {
        if !self.at_end() {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn mult(&mut self) {
        let mut matched = false;
        if self.match_token(&TokenType::Mult) && self.match_token(&TokenType::LeftParen) {
            matched = true;
            self.binary();
        }
        if self.match_token(&TokenType::Do) {
            matched = true;
            self.emit_op(Op::Do);
        }
        if self.match_token(&TokenType::Dont) {
            matched = true;
            self.emit_op(Op::Dont);
        }

        if !matched {
            self.advance();
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
                self.emit_op(Op::Mult(num1, num2));
            }
        }
    }

    fn number(&mut self) -> i32 {
        match self.previous() {
            TokenType::Number(n) => *n,
            _ => panic!("Shouldn't have gotten here: {:?}", self.previous()),
        }
    }

    fn emit_op(&mut self, op: Op) {
        self.ops.push(op);
    }

    fn parse(tokens: &[TokenType]) -> Vec<Op> {
        let mut parser = Parser::new(tokens.to_vec());

        while !parser.at_end() {
            parser.mult();
        }

        parser.ops
    }
}

struct VM {
    can_execute: bool,
    ops: Vec<Op>,
}

impl VM {
    fn new(ops: Vec<Op>) -> Self {
        Self {
            can_execute: true,
            ops,
        }
    }

    fn evaluate(&mut self) -> i32 {
        let mut sum = 0;
        for op in &self.ops {
            let out = match op {
                Op::Mult(num1, num2) => {
                    if true {
                        println!("Exec: Mult({num1},{num2})");
                        Some(num1 * num2)
                    } else {
                        None
                    }
                }
                Op::Do => {
                    self.can_execute = true;
                    println!("Exec: Do");
                    None
                }
                Op::Dont => {
                    self.can_execute = false;
                    println!("Exec: Dont");
                    None
                }
            };

            if let Some(num) = out {
                sum += num;
            }
        }

        sum
    }
}

pub fn find_sum() {
    let tokens = scan();
    println!("{:?}", tokens);
    let ops = Parser::parse(&tokens);

    println!("Num ops: {}", ops.len());

    let mut vm = VM::new(ops.clone());

    let sum = vm.evaluate();

    println!("Sum: {sum}");

    let (mults, rest): (Vec<Op>, Vec<Op>) =
        ops.iter().partition(|op| matches!(**op, Op::Mult(_, _)));

    let (dos, donts): (Vec<Op>, Vec<Op>) = rest.iter().partition(|op| matches!(**op, Op::Do));

    println!("({}) : {:?}", mults.len(), mults);
    println!("({}) : {:?}", dos.len(), dos);
    println!("({}) : {:?}", donts.len(), donts);
}
