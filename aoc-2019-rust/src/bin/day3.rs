use std::time::Instant;
use std::collections::HashSet;
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
    let input = read_input("inputs/day3.txt");
    let wires : Vec<Vec<Move>> = parse_input(&input);

    let wire_1 = generate_path(&wires[0]);
    let wire_2 = generate_path(&wires[1]);
    let set_1: HashSet<(i32,i32)> = wire_1.iter().copied().collect();
    let set_2: HashSet<(i32,i32)> = wire_2.iter().copied().collect();

    let intersections: HashSet<_> = set_1.intersection(&set_2).collect();

    let smallest_manhattan = intersections.iter()
        .map(|coordinate| calc_manhattan(coordinate))
        .min().expect("Expected at least one intersection");
    println!("{smallest_manhattan}");
}

fn part2() {
    let input = read_input("inputs/day3.txt");
    let wires : Vec<Vec<Move>> = parse_input(&input);

    let wire_1 = generate_path(&wires[0]);
    let wire_2 = generate_path(&wires[1]);
    let set_1: HashSet<(i32,i32)> = wire_1.iter().copied().collect();
    let set_2: HashSet<(i32,i32)> = wire_2.iter().copied().collect();

    let intersections: HashSet<_> = set_1.intersection(&set_2).collect();

    let nearest = find_nearest_intersection(&wire_1, &wire_2, &intersections);

    println!("{nearest}");
}

fn parse_input(contents : &str) -> Vec<Vec<Move>> {
    let lines : Vec<&str> = contents.split("\n").collect();
    let mut wires : Vec<Vec<Move>> = Vec::new();
    for line in &lines {
        wires.push(line.split(",").map(|s| Move::from_string(s)).collect());
    }
    wires
}

fn generate_path(wire: &Vec<Move>) -> Vec<(i32,i32)> {
    let mut path : Vec<(i32,i32)> = Vec::new();
    let mut coord : (i32,i32) = (0,0);
    for mv in wire {
        for _ in 1..=mv.amount {
            match mv.direction {
                Direction::Up => {coord.1 += 1},
                Direction::Down => {coord.1 += -1}
                Direction::Left => {coord.0 += -1}
                Direction::Right => {coord.0 += 1}
            };
            path.push(coord);
        }
    }
    path
}

fn calc_manhattan(coordinate: &(i32, i32)) -> i32 {
    coordinate.0.abs() + coordinate.1.abs()
}

fn find_nearest_intersection(wire_1 : &Vec<(i32,i32)>, wire_2: &Vec<(i32,i32)>, intersections : &HashSet<&(i32,i32)>) -> i32 {
    let mut nearest : usize = usize::MAX;
    for &crossing in intersections {
        let len_one = wire_1.iter().position(|x| x == crossing).unwrap() + 1;
        let len_two = wire_2.iter().position(|x| x == crossing).unwrap() + 1;
        if len_one + len_two <= nearest {
            nearest = len_one + len_two;
        }
    }
    nearest as i32
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction : Direction,
    amount : i32,
}

impl Move {
    fn from_string(input_string : &str) -> Move {
        let direction = match input_string.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Ai... parse error for a direction")
        };
        let amount : i32 = input_string[1..].parse().ok().unwrap();
        let mv = Move {
            direction,
            amount,
        };
        mv
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{parse_input, calc_manhattan, find_nearest_intersection, Move, generate_path};

    #[test]
    fn test_manhattan() {
        assert_eq!(calc_manhattan(&(3,3)), 6);
    }

    #[test]
    fn test_one() {
        let input : String = String::from("R8,U5,L5,D3\nU7,R6,D4,L4");
        let wires : Vec<Vec<Move>> = parse_input(&input);

        let wire_1 = generate_path(&wires[0]);
        let wire_2 = generate_path(&wires[1]);
        let set_1: HashSet<(i32,i32)> = wire_1.iter().copied().collect();
        let set_2: HashSet<(i32,i32)> = wire_2.iter().copied().collect();

        let intersections: HashSet<_> = set_1.intersection(&set_2).collect();

        let smallest = intersections.iter()
            .map(|coordinate| calc_manhattan(coordinate))
            .min().expect("Expected at least one intersection");

        let nearest = find_nearest_intersection(&wire_1, &wire_2, &intersections);
        assert_eq!(wire_1.len(), 21);
        assert_eq!(wire_2.len(), 21);
        assert_eq!(intersections.len(), 2);
        assert_eq!(smallest, 6);
        assert_eq!(nearest, 30);
    }

    #[test]
    fn test_two() {
        let input : String = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let wires : Vec<Vec<Move>> = parse_input(&input);

        let wire_1 = generate_path(&wires[0]);
        let wire_2 = generate_path(&wires[1]);
        let set_1: HashSet<(i32,i32)> = wire_1.iter().copied().collect();
        let set_2: HashSet<(i32,i32)> = wire_2.iter().copied().collect();

        let intersections: HashSet<_> = set_1.intersection(&set_2).collect();

        let smallest = intersections.iter()
            .map(|coordinate| calc_manhattan(coordinate))
            .min().expect("Expected at least one intersection");
        let nearest = find_nearest_intersection(&wire_1, &wire_2, &intersections);
        assert_eq!(smallest, 159);
        assert_eq!(nearest, 610);
    }

    #[test]
    fn test_three() {
        let input : String = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let wires : Vec<Vec<Move>> = parse_input(&input);

        let wire_1 = generate_path(&wires[0]);
        let wire_2 = generate_path(&wires[1]);
        let set_1: HashSet<(i32,i32)> = wire_1.iter().copied().collect();
        let set_2: HashSet<(i32,i32)> = wire_2.iter().copied().collect();

        let intersections: HashSet<_> = set_1.intersection(&set_2).collect();

        let smallest = intersections.iter()
            .map(|coordinate| calc_manhattan(coordinate))
            .min().expect("Expected at least one intersection");
        let nearest = find_nearest_intersection(&wire_1, &wire_2, &intersections);
        assert_eq!(smallest, 135);
        assert_eq!(nearest, 410);
    }
}