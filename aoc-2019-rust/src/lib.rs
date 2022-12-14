use std::{env, fs};
use std::path::Path;

pub mod intcode;

pub fn read_arg() -> String {
    read_input(&get_first_arg())
}

pub fn get_first_arg() -> String {
    let args : Vec<String> = env::args().collect();
    args[1].clone()
}

pub fn read_input(path: &str) -> String {
    fs::read_to_string(Path::new(path)).expect("Could not read input file {path}")
}