use std::{str::Chars, iter::Peekable, fmt::Display};
use std::collections::HashMap;

use crate::vm::{Value, VM};

use super::vm;
// Scanning

// Tokens
enum Token<'a> {
    // Labels
    NamedLabel(&'a str), NumberLabel(usize),
    // Arguments to Opcodes. 
    ImmediateNumber(vm::Value), ImmediateLabel(&'a str), String(&'a str),
    // Opcodes
    OpCode(vm::Opcode),
    // End of String
    EndOfInput,
}

struct Compiler<'a> {
    labels:     HashMap<&'a str, usize>,
    had_error:  bool,
}

struct Scanner<'a> {
    source:     &'a str,
    iter:       Peekable<Chars<'a>>,
    pos:        usize,
    length:     usize,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndOfInput                    => write!(f,"EndOfInput"),
            Self::NamedLabel(name)       => write!(f,"NamedLabel({})", name),
            Self::NumberLabel(num)      => write!(f,"NumberLabel({})", num),
            Self::ImmediateLabel(name)   => write!(f,"ImmediateLabel({})", name),
            Self::ImmediateNumber(num)    => write!(f,"ImmediateNumber({})",num),
            Self::String(str)            => write!(f,"String({})", str),
            Self::OpCode(code)         => write!(f, "OpCode({})", code),
        }
    }
}

impl<'a> Scanner<'a> {
    fn new<'b>(source: &'b str) -> Scanner<'b> {
        Scanner {
            source:     source,
            length:     source.chars().count(),
            iter:       source.chars().peekable(),
            pos:        0, // We start at the first char
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        if self.at_end() { // If we are at the end, we return EndOfInput
            return Token::EndOfInput;
        };

        let next_char = self.advance();
        
        match next_char {
            ';' => {    // If we encounter an comment, we skip until the end of line and return the next Token
                while !self.at_end() && self.peek() != '\n'  {
                    self.advance();
                }

                if !self.at_end() {self.advance();} // Go over the ending newline
                
                return self.next_token();
            }
            '"' => {
                let start = self.pos;
                while !self.at_end() && self.peek() != '"' {
                    self.advance();
                }

                if!self.at_end() { // skip over "
                    self.advance();
                }
                
                let string = &self.source[start..self.pos-1];
                return Token::String(string);
            }
            a => {
                if a.is_alphabetic() { 
                    // If we see a alphabetic character, we return a symbol
                    return self.symbol();
                } else if a.is_digit(10) || a == '-' { 
                    // If we see a digit, we return a number
                    return self.number(a, 10);
                } else if a.is_whitespace() { 
                    // We skip whitespace and return the next token
                    while !self.at_end() && self.peek().is_whitespace() {
                        self.advance();
                    }

                    return self.next_token();
                } else { // Otherwise, it must be an disallowed character
                    panic!("Unexpected character: {}", next_char);
                }
            }
        
        }
    }

    fn label_or_opcode(symbol: &str) -> Token {
        match symbol {
            "GETC"          => Token::OpCode(vm::Opcode::GETC           ),
            "HALT"          => Token::OpCode(vm::Opcode::HALT           ),
            "LOADI"         => Token::OpCode(vm::Opcode::LOADI          ),
            "LOAD"          => Token::OpCode(vm::Opcode::LOAD           ),
            "LOADIND"       => Token::OpCode(vm::Opcode::LOADIND        ),
            "STORE"         => Token::OpCode(vm::Opcode::STORE          ),
            "ADDI"          => Token::OpCode(vm::Opcode::ADDI           ),
            "ADD"           => Token::OpCode(vm::Opcode::ADD            ),
            "SUBTRACTI"     => Token::OpCode(vm::Opcode::SUBTRACTI      ),
            "SUBTRACT"      => Token::OpCode(vm::Opcode::SUBTRACT       ),
            "MULTIPLYI"     => Token::OpCode(vm::Opcode::MULTIPLYI      ),
            "MULTIPLY"      => Token::OpCode(vm::Opcode::MULTIPLY       ),
            "DIVIDEI"       => Token::OpCode(vm::Opcode::DIVIDEI        ),
            "DIVIDE"        => Token::OpCode(vm::Opcode::DIVIDE         ),
            "EQUALI"        => Token::OpCode(vm::Opcode::EQUALI         ),
            "EQUAL"         => Token::OpCode(vm::Opcode::EQUAL          ),
            "LESSI"         => Token::OpCode(vm::Opcode::LESSI          ),
            "LESS"          => Token::OpCode(vm::Opcode::LESS           ),
            "GREATERI"      => Token::OpCode(vm::Opcode::GREATERI       ),
            "GREATER"       => Token::OpCode(vm::Opcode::GREATER        ),
            "JUMP"          => Token::OpCode(vm::Opcode::JUMP           ),
            "CJUMP"         => Token::OpCode(vm::Opcode::CJUMP          ),
            "JUMPIFZERO"    => Token::OpCode(vm::Opcode::JUMPIFZERO     ),
            "JUMPIFNZERO"   => Token::OpCode(vm::Opcode::JUMPIFNZERO    ),
            "JUMPIFOVERFLOW"=> Token::OpCode(vm::Opcode::JUMPIFOVERFLOW ),
            "MOVE"          => Token::OpCode(vm::Opcode::MOVE           ),
            "MOVEI"         => Token::OpCode(vm::Opcode::MOVEI          ),
            "MOVEIND"       => Token::OpCode(vm::Opcode::MOVEIND        ),
            "PRINT"         => Token::OpCode(vm::Opcode::PRINT          ),
            "PRINTC"        => Token::OpCode(vm::Opcode::PRINTC         ),
            "NOOP"          => Token::OpCode(vm::Opcode::NOOP           ),
            "JUMPLT"        => Token::OpCode(vm::Opcode::JUMPLT         ),
            "JUMPGT"        => Token::OpCode(vm::Opcode::JUMPGT         ),
            "STOREIND"      => Token::OpCode(vm::Opcode::STOREIND       ),
            "REMAINDER"     => Token::OpCode(vm::Opcode::REMAINDER      ),
            "REMAINDERI"    => Token::OpCode(vm::Opcode::REMAINDERI     ),
            "NEGATE"        => Token::OpCode(vm::Opcode::NEGATE         ),
            "SHIFTL"        => Token::OpCode(vm::Opcode::SHIFTL         ),
            "SHIFTLI"       => Token::OpCode(vm::Opcode::SHIFTLI        ),
            "SHIFTR"        => Token::OpCode(vm::Opcode::SHIFTR         ),
            "SHIFTRI"       => Token::OpCode(vm::Opcode::SHIFTRI        ),
            "AND"           => Token::OpCode(vm::Opcode::AND            ),
            "ANDI"          => Token::OpCode(vm::Opcode::ANDI           ),
            "OR"            => Token::OpCode(vm::Opcode::OR             ),
            "ORI"           => Token::OpCode(vm::Opcode::ORI            ),
            "XOR"           => Token::OpCode(vm::Opcode::XOR            ),
            "XORI"          => Token::OpCode(vm::Opcode::XORI           ),
            "NOT"           => Token::OpCode(vm::Opcode::NOT            ),
            _               => Token::ImmediateLabel(symbol             ),
        }
    }

    fn symbol(&mut self) -> Token<'a> {
        let start = self.pos;
        while !self.at_end() && self.peek().is_alphabetic() { // Skip all Label Character
            self.advance();
        }

        let symbol = &self.source[start-1..self.pos]; // The Characters we skipped over

        if !self.at_end() && self.peek() == ':' {
            self.advance();
            let result = Token::NamedLabel(symbol);
            result
        } else {
            let result = Scanner::label_or_opcode(symbol);
            result
        }
    }

    fn number(&mut self, start: char, base: u32) -> Token<'a> {
        let sign = start != '-';
        // The parsed number
        let mut number: i32 = if start != '-' {start.to_digit(base).unwrap() as i32} else {0}; 

        // If we are at the end, we return a number token
        if self.at_end() {return Token::ImmediateNumber(number as vm::Value);}

        // As long as we aren't at the end and the next character is a digit
        while !self.at_end() && self.peek().is_digit(base) {
            number *= base as i32;
            number += self.advance().to_digit(base).unwrap() as i32;
        }
        if !sign { number = -number}
        // If the number is followed by a :, it is a NumberLabel
        if !self.at_end() && self.peek() == ':' {
            self.advance();
            return Token::NumberLabel(number as usize);
        } else {
            return Token::ImmediateNumber(number as vm::Value);
        }
    }
    
    fn advance(&mut self) -> char {
        let result = self.iter.next().expect("Error: read past index");
        self.pos += result.len_utf8();
        result
    }

    fn peek(&mut self) -> char {
        *self.iter.peek().expect("Error: tried to peek past the last element")
    }

    fn at_end(&self) -> bool {
        return self.pos >= self.length;
    }

    fn to_tokens(mut self) -> Vec<Token<'a>> {
        let mut result = Vec::new();
        while !self.at_end() {
            result.push(self.next_token());
        }

        result
    }
}

pub fn compile(source: &str) -> Option<vm::VM> {
    // The resulting vm
    let mut vm = vm::VM::new();

    // The compiler state
    let mut compiler = Compiler::new();
    // The scanner
    let scanner = Scanner::new(source);
    let tokens = scanner.to_tokens();
    
    // Find and define all labels in the source code
    compiler.define_labels(&tokens);

    // Parse all instruction
    parse_ops(&mut compiler, &mut vm, tokens);
    
    if compiler.had_error { // If there was an error compiling the code, we return None
        Option::None
    } else {                // If compilation was successfull, we return the vm
        Option::Some(vm)
    }
}

impl<'a> Compiler<'a> {
    fn new() -> Compiler<'a> {
        Compiler {
            labels:     HashMap::new(),
            had_error:  false,
        }
    }

    fn define_label(&mut self, name: &'a str, pos: usize) {
        self.labels.insert(name, pos);
    }

    fn get_label(&self, name: &str) -> Option<usize> {
        self.labels.get(name).map(|label| *label)
    }

    fn define_labels(&mut self,tokens: &Vec<Token<'a>>) {
        let mut pos = 0;
        for token in tokens {
            match token {
                Token::NamedLabel(name) => {
                    self.define_label(name, pos);
                }
                Token::NumberLabel(n) => {
                    if *n >= pos {
                        pos = *n;
                    } else {
                        println!("Error: Expect Number Label to be at least the position of the field it labels: got {}, expected at least {}", *n, pos);
                        self.had_error = true;
                    }
                },
                Token::ImmediateNumber(_) => {
                    pos += 1;
                },
                Token::ImmediateLabel(_) => {
                    pos += 1;
                },
                Token::OpCode(_) => {
                    pos += 1;
                },
                Token::String(str) => {
                    pos += str.len();
                }
                Token::EndOfInput => {},
            }
        }
    }
}

fn parse_ops<'a>(compiler: &mut Compiler<'a>, vm: &mut VM, tokens: Vec<Token<'a>>) {
    let mut pos = 0; // The position in the code
    
    for token in tokens {
        match token {
            Token::NamedLabel(_) => {}, // Labels are already defined
            Token::NumberLabel(n) => { // Numberlabel: we fill al skipped fields with 0
                if n > pos {
                    for _ in pos..n {
                        vm.write_value(0);
                        pos += 1;
                    }
                }
            },
            Token::String(str) => {
                for c in str.chars() {
                    vm.write_value(c as Value);
                }
            }
            Token::ImmediateNumber(n) => { // A Number
                vm.write_value(n);
                pos += 1;
            },
            Token::ImmediateLabel(name) => { // We use a label and replace it with its position
                let value = compiler.get_label(name);
                match value {
                    Some(value) => {
                        vm.write_value(value as Value);
                    },
                    None => {
                        println!("Unkown label '{}'", name);
                        compiler.had_error = true;
                    }
                }
                pos += 1;
            },
            Token::OpCode(c) => {   // We write the corresponding Value for the OpCode
                vm.write_value(c as Value);
                pos += 1;
            },
            Token::EndOfInput => {
                break;
            },
        }
    }
}