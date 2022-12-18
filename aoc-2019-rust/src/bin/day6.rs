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
    let input = read_input("inputs/day6.txt");
    let orbits = get_orbits(input.as_str());
    let mut index = build_index(&orbits);

    let com_idx = index.index_of("COM").expect("Expected COM to be in the index");
    index.traverse_set_depth(com_idx, 0);
    let total = index.calc_total_depth();
    println!("{total}");
}

fn part2() {
    let input = read_input("inputs/day6.txt");

}

fn get_orbits(input: &str) -> Vec<(&str, &str)> {
     input.split("\n").map(|line| {
        let planets: Vec<&str> = line.split(")").collect();
        (planets[0], planets[1])
    }).collect()
}

fn build_index(orbits: &Vec<(&str, &str)>) -> Index {
    let mut index = Index::new();
    orbits.iter().for_each(|orbit| {
        let id_1 = index.find_or_insert(orbit.0, None);
        let id_2 = index.find_or_insert(orbit.1, Some(id_1));
        index.add_child(id_1, id_2);
    });
    index
}

struct Node {
    name: String,
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    depth: usize,
}

struct Index {
    id_gen: usize,
    items: Vec<Node>
}

impl Index {
    fn new() -> Self {
        Self {
            id_gen : 0,
            items: Vec::new(),
        }
    }

    fn insert(&mut self, name: &str, parent_idx: Option<usize>) -> usize {
        let inserted_id = self.id_gen;
        self.id_gen += 1;
        let node = Node {
            name: name.to_string(),
            id: inserted_id,
            parent: parent_idx,
            children: vec![],
            depth: 0,
        };
        self.items.push(node);
        inserted_id
    }

    fn find_or_insert(&mut self, name: &str, parent_idx: usize) -> usize {
        if let Some(idx) = self.index_of(name) {
            idx
        } else {
            self.insert(name)
        }
    }

    fn index_of(&self, name: &str) -> Option<usize> {
        if let Some(node) = self.items.iter().find(|node| node.name.as_str() == name ) {
            Some(node.id)
        } else {
            None
        }
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        let node: &mut Node = self.items.get_mut(parent).unwrap();
        node.children.push(child);
    }

    fn traverse_set_depth(&mut self, node_id: usize, depth: usize) {
        let children = {
            let node: &mut Node = self.items.get_mut(node_id).unwrap();
            node.depth += depth;
            node.children.clone()
        };
        children.iter().for_each(|&child| {
            let child_idx = child.clone();
            self.traverse_set_depth(child_idx, depth + 1)
        });
    }

    fn calc_total_depth(&self) -> usize {
        let depth: usize = self.items.iter().map(|node| node.depth).sum();
        depth
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_index, get_orbits};

    #[test]
    fn test_total_orbits() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let orbits = get_orbits(input);
        let mut index = build_index(&orbits);
        let com_idx = index.index_of("COM").expect("Expected COM to be in the index");
        index.traverse_set_depth(com_idx, 0);
        let total = index.calc_total_depth();
        assert_eq!(total, 42);
    }

    #[test]
    fn test_orbit_transfers() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let orbits = get_orbits(input);
        let mut index = build_index(&orbits);
        // let com_idx = index.index_of("YOU").expect("Expected COM to be in the index");
        // index.traverse_set_depth(com_idx, 0);
        // let total = index.calc_total_depth();
        assert_eq!(transfers, 4);
    }
}
