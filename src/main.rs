use std::{io::{self, Read}, char, fs};
use clap::Parser;

enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Print,
    Read,
    Open,
    Close,
}

struct Interpreter {
    memory: [u8; 30000],
    program: Vec<Instruction>,
    ready: bool,
    data_pointer: usize,
    program_pointer: usize,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            memory: [0; 30000],
            program: Vec::new(),
            ready: false,
            data_pointer: 0,
            program_pointer: 0,
        }
    }

    fn load_program(&mut self, program: String) {
        self.program = Vec::new();
        for ins in program.chars().collect::<Vec<char>>() {
            match ins {
                '>' => self.program.push(Instruction::Right),
                '<' => self.program.push(Instruction::Left),
                '+' => self.program.push(Instruction::Increment),
                '-' => self.program.push(Instruction::Decrement),
                '.' => self.program.push(Instruction::Print),
                ',' => self.program.push(Instruction::Read),
                '[' => self.program.push(Instruction::Open),
                ']' => self.program.push(Instruction::Close),
                _ => {},
            }
        }

        if self.program.len() == 0 {
            panic!("No program was present");
        }
        self.ready = true;
    }

    fn increment_pointer(&mut self) {
        self.data_pointer += 1;
        if self.data_pointer == 30000 {
            panic!("Brainfuck traditionally only has 30000 bytes of memory");
        }
    }

    fn decrement_pointer(&mut self) {
        let prev = self.data_pointer;
        self.data_pointer -= 1;
        if self.data_pointer > prev {
            panic!("Can't point to a cell left o the 0-th");
        }
    }

    fn increment_value(&mut self) {
        self.memory[self.data_pointer] += 1;
    }

    fn decrement_value(&mut self) {
        self.memory[self.data_pointer] -= 1;
    }

    fn print_data(&self) {
        print!("{}", char::from(self.memory[self.data_pointer]));
    }

    // Only works with ascii
    fn scan_data(&mut self) {
        let mut buf: [u8; 1] = [0];
        io::stdin().read_exact(&mut buf).expect("Couldn't read");
        self.memory[self.data_pointer] = buf[0];
    }

    fn left_conditional(&mut self) {
        if self.memory[self.data_pointer] != 0 {
            return;
        }

        loop {
            match self.program[self.program_pointer] {
                Instruction::Close => break,
                _ => {
                    self.program_pointer += 1;
                    if self.program_pointer >= self.program.len() {
                        panic!("The program is not well formed");
                    }
                }
            }
        }
    }

    fn right_conditional(&mut self) {
        if self.memory[self.data_pointer] == 0 {
            return;
        }

        loop {
            match self.program[self.program_pointer] {
                Instruction::Open => break,
                _ => {
                    if self.program_pointer == 0 {
                        panic!("The program is not well formed");
                    }
                    self.program_pointer -= 1;
                }
            }
        }
    }

    fn fetch_execute(&mut self) -> bool {
        match self.program[self.program_pointer] {
            Instruction::Right => self.increment_pointer(),
            Instruction::Left => self.decrement_pointer(),
            Instruction::Increment => self.increment_value(),
            Instruction::Decrement => self.decrement_value(),
            Instruction::Print => self.print_data(),
            Instruction::Read => self.scan_data(),
            Instruction::Open => self.left_conditional(),
            Instruction::Close => self.right_conditional(),
        }
        self.program_pointer += 1;
        return self.program_pointer == self.program.len();
    }

    fn start(&mut self) -> bool {
        if !self.ready {
            return false;
        }
        self.ready = false;
        loop {
            if self.fetch_execute() {
                break;
            }
        }
        return true;
    }
}

#[derive(Parser)]
struct Cli {
    file: String,
}

fn main() {
    let args = Cli::parse();
    let mut inter = Interpreter::new();
    let prog = fs::read_to_string(args.file).expect("File doesn't exist");
    inter.load_program(prog);
    inter.start();
}
