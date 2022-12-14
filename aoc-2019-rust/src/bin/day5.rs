use std::collections::HashMap;
use std::time::Instant;
use aoc_2019_rust::intcode::Automaton;
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
    let input = read_input("inputs/day5.txt");

    let mut automaton = Automaton {
        instruction_set: HashMap::new(),
        pc: 0,
        finished: false,
        memory: vec![],
        input: 0,
        last_output: 0,
    };
    automaton.init().load(input.as_str()).set_input(1).run();
}

fn part2() {
    let input = read_input("inputs/day5.txt");

    let mut automaton = Automaton {
        instruction_set: HashMap::new(),
        pc: 0,
        finished: false,
        memory: vec![],
        input: 0,
        last_output: 0,
    };
    automaton.init().load(input.as_str()).set_input(5).run();
}