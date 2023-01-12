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
        .filter(| &(index,value) | (index + 1) % 3 == 0 )
        .map(| (index,&value) | Tile::from(value))
        .filter(| tile | tile == &Tile::Block)
        .count();
    println!("{block_tiles}");
}

fn part2() {
    // let program = read_input("inputs/day13.txt");
}

struct Arcade {
    computer: Automaton,
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
