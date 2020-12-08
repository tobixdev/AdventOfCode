use std::collections::VecDeque;
use crate::util::read_lines;

#[derive(Clone, Debug)]
pub struct Interpreter<'a> {
    accumulator: i64,
    ip: usize,
    program: &'a Program,
    visited: Vec<bool>,
}

#[derive(Debug)]
pub struct Program {
    code: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    instruction_code: InstructionCode,
    argument: i64,
}

#[derive(Debug)]
enum InstructionCode {
    Acc,
    Jmp,
    Nop,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program) -> Interpreter<'a> {
        Interpreter {
            accumulator: 0,
            ip: 0,
            program,
            visited: (0..program.code.len()).map(|_i| false).collect(),
        }
    }

    pub fn execute(&mut self) -> Result<i64, i64> {
        while self.ip != self.program.code.len() {
            if self.visited[self.ip] {
                return Err(self.accumulator);
            }
            self.next()
        }
        return Ok(self.accumulator);
    }

    fn next(&mut self) {
        let inst = &self.program.code[self.ip];
        self.visited[self.ip] = true;

        match inst.instruction_code {
            InstructionCode::Acc => {
                self.accumulator += inst.argument;
                self.ip += 1;
            }
            InstructionCode::Jmp => self.ip = (self.ip as i64 + inst.argument) as usize,
            InstructionCode::Nop => self.ip += 1,
        };
    }

    pub fn execute_repair(&mut self) -> i64 {
        let mut work_set = VecDeque::<Interpreter>::new();
        work_set.push_back(self.clone());
        
        while self.ip != self.program.code.len() {
            if self.visited[self.ip] {
                println!("Stopping repair attempt generation on {}.", self.ip);
                break;
            }

            let inst = &self.program.code[self.ip];

            match inst.instruction_code {
                InstructionCode::Jmp => {
                    println!("Repair attempt swap jmp -> nop on position {}.", self.ip);
                    work_set.push_back(Interpreter {
                        accumulator: self.accumulator,
                        ip: self.ip + 1,
                        program: self.program,
                        visited: self.visited.clone(),
                    })
                }
                InstructionCode::Nop => {
                    println!("Repair attempt swap nop -> jmp on position {}.", self.ip);
                    work_set.push_back(Interpreter {
                        accumulator: self.accumulator,
                        ip: (self.ip as i64 + inst.argument) as usize,
                        program: self.program,
                        visited: self.visited.clone(),
                    })
                }
                _ => ()
            };

            self.next();
        }
        
        while !work_set.is_empty() {
            let mut instance = work_set.pop_front().unwrap();


            if let Ok(result) = instance.execute() {
                return result;
            } else {
                println!("Repair attempt failed!");
            }
        }

        panic!("Could not repair program!");
    }

    pub fn get_result(&self) -> i64 {
        self.accumulator
    }
}

impl Program {
    pub fn read() -> Program {
        let lines = read_lines("./src/problems/day8/input.txt").unwrap();

        let mut code = Vec::<Instruction>::new();
        for line in lines {
            code.push(Instruction::from(&line.unwrap()))
        }

        Program { code }
    }
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let splitted = line.split(" ").collect::<Vec<&str>>();
        let instruction_code = match splitted[0] {
            "acc" => InstructionCode::Acc,
            "jmp" => InstructionCode::Jmp,
            "nop" => InstructionCode::Nop,
            _ => panic!("Unknown instruction."),
        };
        let argument = splitted[1].parse().unwrap();

        Instruction {
            instruction_code,
            argument,
        }
    }
}
