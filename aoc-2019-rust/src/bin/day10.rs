use std::cmp::Ordering;
use std::time::Instant;
use itertools::Itertools;
use aoc_2019_rust::read_input;

type Coordinate = (usize, usize);
type Vector = (i32, i32);
type NormalVector = (f32, f32);

const NUM_DEC_ROUND : f32 = 100000.0;

fn round_vec(vec: NormalVector) -> NormalVector {
    let x = (vec.0 * NUM_DEC_ROUND).round() / NUM_DEC_ROUND;
    let y = (vec.1 * NUM_DEC_ROUND).round() / NUM_DEC_ROUND;
    (x,y)
}

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
    let visibles: Vec<usize> = map.iter()
        .map(|coord| map.iter()
            .map(|other| distance_vector(*coord, *other))
            .filter(|me| me != &(0,0))
            .map(|vector| normalize_vector(&vector))
            .map(|v| round_vec(v))
            .sorted_by(|a, b| {
                let len_a = (a.0.powi(2) + a.1.powi(2)).sqrt();
                let len_b = (b.0.powi(2) + b.1.powi(2)).sqrt();
                if len_a < len_b {
                    Ordering::Less
                } else if a.0 == b.0 && a.1 == b.1 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }).dedup()
            .collect::<Vec<NormalVector>>())
        .map(|normals| normals.len())
        .collect();
    visibles
}

fn highest_visible(visibles: Vec<usize>) -> usize {
    let aa = visibles.iter().sorted().rev().collect::<Vec<&usize>>();
    **(aa.first().unwrap())
}

fn distance_vector(a: Coordinate, b: Coordinate) -> Vector {
    let a_x = a.0 as i32;
    let a_y = a.1 as i32;
    let b_x = b.0 as i32;
    let b_y = b.1 as i32;

    (b_x - a_x, b_y - a_y)
}

fn normalize_vector(vector: &Vector) -> NormalVector {
    let determinant = ((vector.0.pow(2) + vector.1.pow(2)) as f32).sqrt();
    let x = vector.0 as f32/determinant;
    let y = vector.1 as f32/determinant;
    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::{distance_vector, highest_visible, normalize_vector, parse_map, visibles_per_asteroid};

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
    fn test_distance_vector_q1() {
        let a = (2usize, 2usize);
        let b = (0usize, 0usize);
        let dist = distance_vector(a, b);
        assert_eq!(dist, (-2,-2));
    }

    #[test]
    fn test_distance_vector_q2() {
        let a = (2usize, 2usize);
        let b = (4usize, 0usize);
        let dist = distance_vector(a, b);
        assert_eq!(dist, (2,-2));
    }

    #[test]
    fn test_distance_vector_q3() {
        let a = (0usize, 0usize);
        let b = (2usize, 2usize);
        let dist = distance_vector(a, b);
        assert_eq!(dist, (2,2));
    }

    #[test]
    fn test_distance_vector_q4() {
        let a = (2usize, 2usize);
        let b = (0usize, 4usize);
        let dist = distance_vector(a, b);
        assert_eq!(dist, (-2,2));
    }

    #[test]
    fn test_normalize_vector() {
        let normal_1 = normalize_vector(&(2,2));
        assert_eq!(normal_1, ( 2f32.sqrt()/2f32 , 2f32.sqrt()/2f32 ));

        let normal_2 = normalize_vector(&(4,4));
        assert_eq!(normal_2, ( 2f32.sqrt()/2f32 , 2f32.sqrt()/2f32 ));

        assert_eq!(normal_1, normal_2);
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