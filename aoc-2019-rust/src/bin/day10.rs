use std::time::Instant;
use itertools::Itertools;
use aoc_2019_rust::read_input;

type Coordinate = (usize, usize);

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
    let input = read_input("inputs/day10.txt");
    let map: Vec<(usize, usize)> = parse_map(&input);
    let visibles = visibles_per_asteroid(map);
    let highest = highest_visible(visibles);
    println!("{highest}");
}

fn part2() {
    // let input = read_input("inputs/day9.txt");
}

fn parse_map(input: &str) -> Vec<Coordinate> {
    input.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, row)|
            row.chars().enumerate()
                .filter(|(_, ch)|ch == &'#')
                .map(|(x, _)| (x,y))
                .collect::<Vec<(usize,usize)>>()
        ).flatten().collect()
}

fn visibles_per_asteroid(map: Vec<Coordinate>) -> Vec<usize> {
    let count = map.iter().map(|coordinate| map.iter()
        .filter(|&other| coordinate != other )
        .map(|other| angle(&coordinate, &other))
        .sorted_by(f32::total_cmp)
        .dedup().collect::<Vec<f32>>())
        .map(|asteroids| asteroids.len())
        .collect::<Vec<usize>>();
    count
}

fn highest_visible(visibles: Vec<usize>) -> usize {
    let aa = visibles.iter().sorted().rev().collect::<Vec<&usize>>();
    **(aa.first().unwrap())
}

fn angle(a: &Coordinate, b: &Coordinate) -> f32 {
    let x = b.0 as f32 - a.0 as f32;
    let y = b.1 as f32 - a.1 as f32;
    let degrees = y.atan2(x).to_degrees() + 90.0;

    if degrees < 0.0 {
        degrees + 360.0
    } else {
        degrees
    }
}

#[cfg(test)]
mod tests {
    use crate::{angle, highest_visible, parse_map, visibles_per_asteroid};

    #[test]
    fn test_parse_map() {
        let input = r"
                .#..#
                .....
                #####
                ....#
                ...##
            ";
        let map = parse_map(input);
        let expected = vec![(1,0),(4,0),(0,2),(1,2),(2,2),(3,2),(4,2),(4,3),(3,4),(4,4)];
        assert_eq!(map, expected);
    }

    #[test]
    fn test_angle_q1() {
        let a = (2usize, 2usize);
        let b = (0usize, 0usize);
        let angle = angle(&a, &b);
        assert_eq!(angle, 315.0);
    }

    #[test]
    fn test_angle_q2() {
        let a = (2usize, 2usize);
        let b = (4usize, 0usize);
        let angle = angle(&a, &b);
        assert_eq!(angle, 45.0);
    }

    #[test]
    fn test_angle_q3() {
        let a = (2usize, 2usize);
        let b = (4usize, 4usize);
        let angle = angle(&a, &b);
        assert_eq!(angle, 135.0);
    }

    #[test]
    fn test_angle_q4() {
        let a = (2usize, 2usize);
        let b = (0usize, 4usize);
        let angle = angle(&a, &b);
        assert_eq!(angle, 225.0);
    }

    #[test]
    fn test_angle_self() {
        let a = (2usize, 2usize);
        let b = (2usize, 2usize);
        let angle = angle(&a, &b);
        assert_eq!(angle, 90.0);
    }

    #[test]
    fn test_example_1() {
        let input = r"
                .#..#
                .....
                #####
                ....#
                ...##
            ";
        let map = parse_map(input);
        let visibles = visibles_per_asteroid(map);
        assert_eq!(visibles, [7, 7, 6, 7, 7, 7, 5, 7, 8, 7]);
        let highest = highest_visible(visibles);
        assert_eq!(highest, 8);
    }

    #[test]
    fn test_larger_example_1() {
        let input = r"
                ......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####
            ";
        let map = parse_map(input);
        let visibles = visibles_per_asteroid(map);
        let highest = highest_visible(visibles);
        assert_eq!(highest, 33);
    }

    #[test]
    fn test_larger_example_2() {
        let input = r"
                #.#...#.#.
                .###....#.
                .#....#...
                ##.#.#.#.#
                ....#.#.#.
                .##..###.#
                ..#...##..
                ..##....##
                ......#...
                .####.###.
            ";
        let map = parse_map(input);
        let visibles = visibles_per_asteroid(map);
        let highest = highest_visible(visibles);
        assert_eq!(highest, 35);
    }

    #[test]
    fn test_larger_example_3() {
        let input = r"
                .#..#..###
                ####.###.#
                ....###.#.
                ..###.##.#
                ##.##.#.#.
                ....###..#
                ..#.#..#.#
                #..#.#.###
                .##...##.#
                .....#.#..
            ";
        let map = parse_map(input);
        let visibles = visibles_per_asteroid(map);
        let highest = highest_visible(visibles);
        assert_eq!(highest, 41);
    }

    #[test]
    fn test_larger_example_4() {
        let input = r"
                .#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##
            ";
        let map = parse_map(input);
        let visibles = visibles_per_asteroid(map);
        let highest = highest_visible(visibles);
        assert_eq!(highest, 210);
    }
}