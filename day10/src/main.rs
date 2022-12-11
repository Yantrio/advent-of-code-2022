use std::fmt::{Debug, Display};
use std::str::FromStr;

fn main() {
    let instructions: InstructionSet = include_str!("input").lines().map(|line| line.parse().unwrap()).collect();

    println!("part1 {}", part1(instructions.clone()));
    println!("part2:\n{}", part2(instructions.clone()));
}

fn part1(instructions: InstructionSet) -> isize {
    let mut cpu = CPU::new(instructions);

    let mut sum = 0;
    while !cpu.is_end_of_program() {
        cpu.tick();
        if let 20 | 60 | 100 | 140 | 180 | 220 = cpu.cycles {
            sum += cpu.cycles * cpu.registerX;
        }
    }
    sum
}

fn part2(instructions: InstructionSet) -> String {
    let mut cpu = CPU::new(instructions);
    let mut crt = CRT::new(40, 6);
    while !cpu.is_end_of_program() {
        cpu.tick();
        crt.tick(&cpu);
    }

    crt.to_string()
}

struct CPU {
    registerX: isize,
    cycles: isize,
    cycles_in_current_instruction: isize,
    program: InstructionSet,
    program_counter: usize,
}

struct CRT {
    width: usize,
    height: usize,

    screen: Vec<Vec<bool>>,

    cursor_x: usize,
    cursor_y: usize,
}

impl CRT {
    fn new(width: usize, height: usize) -> Self {
        CRT { width, height, cursor_x: 0, cursor_y: 0, screen: vec![vec![false; width]; height] }
    }

    fn tick(&mut self, cpu: &CPU) {
        // the sprite is three wide, with the center being at cpu.registerX
        // if our current cursor_x is within the sprite, we need to draw it
        let range = cpu.registerX - 1..=cpu.registerX + 1;

        // todo: fix this to be more efficient
        let x = self.cursor_x as isize;
        if range.contains(&x) {
            println!("drawing sprite at {},{}", self.cursor_x, self.cursor_y);
            self.screen[self.cursor_y][self.cursor_x] = true;
        }
        self.cursor_x += 1;
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // iterate over all lines and print the pixels
        for y in 0..self.height {
            for x in 0..self.width {
                if self.screen[y][x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C {:?} PC {} X {} CI {:?}", self.cycles, self.program_counter, self.registerX, self.program[self.program_counter])
    }
}

impl CPU {
    fn new(instructions: InstructionSet) -> CPU {
        CPU {
            registerX: 1,
            cycles: 0,
            cycles_in_current_instruction: 0,
            program: instructions,
            program_counter: 0,
        }
    }

    fn tick(&mut self) {
        let current_instruction = &self.program[self.program_counter];
        match current_instruction {
            Instruction::Noop => {
                // go straight to the next instruction
                self.next_instruction();
            }
            Instruction::AddX(value) => {
                match self.cycles_in_current_instruction {
                    2 => {
                        self.registerX += value;
                        self.next_instruction();
                    }
                    _ => {}
                }
            }
            _ => panic!("Unknown instruction"),
        }
        self.cycles += 1;
        self.cycles_in_current_instruction += 1;
    }

    fn is_end_of_program(&self) -> bool {
        self.program_counter >= self.program.len()
    }

    fn next_instruction(&mut self) {
        self.program_counter += 1;
        self.cycles_in_current_instruction = 0;
    }
}

type InstructionSet = Vec<Instruction>;

#[derive(Debug, Clone)]
enum Instruction {
    Unknown,
    Noop,
    AddX(isize),
}


impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts.next().unwrap();

        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(parts.next().unwrap().parse().unwrap())),
            _ => Ok(Instruction::Unknown),
        }
    }
}