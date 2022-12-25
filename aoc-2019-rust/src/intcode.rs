use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[allow(dead_code)]
pub struct InstructionDef {
    opcode : i8,
    no_params : i8,
}

pub struct Instruction {
    opcode : i8,
    params : Vec<Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
    address: u128,
    mode : ParameterMode,
}

#[derive(Debug, Copy, Clone)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

pub struct Automaton {
    pub instruction_set : HashMap<i8, InstructionDef>,
    pub pc : u128,
    pub halted: bool,
    pub blocked : bool,
    pub memory : HashMap<u128,i128>,
    pub input : VecDeque<i128>,
    pub output : Vec<i128>,
    pub relative_base: i128,
}

impl Automaton {
    pub fn new() -> Self {
        let mut automaton = Self {
            instruction_set: HashMap::new(),
            pc: 0,
            halted: false,
            blocked: false,
            memory: HashMap::new(),
            input: VecDeque::new(),
            output: Vec::new(),
            relative_base: 0,
        };

        automaton.init();
        automaton
    }

    pub fn new_with_program(program: &str) -> Self {
        let automaton = Automaton::new();
        automaton.load(program)
    }

    fn init(&mut self) {
        self.instruction_set.insert(1, InstructionDef {opcode : 1, no_params : 3});
        self.instruction_set.insert(2, InstructionDef {opcode : 2, no_params : 3});
        self.instruction_set.insert(3, InstructionDef {opcode : 3, no_params : 1});
        self.instruction_set.insert(4, InstructionDef {opcode : 4, no_params : 1});

        self.instruction_set.insert(5, InstructionDef {opcode : 5, no_params : 2});
        self.instruction_set.insert(6, InstructionDef {opcode : 6, no_params : 2});
        self.instruction_set.insert(7, InstructionDef {opcode : 7, no_params : 3});
        self.instruction_set.insert(8, InstructionDef {opcode : 8, no_params : 3});
        self.instruction_set.insert(9, InstructionDef {opcode : 9, no_params : 1});

        self.instruction_set.insert(99, InstructionDef {opcode : 99, no_params : 0});

        self.input.clear();
    }

    fn load(mut self, input : &str) -> Self {
        input.split(",")
            .filter_map(|w| w.parse().ok())
            .enumerate()
            .for_each(|(i, instr)| {
                self.memory.insert(i as u128, instr);
            });

        self
    }

    fn decode(&self) -> Instruction {
        let val = self.read_value(self.pc, ParameterMode::Immediate) as i128;
        match val {
            1..=99 => self.decode_default(&val),
            100..=99999 => self.decode_extended(&val),
            invalid => panic!("Invalid opcode: {invalid}")
        }
    }

    fn decode_default(&self, val: &i128) -> Instruction {
        let opcode = *val as i8;
        let num_params = self.instruction_set.get(&opcode).unwrap().no_params as u128;
        let mut params = Vec::new();
        for i in 1..=num_params {
            params.push(Parameter {
                address: self.pc + i,
                mode : ParameterMode::Position,
            });
        };
        Instruction {
            opcode,
            params,
        }
    }

    fn decode_extended(&self, val: &i128) -> Instruction {
        let mut op_extended = val.to_string().chars().rev().collect::<String>();
        while op_extended.len() < 5 {
            op_extended.push('0');
        }
        let op_extended = op_extended.chars().rev().collect::<String>();
        let opcode = &op_extended[3..=4].parse::<i8>().ok().unwrap();

        let mut params = Vec::new();
        for i in 1..=(self.instruction_set.get(&opcode).unwrap().no_params as u128) {
            let index = 3-(i as usize);
            let param_mode = &op_extended[index..=index].parse().ok().unwrap();
            let address = self.pc + i;

            params.push(Parameter {
                address,
                mode : match param_mode {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    2 => ParameterMode::Relative,
                    _ => panic!("unknown parameter mode"),
                },
            });
        };
        Instruction {
            opcode: *opcode,
            params,
        }
    }

    pub fn run(&mut self) {
        while !self.halted && !self.blocked {
            let instruction = self.decode();
            self.do_operation(&instruction);
        }
    }

    pub fn dump_memory(&self) -> Vec<i128> {
        let mem : Vec<i128> = self.memory.keys()
            .sorted()
            .map(|key| self.memory.get(key))
            .filter(|opt|opt.is_some())
            .map(|opt|opt.unwrap())
            .map(|val| val.clone())
            .collect();
        mem
    }

    pub fn get_last_output(&self) -> i128 {
        self.output.iter().last().unwrap().clone()
    }

    pub fn add_initial_input(mut self, input : i128) -> Self {
        self.input.push_back(input);

        self
    }

    pub fn runtime_input(&mut self, input: i128) {
        self.input.push_back(input);
        self.blocked = false;
    }

    fn has_input(&mut self) -> bool {
        !self.input.is_empty()
    }

    fn read_input(&mut self) -> i128 {
        self.input.pop_front().expect("Reached end of input")
    }

    fn write_value(&mut self, value: i128, write_parameter: u128, mode: ParameterMode) {
        let address = self.parameter_to_address(write_parameter, mode) as u128;
        self.memory.insert(address, value);
    }

    fn read_value(&self, read_parameter: u128, mode: ParameterMode) -> i128 {
        let address = self.parameter_to_address(read_parameter, mode);
        let value = self.read_from_address(address);
        value
    }

    fn parameter_to_address(&self, param_value: u128, mode: ParameterMode) -> u128 {
        match mode {
            ParameterMode::Position => {
                self.read_from_address(param_value) as u128
            }
            ParameterMode::Immediate => {
                param_value
            }
            ParameterMode::Relative => {
                (self.read_from_address(param_value) + self.relative_base) as u128
            }
        }
    }

    fn read_from_address(&self, address: u128) -> i128 {
        *self.memory.get(&address).unwrap_or(&0)
    }

    fn get_increment_for_opcode(&self, opcode : &i8) -> u128 {
        (self.instruction_set.get(opcode).unwrap().no_params + 1) as u128
    }

    fn do_operation(&mut self, instruction : &Instruction) {
        let new_pc = match instruction.opcode {
            1 => self.op_add(instruction),
            2 => self.op_mult(instruction),
            3 => self.op_input(instruction),
            4 => self.op_output(instruction),

            5 => self.op_jump_if_true(instruction),
            6 => self.op_jump_if_false(instruction),
            7 => self.op_less_than(instruction),
            8 => self.op_equals(instruction),
            9 => self.op_relative_base(instruction),

            99 => { self.op_exit(); 0 },
            _ => panic!("Unsupported instruction opcode!"),
        };

        if !self.blocked {
            self.pc = new_pc;
        }
    }

    fn op_add(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);
        let param3 = instr.params.get(2).unwrap();

        let result = op1 + op2;
        self.write_value(result, param3.address, param3.mode);

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_mult(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);
        let param3 = instr.params.get(2).unwrap();

        let result = op1 * op2;
        self.write_value(result, param3.address, param3.mode);

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_jump_if_true(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);

        if op1 != 0 {
            op2 as u128
        } else {
            self.pc + self.get_increment_for_opcode(&instr.opcode)
        }
    }

    fn op_jump_if_false(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);

        if op1 == 0 {
            op2 as u128
        } else {
            self.pc + self.get_increment_for_opcode(&instr.opcode)
        }
    }

    fn op_less_than(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);
        let param3 = instr.params.get(2).unwrap();

        self.write_value(match op1 < op2 {
            true => 1,
            false => 0,
        }, param3.address, param3.mode);

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_equals(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address, param1.mode);
        let param2 = instr.params.get(1).unwrap();
        let op2 = self.read_value(param2.address, param2.mode);
        let param3 = instr.params.get(2).unwrap();

        self.write_value(match op1 == op2 {
            true => 1,
            false => 0,
        }, param3.address, param3.mode);

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_input(&mut self, instr : &Instruction) -> u128 {
        let pc_increment = if self.has_input() {
            let param1 = instr.params.get(0).unwrap();
            let input = self.read_input();
            self.write_value(input, param1.address, param1.mode);
            self.get_increment_for_opcode(&instr.opcode)
        } else {
            self.blocked = true;
            0
        };

        self.pc + pc_increment
    }

    fn op_output(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address as u128, param1.mode);
        self.output.push(op1);

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_relative_base(&mut self, instr : &Instruction) -> u128 {
        let param1 = instr.params.get(0).unwrap();
        let op1 = self.read_value(param1.address as u128, param1.mode);
        self.relative_base += op1;

        self.pc + self.get_increment_for_opcode(&instr.opcode)
    }

    fn op_exit(&mut self) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::Automaton;

    #[test]
    fn test_one() {
        let mut automaton = Automaton::new_with_program("1,0,0,0,99")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.dump_memory(), vec![2,0,0,0,99]);
    }

    #[test]
    fn test_two() {
        let mut automaton = Automaton::new_with_program("2,3,0,3,99")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,3,0,6,99]);
    }

    #[test]
    fn test_three() {
        let mut automaton = Automaton::new_with_program("2,4,4,5,99,0")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_four() {
        let mut automaton = Automaton::new_with_program("1,1,1,4,99,5,6,0,99")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_five() {
        let mut automaton = Automaton::new_with_program("1002,4,3,4,33")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_negative_values() {
        let mut automaton = Automaton::new_with_program("1101,100,-1,4,0")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_equal_positional() {
        let mut automaton_equal_8 = Automaton::new_with_program("3,9,8,9,10,9,4,9,99,-1,8")
            .add_initial_input(8);
        automaton_equal_8.run();
        assert_eq!(automaton_equal_8.get_last_output(), 1);

        let mut automaton_less_than_8 = Automaton::new_with_program("3,9,8,9,10,9,4,9,99,-1,8")
            .add_initial_input(1);
        automaton_less_than_8.run();
        assert_eq!(automaton_less_than_8.get_last_output(), 0);
    }

    #[test]
    fn test_less_than_positional() {
        let mut automaton = Automaton::new_with_program("3,9,7,9,10,9,4,9,99,-1,8")
            .add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);

        let mut automaton = Automaton::new_with_program("3,9,7,9,10,9,4,9,99,-1,8")
            .add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);
    }

    #[test]
    fn test_equal_immediate() {
        let mut automaton_equal_8 = Automaton::new_with_program("3,3,1108,-1,8,3,4,3,99")
            .add_initial_input(8);
        automaton_equal_8.run();
        assert_eq!(automaton_equal_8.get_last_output(), 1);

        let mut automaton_less_than_8 = Automaton::new_with_program("3,3,1108,-1,8,3,4,3,99")
            .add_initial_input(1);
        automaton_less_than_8.run();
        assert_eq!(automaton_less_than_8.get_last_output(), 0);
    }

    #[test]
    fn test_less_than_immediate() {
        let mut automaton = Automaton::new_with_program("3,3,1107,-1,8,3,4,3,99")
            .add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);

        let mut automaton = Automaton::new_with_program("3,3,1107,-1,8,3,4,3,99")
            .add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);
    }

    #[test]
    fn test_jump_position() {
        let mut automaton = Automaton::new_with_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")
            .add_initial_input(0);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);

        let mut automaton = Automaton::new_with_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);
    }

    #[test]
    fn test_jump_immediate() {
        let mut automaton = Automaton::new_with_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")
            .add_initial_input(0);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);

        let mut automaton = Automaton::new_with_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")
            .add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);
    }

    #[test]
    fn test_around_eight() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut automaton = Automaton::new_with_program(program)
            .add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 999);

        let mut automaton = Automaton::new_with_program(program)
            .add_initial_input(8);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1000);

        let mut automaton = Automaton::new_with_program(program)
            .add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1001);
    }

    #[test]
    fn test_relative_example_1() {
        let program = "109,2000,109,19,99";
        let mut automaton = Automaton::new_with_program(program);
        automaton.run();
        assert_eq!(automaton.relative_base, 2019);
    }

    #[test]
    fn test_relative_example_2() {
        let program = "109,2019,204,-34,99";
        let mut automaton = Automaton::new_with_program(program);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);
    }

    #[test]
    fn test_produce_copy_of_self() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut automaton = Automaton::new_with_program(program);
        automaton.run();
        assert_eq!(automaton.dump_memory(), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99, 16, 1]);
    }

    #[test]
    fn test_produce_16_digit_output() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let mut automaton = Automaton::new_with_program(program);
        automaton.run();
        assert!(automaton.get_last_output() > 1_000_000_000_000_000);
    }

    #[test]
    fn test_output_large_nr() {
        let program = "104,1125899906842624,99";
        let mut automaton = Automaton::new_with_program(program);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1125899906842624);
    }
}