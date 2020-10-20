use std::io::Read;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Runtime {
    pub tape: [u8; 30000],
    pub data_pointer: usize,
    pub instruction_pointer: usize,
    pub instructions: Vec<Token>,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    IncDataPointer,
    DecDataPointer,
    IncByte,
    DecByte,
    OutputByte,
    InputByte,
    JumpForward,
    JumpBack,
}

fn tokenize(c: char) -> Option<Token> {
    match c {
        '>' => Some(Token::IncDataPointer),
        '<' => Some(Token::DecDataPointer),
        '+' => Some(Token::IncByte),
        '-' => Some(Token::DecByte),
        '.' => Some(Token::OutputByte),
        ',' => Some(Token::InputByte),
        '[' => Some(Token::JumpForward),
        ']' => Some(Token::JumpBack),
        _ => None
    }
}

fn lex(program: String) -> Vec<Token> {
    program.chars().flat_map(|c| tokenize(c)).collect::<Vec<Token>>()
}

fn execute(r: &mut Runtime) {
    loop {
        match r.instructions[r.instruction_pointer] {
            Token::IncDataPointer => r.data_pointer += 1,
            Token::DecDataPointer => r.data_pointer -= 1,
            Token::IncByte => r.tape[r.data_pointer] += 1,
            Token::DecByte => r.tape[r.data_pointer] -= 1,
            Token::OutputByte => {
                if let Ok(res) =  std::str::from_utf8(&[r.tape[r.data_pointer]]) {
                    print!("{}",  res);
                }
            }
            Token::InputByte => {
                let input: Option<u8> = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok());

                if let Some(res) = input {
                    r.tape[r.data_pointer] = res;
                }
            }
            Token::JumpForward => {
                if r.tape[r.data_pointer] == 0 {
                    let mut stack = 1;
                    while stack > 0 {
                        r.instruction_pointer += 1;
                        match r.instructions[r.instruction_pointer] {
                            Token::JumpForward => stack += 1,
                            Token::JumpBack => stack -= 1,
                            _ => (),
                        }
                    }
                }
            } 
            Token::JumpBack => {
                if r.tape[r.data_pointer] != 0 {
                    let mut stack = 1;
                    while stack > 0 {
                        r.instruction_pointer -= 1;
                        match r.instructions[r.instruction_pointer] {
                            Token::JumpForward => stack -= 1,
                            Token::JumpBack => stack += 1,
                            _ => (),
                        }
                    }
                }
            }
        };
        
        r.instruction_pointer += 1;
        
        if r.instruction_pointer == r.instructions.len() {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let files = &args[1..];
        
        for file in files {
            let contents = fs::read_to_string(file).expect("Couldn't read file");
            let instructions = lex(contents);

            let mut runtime = Runtime {
                tape: [0; 30000],
                data_pointer: 0,
                instruction_pointer: 0,
                instructions: instructions,
            };

            execute(&mut runtime);
        }
    }
}