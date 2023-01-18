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
    let input = read_input("inputs/day15.txt");

    let mut remote = RemoteControl::new(&input);
    remote.start();
}

fn part2() {
    // let input = read_input("inputs/day15.txt");
}

#[derive(Debug)]
enum Command {
    North,
    South,
    West,
    East,
}

impl From<&Command> for i128 {
    fn from(cmd: &Command) -> Self {
        match cmd {
            Command::North => { 1 }
            Command::South => { 2 }
            Command::West => { 3 }
            Command::East => { 4 }
        }
    }
}

#[derive(Debug)]
enum StatusCode {
    HitWall,
    Moved,
    Finished,
}

impl From<i128> for StatusCode {
    fn from(value: i128) -> Self {
        match value {
            0 => StatusCode::HitWall,
            1 => StatusCode::Moved,
            2 => StatusCode::Finished,
            undefined_code => panic!("Unexpected Status Code: {undefined_code}")
        }
    }
}

impl From<StatusCode> for i128 {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode::HitWall => { 0 }
            StatusCode::Moved => { 1 }
            StatusCode::Finished => { 2 }
        }
    }
}

type Coordinate = (isize, isize);
const COMMANDS: [Command; 4] = [Command::North, Command::East, Command::South, Command::West];

struct RemoteControl {
    computer: Automaton,
    current: Coordinate,
    visited: Vec<Coordinate>,
}

impl RemoteControl {
    fn new(program: &str) -> Self {
        Self {
            computer: Automaton::new_with_program(program),
            current: (0, 0),
            visited: vec![],
        }
    }

    fn start(&mut self) {
        self.computer.run();

        let minimal = COMMANDS.iter()
            .filter_map(|cmd| self.step(cmd, (0,0), &mut vec![]))
            .min()
            .unwrap_or(0);
        println!("{minimal}");
    }

    fn step(&mut self, command: &Command, current_coordinate : Coordinate, visited: &mut Vec<Coordinate>) -> Option<usize> {
        let new_coordinate = new_coordinate(command, &current_coordinate);
        println!("from {:?} go {:?} to {:?}", current_coordinate, command, new_coordinate);
        if visited.contains(&new_coordinate) {
            println!(" we were already at {:?}", new_coordinate);
            return None;
        }

        self.computer.runtime_input(command.into());
        self.computer.run();
        let code = StatusCode::from(self.computer.get_last_output());
        println!(" which yields {code:?}");
        match code {
            StatusCode::HitWall => { None }
            StatusCode::Moved => {
                visited.push(new_coordinate);
                let next_step = COMMANDS.iter()
                    .filter_map(|cmd| self.step(cmd, new_coordinate, visited))
                    .min();
                if let Some(next_step) = next_step {
                    Some(1 + next_step)
                } else { None }
            } // continue recursion
            StatusCode::Finished => {
                println!("found at {:?}", new_coordinate);
                Some(1)
            }
        }
    }
}

fn new_coordinate(command: &Command, current : &Coordinate) -> Coordinate {
    match command {
        Command::North => {
            (current.0, current.1 - 1)
        }
        Command::South => {
            (current.0, current.1 + 1)
        }
        Command::West => {
            (current.0 - 1, current.1)
        }
        Command::East => {
            (current.0 + 1, current.1)
        }
    }
}