#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    BinaryOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { op, left, right } => {
            let l = eval(left);
            let r = eval(right);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' => { chars.next(); }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '/' => { tokens.push(Token::Slash); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n: f64 = num_str.parse().unwrap();
                tokens.push(Token::Number(n));
            }
            _ => panic!("unexpected character: {}", ch),
        }
    }
    tokens
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(token) = self.current() {
            let op = match token {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_term();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while let Some(token) = self.current() {
            let op = match token {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr();
                self.advance();
                expr
            }
            _ => panic!("unexpected token: {:?}", self.current()),
        }
    }
}

fn main() {
    let input
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);  
    let ast = parser.parse_expr();         
    println!("AST: {:?}", ast);            

    let result = eval(&ast);
    println!("{} = {}", input, result);    }
