use std::{fmt::Display, io::{stdin, Stdin}};

pub type Value = i16;


// If an opcode ends with I, it takes an immediate argument
#[repr(i16)]
pub enum Opcode {
    NOOP = 0, // Does Nothing, is 0
    LOADI, LOAD, LOADIND, STORE, STOREIND,  // load and store for the accumulator
                                            // LOADIND loads value from field address stored
    // Arithmetic instructions.
    ADDI, ADD, SUBTRACTI, SUBTRACT,
    MULTIPLYI, MULTIPLY, DIVIDEI, DIVIDE,
    REMAINDER, REMAINDERI, NEGATE,
    // Bit Arithmetic
    SHIFTL, SHIFTLI, SHIFTR, SHIFTRI,
    AND, ANDI, OR, ORI, XOR, XORI,
    NOT,
    // Logical instructions
    EQUALI, EQUAL, GREATERI, GREATER,
    LESSI, LESS,
    // Control flow
    JUMP, CJUMP, // JUMP jumps to argument, CJUMP jumps to field address stored
    JUMPIFZERO, JUMPIFNZERO, JUMPLT, JUMPGT,
    JUMPIFOVERFLOW,
    // Move fields
    MOVE, // 2 Arguments, moves from first to second
    MOVEI, // Moves first (immediate) argument to second argument
    MOVEIND, // Moves from first argument to field address stored

    HALT, // Ends the Execution of the MV
    PRINT, // Prints the Value of the Accumulator, with newline
    PRINTC, // Prints the Value of the Accumulator as char, no newline
    GETC, // Stores the next ascii character from the console in the accumulator
}

impl Into<Opcode> for Value {

    fn into(self) -> Opcode {
        match self {
            x if Opcode::GETC           as Value == x => Opcode::GETC,
            x if Opcode::SHIFTL         as Value == x => Opcode::SHIFTL,
            x if Opcode::SHIFTLI        as Value == x => Opcode::SHIFTLI,
            x if Opcode::SHIFTR         as Value == x => Opcode::SHIFTR,
            x if Opcode::SHIFTRI        as Value == x => Opcode::SHIFTRI,
            x if Opcode::AND            as Value == x => Opcode::AND,
            x if Opcode::ANDI           as Value == x => Opcode::ANDI,
            x if Opcode::OR             as Value == x => Opcode::OR,
            x if Opcode::ORI            as Value == x => Opcode::ORI,
            x if Opcode::XOR            as Value == x => Opcode::XOR,
            x if Opcode::XORI           as Value == x => Opcode::XORI,
            x if Opcode::NOT            as Value == x => Opcode::NOT, 
            x if Opcode::NEGATE         as Value == x => Opcode::NEGATE,
            x if Opcode::REMAINDER      as Value == x => Opcode::REMAINDER,
            x if Opcode::REMAINDERI     as Value == x => Opcode::REMAINDERI,
            x if Opcode::STOREIND       as Value == x => Opcode::STOREIND,
            x if Opcode::JUMPGT         as Value == x => Opcode::JUMPGT,
            x if Opcode::JUMPLT         as Value == x => Opcode::JUMPLT,
            x if Opcode::NOOP           as Value == x => Opcode::NOOP,
            x if Opcode::PRINTC         as Value == x => Opcode::PRINTC,
            x if Opcode::PRINT          as Value == x => Opcode::PRINT,
            x if Opcode::LOAD           as Value == x => Opcode::LOAD,
            x if Opcode::LOADI          as Value == x => Opcode::LOADI,
            x if Opcode::LOADIND        as Value == x => Opcode::LOADIND,
            x if Opcode::STORE          as Value == x => Opcode::STORE,
            x if Opcode::ADD            as Value == x => Opcode::ADD,
            x if Opcode::ADDI           as Value == x => Opcode::ADDI,
            x if Opcode::SUBTRACT       as Value == x => Opcode::SUBTRACT,
            x if Opcode::SUBTRACTI      as Value == x => Opcode::SUBTRACTI,
            x if Opcode::MULTIPLY       as Value == x => Opcode::MULTIPLY,
            x if Opcode::MULTIPLYI      as Value == x => Opcode::MULTIPLYI,
            x if Opcode::DIVIDE         as Value == x => Opcode::DIVIDE,
            x if Opcode::DIVIDEI        as Value == x => Opcode::DIVIDEI,
            x if Opcode::EQUAL          as Value == x => Opcode::EQUAL,
            x if Opcode::EQUALI         as Value == x => Opcode::EQUALI,
            x if Opcode::GREATER        as Value == x => Opcode::GREATER,
            x if Opcode::GREATERI       as Value == x => Opcode::GREATERI,
            x if Opcode::LESS           as Value == x => Opcode::LESS,
            x if Opcode::LESSI          as Value == x => Opcode::LESSI,
            x if Opcode::JUMP           as Value == x => Opcode::JUMP,
            x if Opcode::CJUMP          as Value == x => Opcode::CJUMP,
            x if Opcode::JUMPIFZERO     as Value == x => Opcode::JUMPIFZERO,
            x if Opcode::JUMPIFNZERO    as Value == x => Opcode::JUMPIFNZERO,
            x if Opcode::JUMPIFOVERFLOW as Value == x => Opcode::JUMPIFOVERFLOW,
            x if Opcode::MOVE           as Value == x => Opcode::MOVE,
            x if Opcode::MOVEI          as Value == x => Opcode::MOVEI,
            x if Opcode::MOVEIND        as Value == x => Opcode::MOVEIND,
            x if Opcode::HALT           as Value == x => Opcode::HALT,
            x => {
                panic!("Unknown Opcode {}", x);
            }
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Opcode::GETC            => "GETC",
            Opcode::SHIFTL          => "SHIFTL",
            Opcode::SHIFTLI         => "SHIFTLI",
            Opcode::SHIFTR          => "SHIFTR",
            Opcode::SHIFTRI         => "SHIFTRI",
            Opcode::AND             => "AND",
            Opcode::ANDI            => "ANDI",
            Opcode::OR              => "OR",
            Opcode::ORI             => "ORI",
            Opcode::XOR             => "XOR",
            Opcode::XORI            => "XORI",
            Opcode::NOT             => "NOT",
            Opcode::NEGATE          => "NEGATE",
            Opcode::REMAINDER       => "REMAINDER",
            Opcode::REMAINDERI      => "REMAINDERI",
            Opcode::JUMPLT          => "JUMPLT",
            Opcode::JUMPGT          => "JUMPGT",
            Opcode::NOOP            => "NOOP",
            Opcode::PRINTC          => "PRINTC",
            Opcode::JUMP            => "JUMP",
            Opcode::PRINT           => "PRINT",
            Opcode::LOADI           => "LOADI",
            Opcode::LOAD            => "LOAD",
            Opcode::LOADIND         => "LOADIND",
            Opcode::STORE           => "STORE",
            Opcode::STOREIND        => "STOREIND",
            Opcode::ADDI            => "ADDI",
            Opcode::ADD             => "ADD",
            Opcode::SUBTRACTI       => "SUBTRACTI",
            Opcode::SUBTRACT        => "SUBTRACT",
            Opcode::MULTIPLYI       => "MULTIPLYI",
            Opcode::MULTIPLY        => "MULTIPLY",
            Opcode::DIVIDEI         => "DIVIDEI",
            Opcode::DIVIDE          => "DIVIDE",
            Opcode::EQUALI          => "EQUALI",
            Opcode::EQUAL           => "EQUAL",
            Opcode::GREATERI        => "GREATERI",
            Opcode::GREATER         => "GREATER",
            Opcode::LESSI           => "LESSI",
            Opcode::LESS            => "LESS",
            Opcode::CJUMP           => "CJUMP",
            Opcode::JUMPIFZERO      => "JUMPIFZERO",
            Opcode::JUMPIFNZERO     => "JUMPIFNZERO",
            Opcode::JUMPIFOVERFLOW  => "JUMPIFOVERFLOW",
            Opcode::MOVE            => "MOVE",
            Opcode::MOVEI           => "MOVEI",
            Opcode::MOVEIND         => "MOVEIND",
            Opcode::HALT            => "HALT", 
        }
    )
    }
}

pub struct VM {
    // Register
    pub accumulator:    Value,
    pub pc:             usize,

    // Fields
    pub fields:         Vec<Value>,

    // Input buffer
    input_buffer: String,
    stdin: Stdin
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VM {{\n\taccumulator: {}, \n\tpc: {},\n\tfields: {:?}\n}}", self.accumulator, self.pc, self.fields)
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            accumulator:    0,
            pc:             0,
            fields:         Vec::new(),
            input_buffer: String::new(),
            stdin: stdin()
        }
    }

    pub fn write_value(&mut self, val: Value) {
        self.fields.push(val);
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.next_value().into();
            
            // println!("Executing {}", instruction);
            
            match instruction {
                Opcode::NOOP => {}
                Opcode::GETC => {
                    if self.input_buffer.is_empty() { // No characters are bufffered
                        match self.stdin.read_line(&mut self.input_buffer) {
                            Err(error) => {
                                panic!("{}", error)
                            }
                            _ => {
                            }
                        }
                    }
                    let character = self.input_buffer.remove(0);
                    self.accumulator = character as i16;
                }
                Opcode::SHIFTL => {
                    let field = self.next_value();
                    self.accumulator <<= self.fields[field as usize];
                }
                Opcode::SHIFTLI => {
                    let arg = self.next_value();
                    self.accumulator <<= arg;
                }
                Opcode::SHIFTR => {
                    let field = self.next_value();
                    self.accumulator >>= self.fields[field as usize];
                }
                Opcode::SHIFTRI => {
                    let arg = self.next_value();
                    self.accumulator >>= arg;
                }
                Opcode::AND => {
                    let field = self.next_value();
                    self.accumulator &= self.fields[field as usize];
                }
                Opcode::ANDI => {
                    let arg = self.next_value();
                    self.accumulator &= arg;
                }
                Opcode::OR => {
                    let field = self.next_value();
                    self.accumulator |= self.fields[field as usize];
                }
                Opcode::ORI => {
                    let arg = self.next_value();
                    self.accumulator |=  arg;
                }
                Opcode::XOR => {
                    let field = self.next_value();
                    self.accumulator ^= self.fields[field as usize];
                }
                Opcode::XORI => {
                    let arg = self.next_value();
                    self.accumulator ^= arg;
                }
                Opcode::NOT => {
                    self.accumulator = !self.accumulator;
                }
                Opcode::PRINTC => {
                    print!("{}", char::from_u32(self.accumulator as u32).unwrap_or(char::REPLACEMENT_CHARACTER));
                }
                Opcode::PRINT => {
                    println!("{}", self.accumulator);
                },
                Opcode::LOAD => { // Store Value in field stored in argument field
                    let field = self.next_value();
                    self.accumulator = self.fields[field as usize];
                }
                Opcode::LOADI => { // Store immediate value
                    let arg = self.next_value();
                    self.accumulator = arg;
                },
                Opcode::LOADIND => { // Store value at field stored in accumulator
                    self.accumulator = self.fields[self.accumulator as usize];
                },
                Opcode::STORE => {
                    let field = self.next_value();
                    self.fields[field as usize] = self.accumulator;
                },
                Opcode::STOREIND => {
                    let address_field = self.next_value() as usize;
                    let address = self.fields[address_field] as usize;
                    self.fields[address] = self.accumulator;
                }
                Opcode::REMAINDERI => {
                    let arg = self.next_value();
                    self.accumulator = self.accumulator.wrapping_rem(arg);
                }
                Opcode::REMAINDER => {
                    let field = self.next_value();
                    self.accumulator = self.accumulator.wrapping_rem(self.fields[field as usize]);
                }
                Opcode::ADDI => {
                    let arg = self.next_value();
                    self.accumulator = arg.wrapping_add(self.accumulator);
                },
                Opcode::ADD => {
                    let field = self.next_value();
                    self.accumulator = self.fields[field as usize].wrapping_add(self.accumulator);
                },
                Opcode::SUBTRACTI => {
                    let arg = self.next_value();
                    self.accumulator = self.accumulator.wrapping_sub(arg);
                },
                Opcode::SUBTRACT => {
                    let field = self.next_value();
                    self.accumulator -= self.accumulator.wrapping_sub(self.fields[field as usize]);
                },
                Opcode::NEGATE => {
                    self.accumulator = self.accumulator.wrapping_neg();
                }
                Opcode::MULTIPLYI => {
                    let arg = self.next_value();
                    self.accumulator = arg.wrapping_mul(self.accumulator);
                },
                Opcode::MULTIPLY => {
                    let field = self.next_value();
                    self.accumulator = self.fields[field as usize].wrapping_mul(self.accumulator);
                },
                Opcode::DIVIDEI => {
                    let arg = self.next_value();
                    self.accumulator = self.accumulator.wrapping_div(arg);
                },
                Opcode::DIVIDE => {
                    let field = self.next_value();
                    self.accumulator /= self.accumulator.wrapping_div(self.fields[field as usize]);
                },
                Opcode::EQUALI => {
                    let arg = self.next_value();
                    self.accumulator = if self.accumulator == arg {1} else {0};
                },
                Opcode::EQUAL => {
                    let field = self.next_value();
                    self.accumulator = if self.accumulator == self.fields[field as usize] {1} else {0};
                },
                Opcode::GREATERI => { // Test if argument is greater than accumulator
                    let arg = self.next_value();
                    self.accumulator = if self.accumulator < arg {1} else {0};
                },
                Opcode::GREATER => {
                    let field = self.next_value();
                    self.accumulator = if self.accumulator < self.fields[field as usize] {1} else {0};
                },
                Opcode::LESSI => { // Tet if argument is less than accumulator
                    let arg = self.next_value();
                    self.accumulator = if self.accumulator > arg {1} else {0};
                },
                Opcode::LESS => {
                    let field = self.next_value();
                    self.accumulator = if self.accumulator > self.fields[field as usize] {1} else {0};
                },
                Opcode::JUMP => {
                    let arg = self.next_value();
                    self.pc = arg as usize;
                },
                Opcode::CJUMP => {
                    self.pc = self.accumulator as usize;
                },
                Opcode::JUMPIFZERO => {
                    let arg = self.next_value();
                    if self.accumulator == 0 {
                        self.pc = arg as usize;
                    }
                },
                Opcode::JUMPIFNZERO => {
                    let arg = self.next_value();
                    if self.accumulator != 0 {
                        self.pc = arg as usize;
                    }
                },
                Opcode::JUMPLT => {
                    let arg = self.next_value();
                    if self.accumulator < 0 {
                        self.pc = arg as usize;
                    }
                }
                Opcode::JUMPGT => {
                    let arg = self.next_value();
                    if self.accumulator > 0 {
                        self.pc = arg as usize;
                    }
                }
                Opcode::JUMPIFOVERFLOW => {
                    panic!("Overflow detection not implemented yet.");
                },
                Opcode::MOVE => {
                    let from = self.next_value();
                    let to = self.next_value();
                    self.fields[to as usize] = self.fields[from as usize];
                },
                Opcode::MOVEI => {
                    let value = self.next_value();
                    let to = self.next_value();
                    self.fields[to as usize] = value;
                },
                Opcode::MOVEIND => {
                    let from = self.next_value();
                    self.fields[self.accumulator as usize] = self.fields[from as usize];
                },
                Opcode::HALT => {
                    println!("\n");
                    break;
                },
            }
        }
    }

    fn next_value(&mut self) -> Value {
        let value = self.fields[self.pc];
        self.pc += 1;
        value
    }
}
