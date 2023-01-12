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
    sim.take_steps(1000);
    let total_energy = sim.total_energy();
    println!("{total_energy}");
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
        let moons = input.split("\n")
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty())
            .map(|line| {
                Moon::from_line(line)
            }).collect();

        Self {
            moons,
            step: 0,
        }
    }

    fn step(&mut self) {
        (0..self.moons.len())
            .combinations(2)
            .for_each(|combo| {
                let moon_1 = combo.get(0).unwrap();
                let moon_2 = combo.get(1).unwrap();
                self.apply_gravity(*moon_1, *moon_2);
        });
        self.apply_velocity();
        self.step += 1;
    }

    fn take_steps(&mut self, n: usize) {
        (0..n).for_each(|_| self.step() );
    }

    fn apply_gravity(&mut self, idx_1: usize, idx_2: usize) {
        let moon_1 = self.moons.get(idx_1).unwrap();
        let moon_2 = self.moons.get(idx_2).unwrap();
        let mut new_1 = moon_1.clone();
        let mut new_2 = moon_2.clone();

        match moon_1.position.x.cmp(&moon_2.position.x) {
            Ordering::Less => {
                new_1.velocity.x += 1;
                new_2.velocity.x -= 1;
            }
            Ordering::Equal => {
            }
            Ordering::Greater => {
                new_1.velocity.x -= 1;
                new_2.velocity.x += 1;
            }
        }
        match moon_1.position.y.cmp(&moon_2.position.y) {
            Ordering::Less => {
                new_1.velocity.y += 1;
                new_2.velocity.y -= 1;
            }
            Ordering::Equal => {
            }
            Ordering::Greater => {
                new_1.velocity.y -= 1;
                new_2.velocity.y += 1;
            }
        }
        match moon_1.position.z.cmp(&moon_2.position.z) {
            Ordering::Less => {
                new_1.velocity.z += 1;
                new_2.velocity.z -= 1;
            }
            Ordering::Equal => {
            }
            Ordering::Greater => {
                new_1.velocity.z -= 1;
                new_2.velocity.z += 1;
            }
        }
        self.moons[idx_1] = new_1;
        self.moons[idx_2] = new_2;
    }

    fn apply_velocity(&mut self) {
        self.moons.iter_mut().for_each(|moon| {
            moon.apply_velocity()
        });
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter()
            .fold(0, |acc, moon| acc + moon.total_energy() )
    }
}

#[derive(Clone, Debug)]
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

    fn potential_energy(&self) -> i32 {
        self.position.energy()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.energy()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Clone, Debug)]
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

    fn energy(&self) -> i32 {
        self.x.abs() +
            self.y.abs() +
            self.z.abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::Sim;

    #[test]
    fn test_example_1() {
        let input = r"
                <x=-1, y=0, z=2>
                <x=2, y=-10, z=-7>
                <x=4, y=-8, z=8>
                <x=3, y=5, z=-1>
            ";
        let mut sim = Sim::new(input);
        sim.take_steps(10);

        assert_eq!(sim.moons.get(0).unwrap().position.x, 2);
        assert_eq!(sim.moons.get(0).unwrap().position.y, 1);
        assert_eq!(sim.moons.get(0).unwrap().position.z, -3);
        assert_eq!(sim.moons.get(0).unwrap().velocity.x, -3);
        assert_eq!(sim.moons.get(0).unwrap().velocity.y, -2);
        assert_eq!(sim.moons.get(0).unwrap().velocity.z, 1);

        assert_eq!(sim.total_energy(), 179);
    }
}
