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
    let program = read_input("inputs/day11.txt");
    let mut robot = Robot::new(&program);
    robot.run();
    let painted_panels = robot.number_of_painted_panels();
    println!("{painted_panels}");
}

fn part2() {
    let program = read_input("inputs/day11.txt");
    let mut robot = Robot::with_input(&program, Color::WHITE);
    robot.run();
    robot.print_panels();
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

enum Color {
    BLACK,
    WHITE,
}

impl From<i128> for Color {
    fn from(value: i128) -> Self {
        match value {
            1 => Color::WHITE,
            0 | _ => Color::BLACK,
        }
    }
}

impl From<Color> for i128 {
    fn from(color: Color) -> Self {
        match color {
            Color::BLACK => { 0 }
            Color::WHITE => { 1 }
        }
    }
}

type Coordinate = (i32,i32);

struct Robot {
    location: (i32, i32),
    direction: Direction,
    computer: Automaton,
    panels: HashMap<Coordinate,Color>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Robot {
    fn new(program: &str) -> Self {
        Self {
            location: (0, 0),
            direction: Direction::UP,
            computer: Automaton::new_with_program(program),
            panels: Default::default(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn with_input(program: &str, initial_panel_color: Color) -> Self {
        Self {
            location: (0, 0),
            direction: Direction::UP,
            computer: Automaton::new_with_program(program).add_initial_input(initial_panel_color.into()),
            panels: Default::default(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn run(&mut self) {
        while !self.computer.halted {
            self.computer.run();
            if !self.computer.output.is_empty() {
                let (color_to_paint, direction_to_turn) = self.computer.get_last_2_outputs();

                let color_to_paint = Color::from(color_to_paint);
                self.paint_panel(color_to_paint);

                match direction_to_turn {
                    0 => self.turn_left(),
                    1 | _ => self.turn_right(),
                }

                self.move_forward();
            }
            self.computer.runtime_input(self.current_panel_color());
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::LEFT,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
            Direction::RIGHT => Direction::UP,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }

    fn move_forward(&mut self) {
        self.location = match self.direction {
            Direction::UP => { (self.location.0 - 1, self.location.1) }
            Direction::DOWN => { (self.location.0 + 1, self.location.1) }
            Direction::LEFT => { (self.location.0, self.location.1 - 1 ) }
            Direction::RIGHT => { (self.location.0, self.location.1 + 1) }
        }
    }

    fn current_panel_color(&self) -> i128 {
        let color = self.panels.get(&self.location).or_else(||Some(&Color::BLACK)).unwrap();
        match color {
            Color::BLACK => { 0 }
            Color::WHITE => { 1 }
        }
    }

    fn paint_panel(&mut self, color: Color) {
        self.panels.insert(self.location, color);
        self.min_x = if self.location.0 <= self.min_x { self.location.0 } else { self.min_x };
        self.max_x = if self.location.0 >= self.max_x { self.location.0 } else { self.max_x };
        self.min_y = if self.location.1 <= self.min_y { self.location.1 } else { self.min_y };
        self.max_y = if self.location.1 >= self.max_y { self.location.1 } else { self.max_y };
    }

    fn number_of_painted_panels(&self) -> usize {
        self.panels.len()
    }

    fn print_panels(&self) {
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                let color = self.panels.get(&(x, y)).unwrap_or(&Color::BLACK);
                let color = match color {
                    Color::BLACK => {"."}
                    Color::WHITE => {"#"}
                };
                print!("{color}");
            }
            print!("\n")
        }
    }
}
