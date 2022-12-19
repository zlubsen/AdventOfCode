use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};

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
    param : i32,
    mode : ParameterMode,
}

#[derive(Debug)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

pub struct Automaton {
    pub instruction_set : HashMap<i8, InstructionDef>,
    pub pc : usize,
    pub halted: bool,
    pub blocked : bool,
    pub memory : Vec<i32>,
    pub input : VecDeque<i32>,
    pub last_output : i32,
}

impl Automaton {
    pub fn new() -> Self {
        Self {
            instruction_set: HashMap::new(),
            pc: 0,
            halted: false,
            blocked: false,
            memory: vec![],
            input: VecDeque::new(),
            last_output: 0,
        }
    }

    pub fn init(mut self) -> Self {
        self.instruction_set.insert(1, InstructionDef {opcode : 1, no_params : 3});
        self.instruction_set.insert(2, InstructionDef {opcode : 2, no_params : 3});
        self.instruction_set.insert(3, InstructionDef {opcode : 3, no_params : 1});
        self.instruction_set.insert(4, InstructionDef {opcode : 4, no_params : 1});

        self.instruction_set.insert(5, InstructionDef {opcode : 5, no_params : 2});
        self.instruction_set.insert(6, InstructionDef {opcode : 6, no_params : 2});
        self.instruction_set.insert(7, InstructionDef {opcode : 7, no_params : 3});
        self.instruction_set.insert(8, InstructionDef {opcode : 8, no_params : 3});

        self.instruction_set.insert(99, InstructionDef {opcode : 99, no_params : 0});

        self.input.clear();

        self
    }

    pub fn load(mut self, input : &str) -> Self {
        self.pc = 0;
        self.halted = false;
        self.memory = input.split(",").filter_map(|w| w.parse().ok()).collect();

        self
    }

    pub fn decode(&mut self) -> Option<Instruction> {
        let val = self.memory.get(self.pc).unwrap();
        match val {
            1..=99 => Some(self.decode_default(val)),
            100..=99999 => Some(self.decode_extended(val)),
            _ => None
        }
    }

    pub fn decode_default(&self, val: &i32) -> Instruction {
        let opcode = *val as i8;
        let mut params = Vec::new();
        for i in 1..=(self.instruction_set.get(&opcode).unwrap().no_params as usize) {
            params.push(Parameter {
                param : *self.memory.get(self.pc + i).unwrap(),
                mode : ParameterMode::Position,
            });
        };
        Instruction {
            opcode,
            params,
        }
    }

    pub fn decode_extended(&self, val: &i32) -> Instruction {
        let mut op_extended = val.to_string().chars().rev().collect::<String>();
        while op_extended.len() < 5 {
            op_extended.push('0');
        }
        let op_extended = op_extended.chars().rev().collect::<String>();
        let opcode = &op_extended[3..=4].parse::<i8>().ok().unwrap();

        let mut params = Vec::new();
        for i in 1..=(self.instruction_set.get(&opcode).unwrap().no_params as usize) {
            let index = 3-i;
            let param_mode = &op_extended[index..=index].parse().ok().unwrap();
            let param = *self.memory.get(self.pc + i).unwrap();

            params.push(Parameter {
                param,
                mode : match param_mode {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
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
            let instruction = self.decode().unwrap();
            self.do_operation(&instruction);
        }
    }

    pub fn dump_memory(&self) -> &Vec<i32> {
        &self.memory
    }

    pub fn get_last_output(&self) -> i32 {
        self.last_output
    }

    pub fn add_initial_input(mut self, input : i32) -> Self {
        self.input.push_back(input);

        self
    }

    pub fn runtime_input(&mut self, input: i32) {
        self.input.push_back(input);
        self.blocked = false;
    }

    pub fn has_input(&mut self) -> bool {
        !self.input.is_empty()
    }

    pub fn read_input(&mut self) -> i32 {
        self.input.pop_front().expect("Reached end of input")
    }

    pub fn do_operation(&mut self, instruction : &Instruction) -> &Self {
        match instruction.opcode {
            1 => self.op_add(instruction),
            2 => self.op_mult(instruction),
            3 => self.op_input(instruction),
            4 => self.op_output(instruction),

            5 => self.op_jump_if_true(instruction),
            6 => self.op_jump_if_false(instruction),
            7 => self.op_less_than(instruction),
            8 => self.op_equals(instruction),

            99 => self.op_exit(),
            _ => (),
        }

        if !self.blocked {
            self.pc += self.get_increment_for_opcode(&instruction.opcode);
        }

        self
    }

    pub fn get_param_value<'a>(&'a self, instr : &'a Instruction, param_index: usize) -> &'a i32 {
        match instr.params.get(param_index).unwrap().mode {
            ParameterMode::Position => self.memory.get(instr.params.get(param_index).unwrap().param as usize).unwrap(),
            ParameterMode::Immediate => instr.params.get(param_index).unwrap().param.borrow(),
        }
    }

    pub fn get_address_value<'a>(&'a self, instr : &'a Instruction, param_index: usize) -> &'a i32 {
        instr.params.get(param_index).unwrap().param.borrow()
    }

    pub fn get_increment_for_opcode(&self, opcode : &i8) -> usize {
        (self.instruction_set.get(opcode).unwrap().no_params + 1) as usize
    }

    pub fn op_add(&mut self, instr : &Instruction) {
        let result;
        let address;
        {
            let op1 = self.get_param_value(instr, 0);
            let op2 = self.get_param_value(instr, 1);
            address = *self.get_address_value(instr, 2) as usize;
            result = op1 + op2;
        }
        self.memory[address] = result;
    }

    pub fn op_mult(&mut self, instr : &Instruction) {
        let result;
        let address;
        {
            let op1 = self.get_param_value(instr, 0);
            let op2 = self.get_param_value(instr, 1);
            address = *self.get_address_value(instr, 2) as usize;
            result = op1 * op2;
        }
        self.memory[address] = result;
    }

    pub fn op_jump_if_true(&mut self, instr : &Instruction) {
        let op1 = self.get_param_value(instr, 0);
        let op2 = self.get_param_value(instr, 1);

        if *op1 != 0 {
            self.pc = *op2 as usize;
            self.pc -= self.get_increment_for_opcode(&instr.opcode); // will be re-incremented automatically
        }
    }

    pub fn op_jump_if_false(&mut self, instr : &Instruction) {
        let op1 = self.get_param_value(instr, 0);
        let op2 = self.get_param_value(instr, 1);

        if *op1 == 0 {
            self.pc = *op2 as usize;
            self.pc -= self.get_increment_for_opcode(&instr.opcode); // will be re-incremented automatically
        }
    }

    pub fn op_less_than(&mut self, instr : &Instruction) {
        let op1 = self.get_param_value(instr, 0);
        let op2 = self.get_param_value(instr, 1);
        let address = *self.get_address_value(instr, 2) as usize;

        self.memory[address] = match *op1 < *op2 {
            true => 1,
            false => 0,
        };
    }

    pub fn op_equals(&mut self, instr : &Instruction) {
        let op1 = self.get_param_value(instr, 0);
        let op2 = self.get_param_value(instr, 1);
        let address = *self.get_address_value(instr, 2) as usize;

        self.memory[address] = match *op1 == *op2 {
            true => 1,
            false => 0,
        };
    }

    pub fn op_input(&mut self, instr : &Instruction) {
        if self.has_input() {
            let address = *self.get_address_value(instr, 0) as usize;
            self.memory[address] = self.read_input();
        } else {
            self.blocked = true;
        }
    }

    pub fn op_output(&mut self, instr : &Instruction) {
        let address = *self.get_address_value(instr, 0) as usize;
        self.last_output = self.memory[address];
        // println!("{}", self.memory[address]);
    }

    pub fn op_exit(&mut self) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::Automaton;

    #[test]
    fn test_one() {
        let mut automaton = Automaton::new()
            .init().load("1,0,0,0,99").add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.dump_memory(), &vec![2,0,0,0,99]);
    }

    #[test]
    fn test_two() {
        let mut automaton = Automaton::new()
            .init().load("2,3,0,3,99").add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,3,0,6,99]);
    }

    #[test]
    fn test_three() {
        let mut automaton = Automaton::new()
            .init().load("2,4,4,5,99,0").add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_four() {
        let mut automaton = Automaton::new()
            .init().load("1,1,1,4,99,5,6,0,99").add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_five() {
        let mut automaton = Automaton::new()
            .init().load("1002,4,3,4,33").add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_negative_values() {
        let mut automaton = Automaton::new()
            .init().load("1101,100,-1,4,0").add_initial_input(1);
        automaton.run();
        assert_eq!(*automaton.dump_memory(), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_equal_positional() {
        let mut automaton_equal_8 = Automaton::new()
            .init().load("3,9,8,9,10,9,4,9,99,-1,8").add_initial_input(8);
        automaton_equal_8.run();
        assert_eq!(automaton_equal_8.get_last_output(), 1);

        let mut automaton_less_than_8 = Automaton::new()
            .init().load("3,9,8,9,10,9,4,9,99,-1,8").add_initial_input(1);
        automaton_less_than_8.run();
        assert_eq!(automaton_less_than_8.get_last_output(), 0);
    }

    #[test]
    fn test_less_than_positional() {
        let mut automaton = Automaton::new()
            .init().load("3,9,7,9,10,9,4,9,99,-1,8").add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);

        let mut automaton = Automaton::new()
            .init().load("3,9,7,9,10,9,4,9,99,-1,8").add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);
    }

    #[test]
    fn test_equal_immediate() {
        let mut automaton_equal_8 = Automaton::new()
            .init().load("3,3,1108,-1,8,3,4,3,99").add_initial_input(8);
        automaton_equal_8.run();
        assert_eq!(automaton_equal_8.get_last_output(), 1);

        let mut automaton_less_than_8 = Automaton::new()
            .init().load("3,3,1108,-1,8,3,4,3,99").add_initial_input(1);
        automaton_less_than_8.run();
        assert_eq!(automaton_less_than_8.get_last_output(), 0);
    }

    #[test]
    fn test_less_than_immediate() {
        let mut automaton = Automaton::new()
            .init().load("3,3,1107,-1,8,3,4,3,99").add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);

        let mut automaton = Automaton::new()
            .init().load("3,3,1107,-1,8,3,4,3,99").add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);
    }

    #[test]
    fn test_jump_position() {
        let mut automaton = Automaton::new()
            .init().load("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").add_initial_input(0);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);

        let mut automaton = Automaton::new()
            .init().load("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);
    }

    #[test]
    fn test_jump_immediate() {
        let mut automaton = Automaton::new()
            .init().load("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").add_initial_input(0);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 0);

        let mut automaton = Automaton::new()
            .init().load("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").add_initial_input(1);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1);
    }

    #[test]
    fn test_around_eight() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut automaton = Automaton::new()
            .init()
            .load(program)
            .add_initial_input(7);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 999);

        let mut automaton = Automaton::new().init()
            .load(program)
            .add_initial_input(8);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1000);

        let mut automaton = Automaton::new().init()
            .load(program)
            .add_initial_input(9);
        automaton.run();
        assert_eq!(automaton.get_last_output(), 1001);
    }
}