use std::time::Instant;
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
    let input = read_input("inputs/day1.txt");
    let total : i32 = input.split("\n").filter_map(|w| w.parse().ok())
        .map(|v: i32| (v / 3) - 2)
        .sum();
    println!("{total}");
}

fn part2() {
    let input = read_input("inputs/day1.txt");
    let total : i32 = input.split("\n").filter_map(|w| w.parse().ok())
        .map(|v : i32| calculate_fuel(&v))
        .sum();
    println!("{total}");
}

fn calculate_fuel(mass : &i32) -> i32 {
    if *mass > 0 {
        let fuel : i32 = *mass / 3 - 2;
        if fuel >= 1 {
            fuel + calculate_fuel(&fuel)
        } else {
            0
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_fuel;

    #[test]
    fn test_14() {
        assert_eq!(calculate_fuel(&14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(calculate_fuel(&1969), 966);
    }

    #[test]
    fn test_100756() {
        assert_eq!(calculate_fuel(&100756), 50346);
    }
}