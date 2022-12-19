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

    let mut automaton = Automaton::new().init().load(input.as_str()).add_initial_input(1);
    automaton.run();
    let output = automaton.get_last_output();
    println!("{output}");
}

fn part2() {
    let input = read_input("inputs/day5.txt");

    let mut automaton = Automaton::new().init().load(input.as_str()).add_initial_input(5);
    automaton.run();
    let output = automaton.get_last_output();
    println!("{output}");
}