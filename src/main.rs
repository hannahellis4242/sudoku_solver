extern crate clap;
extern crate itertools;
extern crate serde_json;
extern crate valico;
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

mod json_to_sudoku {
    use serde_json;
    fn parse_value(s: &str) -> Option<char> {
        s.chars()
            .fold(None, |acc, x| if acc.is_none() { Some(x) } else { acc })
            .and_then(|x| if x == '-' { None } else { Some(x) })
    }
    pub fn validate_json(j: &serde_json::Value) -> Result<serde_json::Value, String> {
        use valico::json_dsl;
        let params = json_dsl::Builder::build(|params| {
            params.req_nested("grid", json_dsl::object(), |params| {
                params.req_typed("height", json_dsl::u64());
                params.req_typed("square", json_dsl::u64());
                params.req_typed("values", json_dsl::array_of(json_dsl::string()));
                params.req_typed("width", json_dsl::u64())
            });
            params.req_typed("values", json_dsl::array_of(json_dsl::string()))
        });
        let mut obj = j.clone();
        let state = params.process(&mut obj, &None);
        if state.is_valid() {
            Ok(obj)
        } else {
            Err(format!("{:?}", state))
        }
    }
    use sudoku;
    pub fn parse(j: &serde_json::Value) -> Option<sudoku::Problem> {
        let inputs = j["grid"]["height"]
            .as_u64()
            .map(|x| x as usize)
            .and_then(|height| {
                j["grid"]["width"]
                    .as_u64()
                    .map(|x| x as usize)
                    .map(|width| (height, width))
            }).and_then(|(height, width)| {
                j["grid"]["square"]
                    .as_u64()
                    .map(|x| x as usize)
                    .map(|square| (height, width, square))
            }).and_then(|(height, width, square)| {
                j["grid"]["values"]
                    .as_array()
                    .map(|x| {
                        x.iter()
                            .filter_map(|y| y.as_str().and_then(parse_value))
                            .collect::<Vec<char>>()
                    }).map(|grid_values| (height, width, square, grid_values))
            }).and_then(|(height, width, square, grid_values)| {
                j["values"]
                    .as_array()
                    .map(|x| {
                        x.iter()
                            .map(|y| y.as_str().and_then(parse_value))
                            .collect::<Vec<Option<char>>>()
                    }).map(|values| (height, width, square, grid_values, values))
            });
        inputs.map(
            |(height, width, square, grid_values, values)| sudoku::Problem {
                grid: sudoku::GridInfo {
                    height: height,
                    width: width,
                    square: square,
                    values: grid_values,
                },
                values: values,
            },
        )
    }
    pub fn validate_problem(p: &sudoku::Problem) -> Result<sudoku::Problem, String> {
        //need to ensure that the given values is complete
        let size = p.grid.width * p.grid.height;
        if p.values.len() == size {
            Ok((*p).clone())
        } else {
            Err(format!("Given grid information spesifies a grid of {} by {} requiring {} values, number of values given is {}.",p.grid.width,p.grid.height,size,p.values.len()))
        }
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
        }).and_then(|x| match json_to_sudoku::validate_json(&x) {
            Ok(v) => Some(v),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }).and_then(|x| json_to_sudoku::parse(&x))
        .and_then(|x| match json_to_sudoku::validate_problem(&x) {
            Ok(v) => Some(v),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }).map(|problem| sudoku::solve(&problem))
        .map(move |solutions| println!("{:?}", solutions));
}
