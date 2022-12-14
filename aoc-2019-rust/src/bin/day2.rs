use std::time::Instant;
use aoc_2019_rust::read_input;

fn main() {
    println!("Part 1");
    let start_1 = Instant::now();
    part1();
    let duration_1 = start_1.elapsed();
    println!("- took {} micro secs", duration_1.as_micros());

    println!();

    println!("Part 2");
    let start_2 = Instant::now();
    part2();
    let duration_2 = start_2.elapsed();
    println!("- took {} micro secs", duration_2.as_micros());
}

fn part1() {
    let input = read_input("inputs/day2.txt");
    let program_memory = parse_instructions(input);

    let program_memory = correct_program(program_memory);

    let output = run_program(program_memory)[0];
    println!("{output}");
}

fn part2() {
    let input = read_input("inputs/day2.txt");
    let original_memory : Vec<i32> = parse_instructions(input);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program_memory = original_memory.clone();
            alter_program(&mut program_memory, noun, verb);
            let program_memory = run_program(program_memory);
            let result = program_memory[0];

            if result == 19690720 {
                let answer = (100 * program_memory[1]) + program_memory[2];
                println!("{answer}");
            }
        }
    }
}

fn parse_instructions(contents : String) -> Vec<i32> {
    contents.split(",").filter_map(|w| w.parse().ok()).collect()
}

fn correct_program(mut instr : Vec<i32>) -> Vec<i32> {
    instr[1] = 12;
    instr[2] = 2;
    instr
}

fn alter_program(instr : &mut Vec<i32>, i : i32, j : i32) -> () {
    instr[1] = i;
    instr[2] = j;
}

fn run_program(instr : Vec<i32>) -> Vec<i32> {
    let mut program = Program {
        instructions : instr,
        pc : 0,
        finished : false,
    };

    while !program.finished {
        program.do_operation();
    }
    program.instructions
}

struct Program {
    instructions : Vec<i32>,
    pc : usize,
    finished : bool
}

impl Program {
    fn next_operation(&mut self) -> &Self {
        self.pc = self.pc + 4;
        self
    }

    fn do_operation(&mut self) -> &Self {
        match self.instructions.get(self.pc).unwrap() {
            1 => self.op_add(),
            2 => self.op_mult(),
            99 => self.op_exit(),
            _ => ()
        }

        if ! self.finished {
            self.next_operation();
        }
        self
    }

    fn op_add(&mut self) {
        let op1 = self.instructions.get(*self.instructions.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.instructions.get(*self.instructions.get(self.pc + 2).unwrap() as usize).unwrap();
        let destination = *self.instructions.get(self.pc + 3).unwrap() as usize;
        self.instructions[destination] = op1 + op2;
    }

    fn op_mult(&mut self) {
        let op1 = self.instructions.get(*self.instructions.get(self.pc + 1).unwrap() as usize).unwrap();
        let op2 = self.instructions.get(*self.instructions.get(self.pc + 2).unwrap() as usize).unwrap();
        let destination = *self.instructions.get(self.pc + 3).unwrap() as usize;
        self.instructions[destination] = op1 * op2;
    }

    fn op_exit(&mut self) {
        self.finished = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::run_program;
    use crate::parse_instructions;

    #[test]
    fn test_one() {
        assert_eq!(run_program(parse_instructions(String::from("1,0,0,0,99"))),
                   vec![2,0,0,0,99]);
    }

    #[test]
    fn test_two() {
        assert_eq!(run_program(parse_instructions(String::from("2,3,0,3,99"))),
                   vec![2,3,0,6,99]);
    }

    #[test]
    fn test_three() {
        assert_eq!(run_program(parse_instructions(String::from("2,4,4,5,99,0"))),
                   vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_four() {
        assert_eq!(run_program(parse_instructions(String::from("1,1,1,4,99,5,6,0,99"))),
                   vec![30,1,1,4,2,5,6,0,99]);
    }
}