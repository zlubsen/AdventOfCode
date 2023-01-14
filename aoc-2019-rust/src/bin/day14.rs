use std::collections::HashMap;
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
    let input = read_input("inputs/day14.txt");
    let reactions = parse_reactions(&input);
    // print(&reactions);
    let mut inventory: HashMap<String, usize> = HashMap::new();
    let cost = solve_cost("FUEL", 1, &mut inventory, &reactions);
    println!("{cost}");
}

fn part2() {
    // let input = read_input("inputs/day14.txt");
}

type Recipe = Vec<Ingredient>;

struct Ingredient {
    chemical: String,
    amount: usize,
}

impl Ingredient {
    fn new(chemical: String, amount: usize) -> Self {
        Self {
            chemical,
            amount,
        }
    }
}

struct Reaction {
    recipe: Recipe,
    amount: usize,
}

impl Reaction {
    fn new(recipe: Recipe, amount: usize) -> Self {
        Self {
            recipe,
            amount,
        }
    }
}

fn parse_reactions(input: &str) -> HashMap<String, Reaction> {
    let out = input.split('\n').collect::<Vec<&str>>().iter()
        .map(|&line| line.trim())
        // .inspect(|&line| println!("{line}"))
        .filter(|&line| !line.is_empty())
        .map(|line| line.split(" => ").collect::<Vec<&str>>() )
        .map(|reaction| {
            let ingredients = *reaction.first().unwrap();
            let recipe: Recipe = ingredients.split(", ").collect::<Vec<&str>>().iter()
                .map(|&ingredient| {
                    let ingredient = ingredient.split(" ").collect::<Vec<&str>>();
                    let amount : usize = ingredient.first().unwrap().parse().unwrap();
                    let chemical = ingredient.last().unwrap().to_string();
                    Ingredient::new(chemical, amount)
                }).collect();

            let product = *reaction.last().unwrap();
            let product: Vec<&str> = product.split(" ").collect();
            let amount : usize = product.first().unwrap().parse().unwrap();
            let product_name = product.last().unwrap().to_string();
            let production = (product_name, Reaction::new(recipe, amount));
            production
        }).fold(HashMap::new(), | mut acc, production | {
            acc.insert(production.0, production.1);
            acc
        });
    out
}

#[allow(dead_code)]
fn print(map: &HashMap<String, Reaction>) {
    map.iter().for_each(|rule| {
        let reaction = rule.1.recipe.iter().map(|item| format!("{} {}",item.amount, item.chemical) ).join(", ");
        println!("{} => {} {}", reaction, rule.1.amount, rule.0);
    });
}

fn solve_cost(chemical: &str, amount: usize, inventory: &mut HashMap<String, usize>, rules: &HashMap<String, Reaction>) -> usize {
    if chemical == "ORE" {
        amount
    } else {
        let inventory_amount = *inventory.get(chemical).unwrap_or(&0usize);
        let needed_amount = if inventory_amount > 0 {
            inventory.entry(chemical.to_string()).and_modify(|value| {
                *value = inventory_amount.saturating_sub(amount);
            });
            amount.saturating_sub(inventory_amount)
        } else { amount };

        if needed_amount > 0 {
            let recipe = rules.get(chemical).unwrap();
            let iterations = (needed_amount as f64 / recipe.amount as f64).ceil() as usize;
            let actual_production = recipe.amount * iterations;
            if needed_amount < actual_production {
                let remainder = actual_production - needed_amount;
                inventory.entry(chemical.to_string())
                    .and_modify(|value| *value += remainder )
                    .or_insert(remainder);
            }
            recipe.recipe.iter()
                .map(|ingredient | solve_cost(&ingredient.chemical, ingredient.amount * iterations, inventory, rules) )
                .sum()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{parse_reactions, solve_cost};

    #[test]
    fn test_example_1() {
        let input = r"
            10 ORE => 10 A
            1 ORE => 1 B
            7 A, 1 B => 1 C
            7 A, 1 C => 1 D
            7 A, 1 D => 1 E
            7 A, 1 E => 1 FUEL
        ";
        let reactions = parse_reactions(input);
        let mut inventory: HashMap<String, usize> = HashMap::new();
        let cost = solve_cost("FUEL", 1, &mut inventory, &reactions);
        assert_eq!(cost, 31);
    }

    #[test]
    fn test_example_2() {
        let input = r"
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ";
        let reactions = parse_reactions(input);
        let mut inventory: HashMap<String, usize> = HashMap::new();
        let cost = solve_cost("FUEL", 1, &mut inventory, &reactions);
        assert_eq!(cost, 165);
    }
}