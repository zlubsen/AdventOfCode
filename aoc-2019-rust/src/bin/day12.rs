use std::cmp::Ordering;
use std::time::Instant;
use itertools::Itertools;
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
    let input = read_input("inputs/day12.txt");
    let mut sim = Sim::new(&input);
    sim.step();
}

fn part2() {
    // let input = read_input("inputs/day12.txt");
}

struct Sim {
    moons: Vec<Moon>,
    step: usize,
}

impl Sim {
    fn new(input: &str) -> Self {
        let moons = input.split("\n").map(|line| {
            Moon::from_line(line)
        }).collect();
println!("{:?}", moons);
        Self {
            moons,
            step: 0,
        }
    }

    fn step(&mut self) {
        // let combos : Vec<Vec<usize>> = (0..self.moons.len()).combinations(2).collect();
        // println!("{:?}", combos);
        (0..self.moons.len())
            .combinations(2)
            .for_each(|combo| {
                let moon_1 = combo.get(0).unwrap();
                let moon_2 = combo.get(1).unwrap();
                self.apply_gravity(*moon_1, *moon_2);
        });
        self.apply_velocity();
    }

    fn apply_gravity(&mut self, idx_1: usize, idx_2: usize) {
        let mut moon_1 = self.moons.get_mut(idx_1).unwrap();
        let mut moon_2 = self.moons.get_mut(idx_2).unwrap();
        match moon_1.position.x.cmp(&moon_2.position.x) {
            Ordering::Less => {
                moon_1.position.x += 1;
                moon_2.position.x -= 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                moon_1.position.x -= 1;
                moon_2.position.x += 1;
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.moons.iter_mut().for_each(|moon| {
            moon.apply_velocity()
        });
    }
}

#[derive(Debug)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn from_line(line: &str) -> Self {
        let factors : Vec<i32> = line
            .strip_prefix('<').unwrap()
            .strip_suffix('>').unwrap()
            .split(',').map(|ele| ele.trim()).map(|elem|elem[2..].parse().unwrap()).collect();

        Self {
            position: Vector::with_values(
                factors.get(0).unwrap().clone(),
                factors.get(1).unwrap().clone(),
                factors.get(2).unwrap().clone(),
            ),
            velocity: Vector::new(),
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x = self.position.x + self.velocity.x;
        self.position.y = self.position.y + self.velocity.y;
        self.position.z = self.position.z + self.velocity.z;
    }
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn with_values(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}