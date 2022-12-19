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
    let input = read_input("inputs/day8.txt");
    let layers = parse_layers(25 * 6, input.as_str());
    let ones_by_twos = ones_by_twos(&layers);
    println!("{ones_by_twos}");
}

fn part2() {
    let input = read_input("inputs/day8.txt");
    // let layers = parse_layers(25 * 6, input.as_str());

    // let
}

fn parse_layers(layer_size: usize, input: &str) -> Vec<Vec<char>> {
    let num_layers : usize = input.len()/ layer_size;
    let mut layers = vec![];
    for i in 0..num_layers {
        let pixels : Vec<char> = input.chars().skip(i * layer_size).take(layer_size).collect();
        layers.push(pixels);
    }
    layers
}

fn ones_by_twos(layers: &Vec<Vec<char>>) -> usize {
    let counts : Vec<(usize, usize, usize)> = layers.iter()
        .map(|layer| {
            let zeroes = layer.iter()
                .filter(|&pixel| pixel == &'0')
                .count();
            let ones = layer.iter()
                .filter(|&pixel| pixel == &'1')
                .count();
            let twos = layer.iter()
                .filter(|&pixel| pixel == &'2')
                .count();
            (zeroes, ones, twos)
        })
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0)).collect();

    let layer = counts.last().unwrap();
    let ones_by_twos = layer.1 * layer.2;
    ones_by_twos
}

fn decode(width: usize, height: usize, input: &str) -> Vec<Vec<Vec<char>>> {
    let chars : Vec<char> = input.chars().collect();
    let layer_size = width*height;
    let num_layers = input.len()/layer_size;

    let mut layers: Vec<Vec<Vec<char>>> = Vec::new();
    for layer_idx in 0..num_layers {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for row_idx in 0..width {
            let mut cols: Vec<char> = Vec::new();
            for col_idx in 0..height {
                let index = (layer_idx * (layer_size)) + (row_idx * (height)) + (col_idx);
                let char = chars.get(index).unwrap();
                cols.push(char.clone());
            }
            rows.push(cols);
        }
        layers.push(rows);
    }
    layers
}

fn stack(layers: &Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
    layers.iter().map(|layer|  )
}

#[cfg(test)]
mod tests {
    use crate::{decode, ones_by_twos, parse_layers};

    #[test]
    fn test_pt1() {
        let input = "123456789012";
        let layers = parse_layers(3 * 2, input);
        let ones_by_twos = ones_by_twos(&layers);
        assert_eq!(ones_by_twos, 1);
    }

    #[test]
    fn test_pt2() {
        let input = "0222112222120000";
        let decoded = decode(2,2, input);
        assert_eq!(decoded, vec![vec![vec!['0', '2'], vec!['2', '2']], vec![vec!['1', '1'], vec!['2', '2']], vec![vec!['2', '2'], vec!['1', '2']], vec![vec!['0', '0'], vec!['0', '0']]]);
        let stacked = stack(&decoded);
        assert_eq!(stacked, vec![vec!['0', '1'], vec!['1','0']])
    }
}