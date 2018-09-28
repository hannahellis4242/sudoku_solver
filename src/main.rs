extern crate clap;
extern crate itertools;
extern crate serde_json;
mod sudoku;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

trait ReadToString {
    fn read_to_str(&mut self) -> Result<String, std::io::Error>;
}

impl ReadToString for std::fs::File {
    fn read_to_str(&mut self) -> Result<String, std::io::Error> {
        let mut x = String::new();
        match self.read_to_string(&mut x) {
            Ok(_) => Ok(x),
            Err(e) => Err(e),
        }
    }
}

fn read_file(filename: &str) -> Option<String> {
    match File::open(filename).and_then(|mut x| x.read_to_str()) {
        Ok(x) => Some(x),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn parse_value(s: &str) -> Option<char> {
    s.chars()
        .fold(None, |acc, x| if acc.is_none() { Some(x) } else { acc })
        .and_then(|x| if x == '-' { None } else { Some(x) })
}

fn json_to_sudoku(j: &serde_json::Value) -> sudoku::Problem {
    let g = sudoku::GridInfo {
        height: j["grid"]["height"].as_u64().map(|x| x as usize).unwrap(),
        width: j["grid"]["width"].as_u64().map(|x| x as usize).unwrap(),
        square: j["grid"]["square"].as_u64().map(|x| x as usize).unwrap(),
        values: j["grid"]["values"]
            .as_array()
            .map(|x| {
                x.iter()
                    .filter_map(|y| y.as_str().and_then(parse_value))
                    .collect::<Vec<char>>()
            }).unwrap(),
    };
    let values = j["values"]
        .as_array()
        .map(|x| {
            x.iter()
                .map(|y| y.as_str().and_then(parse_value))
                .collect::<Vec<Option<char>>>()
        }).unwrap();
    sudoku::Problem {
        grid: g,
        values: values,
    }
}

fn main() {
    let matches = App::new("Sudoku solver")
        .version("1.0")
        .author("Hannah")
        .about("solves sudoku puzzles")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        ).get_matches();
    matches
        .value_of("INPUT")
        .and_then(read_file)
        .and_then(|x| match serde_json::from_str(&x) {
            Ok(v) => Some(v),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }).map(|x: serde_json::Value| {
            json_to_sudoku(&x)
        }).map(|problem| sudoku::solve(&problem))
        .map(move |solutions| println!("{:?}",solutions));
}
