use std::cmp::Ordering;
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
    let program = read_input("inputs/day13.txt");
    let mut automaton = Automaton::new_with_program(&program);
    automaton.run();
    let block_tiles = automaton.output.iter()
        .enumerate()
        .filter(| &(index,_) | (index + 1) % 3 == 0 )
        .map(| (_,&value) | Tile::from(value))
        .filter(| tile | tile == &Tile::Block)
        .count();
    println!("{block_tiles}");
}

fn part2() {
    let program = read_input("inputs/day13.txt");
    let mut arcade = Arcade::new(&program);
    arcade.play();
    println!("{}", arcade.score);
}

struct Arcade {
    computer: Automaton,
    ball_position: (i128,i128),
    paddle_position: (i128,i128),
    score: i128,
}

impl Arcade {
    fn new(program : &str) -> Self {
        Self {
            computer: Automaton::new_with_program(program),
            ball_position: (0, 0),
            paddle_position: (0, 0),
            score: 0,
        }
    }

    fn play(&mut self) {
        while !self.computer.halted {
            self.computer.run();
            // 1 process all unvisited outputs
            // println!("{}", self.computer.output.len());
            while self.computer.has_output() {
                // println!("getting 3 outputs");
                let output = self.computer.get_output(3);
                // println!("{:?}", output);
                if let Some(output) = output {
                    self.process_output(&output);
                }
            }

            // 2 determine input
            match self.paddle_position.0.cmp(&self.ball_position.0) {
                Ordering::Less => { self.computer.runtime_input(1) }
                Ordering::Equal => { self.computer.runtime_input(0) }
                Ordering::Greater => { self.computer.runtime_input(-1) }
            }
        }
    }

    fn process_output(&mut self, output: &Vec<i128>) {
        let x = *output.get(0).unwrap();
        let y = *output.get(1).unwrap();
        let value = *output.get(2).unwrap();

        if x == -1 && y == 0 {
            self.score = value;
        } else {
            match Tile::from(value) {
                Tile::Empty => {}
                Tile::Wall => {}
                Tile::Block => {}
                Tile::HorizontalPaddle => { self.paddle_position = (x, y) }
                Tile::Ball => { self.ball_position = (x, y) }
            }
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => { ' ' }
            Tile::Wall => { '|' }
            Tile::Block => { '[' }
            Tile::HorizontalPaddle => { '~' }
            Tile::Ball => { 'O' }
        }
    }
}

impl From<i128> for Tile {
    fn from(value: i128) -> Self {
        match value {
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            0 | _ => Tile::Empty,
        }
    }
}
