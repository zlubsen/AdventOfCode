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
    let cost = solve_cost("FUEL", 1, HashMap::new(), &reactions);
    println!("{cost}");
}

fn part2() {
    let input = read_input("inputs/day14.txt");
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
        .map(|&line| line.split(" => ").collect::<Vec<&str>>() )
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

fn print(map: &HashMap<String, Reaction>) {
    map.iter().for_each(|rule| {
        let reaction = rule.1.recipe.iter().map(|item| format!("{} {}",item.amount, item.chemical) ).join(", ");
        println!("{} => {} {}", reaction, rule.1.amount, rule.0);
    });
}

fn solve_cost(chemical: &str, amount: usize, mut inventory: HashMap<String, usize>, rules: &HashMap<String, Reaction>) -> usize {
    if chemical == "ORE" {
        amount
    } else {
        let inventory_amount = *inventory.get(chemical).unwrap_or(&0usize);
        let needed_amount = if inventory_amount > 0 {
            inventory.entry(chemical.to_string()).and_modify(|value| {
                *value = if amount <= inventory_amount {
                    inventory_amount - amount
                } else { 0 }
            });
            amount - inventory_amount
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
                .map(|ingredient | solve_cost(&ingredient.chemical, ingredient.amount, inventory, rules) )
                .sum()
        } else {
            0
        }
    }
}