use std::time::Instant;
use itertools::Itertools;
use aoc_2019_rust::read_input;
use aoc_2019_rust::intcode::Automaton;

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
    let input = read_input("inputs/day7.txt");

    let output = vec![0,1,2,3,4].iter().permutations(5).unique()
        .map(|sequence| {
            run_sequence(input.as_str(), &sequence, 0)
        })
        .max().expect("Expected a maximum value");

    println!("{output}");
}

fn part2() {
    let input = read_input("inputs/day7.txt");

    let output = vec![5,6,7,8,9].iter().permutations(5).unique()
        .map(|sequence| {
            run_feedback_loop(input.as_str(), &sequence, 0)
        })
        .max().expect("Expected a maximum value");

    println!("{output}");
}

fn run_sequence(program: &str, sequence: &Vec<&i32>, initial_input: i32) -> i32 {
    let mut output = initial_input;
    for &item in sequence {
        let mut amp = Automaton::new()
            .init()
            .load(program)
            .add_initial_input(*item).add_initial_input(output);
        amp.run();
        output = amp.get_last_output();
    }
    output
}

fn run_feedback_loop(program: &str, sequence: &Vec<&i32>, initial_input: i32) -> i32 {
    let mut amps = vec![
        Automaton::new().init().load(program).add_initial_input(**sequence.get(0).unwrap()),
        Automaton::new().init().load(program).add_initial_input(**sequence.get(1).unwrap()),
        Automaton::new().init().load(program).add_initial_input(**sequence.get(2).unwrap()),
        Automaton::new().init().load(program).add_initial_input(**sequence.get(3).unwrap()),
        Automaton::new().init().load(program).add_initial_input(**sequence.get(4).unwrap()),
    ];

    let mut loop_signal = initial_input;
    while !all_halted(&amps) {
        loop_signal = amps.iter_mut().fold(loop_signal, |signal, amp| {
            amp.runtime_input(signal);
            amp.run();
            amp.get_last_output()
        })
    }

    amps[4].get_last_output()
}

fn all_halted(amps: &Vec<Automaton>) -> bool {
    amps.iter()
        .map(|amp| amp.halted)
        .reduce(|acc, instance| acc && instance).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{run_feedback_loop, run_sequence};

    #[test]
    fn test_amps_43210() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phase_sequence = vec![&4,&3,&2,&1,&0];
        let output = run_sequence(program, &phase_sequence, 0);
        assert_eq!(output, 43210);
    }

    #[test]
    fn test_amps_54321() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase_sequence = vec![&0,&1,&2,&3,&4];
        let output = run_sequence(program, &phase_sequence, 0);
        assert_eq!(output, 54321);
    }

    #[test]
    fn test_amps_65210() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase_sequence = vec![&1,&0,&4,&3,&2];
        let output = run_sequence(program, &phase_sequence, 0);
        assert_eq!(output, 65210);
    }

    #[test]
    fn test_feedback_139629729() {
        let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phase_sequence = vec![&9,&8,&7,&6,&5];
        let output = run_feedback_loop(program, &phase_sequence, 0);

        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_feedback_18216() {
        let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let phase_sequence = vec![&9,&7,&8,&5,&6];
        let output = run_feedback_loop(program, &phase_sequence, 0);

        assert_eq!(output, 18216);
    }
}