use std::{fs::read_to_string, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    OpenLoop,
    CloseLoop,
    Expr(char),
    Comment,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '>' | '<' | '.' | ',' | '+' | '-' => Self::Expr(value),
            '[' => Self::OpenLoop,
            ']' => Self::CloseLoop,
            _ => Self::Comment,
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Incr,
    Decr,
    ShiftLeft,
    ShiftRight,
    Output,
    Input,
    Loop { children: Vec<Node> },
}

type Ast = Vec<Node>;
type Tokens = Vec<Token>;

fn tokenize(input: &str) -> Tokens {
    input.chars().into_iter().map(|c| c.into()).collect()
}

fn build_ast<I>(tokens: &mut I) -> Result<Ast, Box<dyn std::error::Error>>
where
    I: Iterator<Item = Token>,
{
    let mut ast = vec![];
    while let Some(token) = tokens.next() {
        match token {
            Token::Expr(c) => match c {
                '>' => ast.push(Node::ShiftRight),
                '<' => ast.push(Node::ShiftLeft),
                '+' => ast.push(Node::Incr),
                '-' => ast.push(Node::Decr),
                '.' => ast.push(Node::Output),
                ',' => ast.push(Node::Input),
                _ => return Err("unknown expression".into()),
            },
            Token::OpenLoop => {
                ast.push(Node::Loop {
                    children: build_ast(tokens)?,
                });
            }
            Token::CloseLoop => {
                return Ok(ast);
            }
            Token::Comment => {}
        }
    }
    Ok(ast)
}

fn parse(input: &str) -> Result<Ast, Box<dyn std::error::Error>> {
    build_ast(&mut tokenize(input).into_iter())
}

fn _interpret<O>(
    ast: &Ast,
    data: &mut [u8],
    ptr: &mut usize,
    output: &mut O,
) -> Result<(), Box<dyn std::error::Error>>
where
    O: std::io::Write,
{
    for node in ast {
        match node {
            Node::Incr => {
                data[*ptr] += 1;
            }
            Node::Decr => {
                data[*ptr] -= 1;
            }
            Node::ShiftLeft => {
                *ptr -= 1;
            }
            Node::ShiftRight => {
                *ptr += 1;
            }
            Node::Output => {
                output.write(&[data[*ptr]]).unwrap();
            }
            Node::Input => {
                let mut input = [0_u8];
                std::io::stdin().read_exact(&mut input).unwrap();
                data[*ptr] = input[0];
            }
            Node::Loop { children } => {
                while data[*ptr] != 0 {
                    _interpret(children, data, ptr, output)?;
                }
            }
        }
    }
    Ok(())
}

fn interpret(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ast = parse(input)?;
    let mut output = std::io::stdout();
    let mut data = [0_u8; 30_000];
    let mut ptr = 0;
    _interpret(&ast, &mut data, &mut ptr, &mut output)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} brainfuck", args[0]);
        return;
    }
    let bf_file_path = &args[1];
    let input = read_to_string(bf_file_path).unwrap();
    interpret(&input).unwrap();
}
