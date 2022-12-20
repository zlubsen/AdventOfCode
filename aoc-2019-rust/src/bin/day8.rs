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
    let image = render(input.clone(), 25*6);
    print_image(image, 25);

    // Attempt where we create vec of columns throught the layers. Produced wrong image.
    // let decoded = decode(25,6, input.as_str());
    // let fused = fuse(&decoded);
    // print_image_2(fused);
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

fn render(image: String, layer_size: usize) -> Vec<char> {
    image
        .as_bytes()
        .chunks(layer_size)
        .fold(vec![' '; layer_size], |display, layer| {
            (0..layer_size)
                .map(|i| {
                    if display[i] == ' ' {
                        match layer[i] {
                            48 => '░',
                            49 => '█',
                            50 => ' ',
                            c => panic!("Unrecognized character: {}", c),
                        }
                    }
                    else {
                        display[i]
                    }
                })
                .collect()
        })
}

fn print_image(image: Vec<char>, width: usize) {
    image.chunks(width).for_each(|row| {
        for (i, pix) in row.iter().enumerate() {
            print!("{pix}");
            if (i+1) % width == 0 {
                print!("\n");
            }
        }
    });
}

fn decode(width: usize, height: usize, input: &str) -> Vec<Vec<Vec<char>>> {
    let chars : Vec<char> = input.chars().collect();
    let layer_size = width*height;
    let num_layers = input.len()/layer_size;

    let mut rows: Vec<Vec<Vec<char>>> = Vec::new();
    for row_idx in 0..height {
        let mut cols: Vec<Vec<char>> = Vec::new();
        for col_idx in 0..width {
            let mut layers: Vec<char> = Vec::new();
            for layer_idx in 0..num_layers {
                let index = (layer_idx * (layer_size)) + (row_idx * (height)) + (col_idx);
                let char = chars.get(index).unwrap();
                layers.push(char.clone());
            }
            cols.push(layers);
        }
        rows.push(cols);
    }
    rows
}

fn fuse(rows: &Vec<Vec<Vec<char>>>) -> Vec<Vec<&char>> {
    let mut fused_row = Vec::new();
    for row in rows {
        let mut fused_col = Vec::new();
        for col in row {
            let pixel : Vec<&char> = col.iter().filter(|&&pix| pix != '2').collect();
            let pixel = pixel.first().unwrap();
            fused_col.push(pixel.clone());
        }
        fused_row.push(fused_col);
    }
    fused_row
}

fn print_image_2(image: Vec<Vec<&char>>) {
    image.iter().for_each(|row| {
        let line = row.iter().map(|&char| {
            if char == &'0' {
                ' '
            } else { '#' }
        }).join("");
        println!("{line}");
    });
}

#[cfg(test)]
mod tests {
    use crate::{ones_by_twos, parse_layers, print_image, render};

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
        let image = render(input.to_string(), 2*2);
        assert_eq!(image, vec!['░','█','█','░']);
        print_image(image, 2);
        // let decoded = decode(2,2, input);
        // assert_eq!(decoded, vec![vec![vec!['0', '1', '2', '0'], vec!['2', '1', '2', '0']], vec![vec!['2', '2', '1', '0'], vec!['2', '2', '2', '0']]]);
        // let fused = fuse(&decoded);
        // assert_eq!(fused, vec![vec![&'0', &'1'], vec![&'1', &'0']]);
        // print_image(fused);
    }
}