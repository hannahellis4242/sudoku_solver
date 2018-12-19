extern crate clap;
extern crate itertools;
extern crate serde_json;
extern crate valico;

mod sudoku;

/*mod sudoku_json {
    use serde_json;
    fn json_error_to_str(e: serde_json::error::Error) -> String {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => format!("failed to read bytes into IO stream"),
            Category::Syntax => format!("syntax error at line {} column {}", e.line(), e.column()),
            Category::Data => format!("input data that is semantically incorrect"),
            Category::Eof => format!("unexpected end of the input data"),
        }
    }
    fn parse_value(s: &str) -> Option<char> {
        s.chars()
            .fold(None, |acc, x| if acc.is_none() { Some(x) } else { acc })
            .and_then(|x| if x == '-' { None } else { Some(x) })
    }
    fn validate_json(j: serde_json::Value) -> Result<serde_json::Value, String> {
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
    fn parse(j: serde_json::Value) -> Result<sudoku::Problem, String> {
        j["grid"]["height"]
            .as_u64()
            .map(|x| x as usize)
            .and_then(|height| {
                j["grid"]["width"]
                    .as_u64()
                    .map(|x| x as usize)
                    .map(|width| (height, width))
            })
            .and_then(|(height, width)| {
                j["grid"]["square"]
                    .as_u64()
                    .map(|x| x as usize)
                    .map(|square| (height, width, square))
            })
            .and_then(|(height, width, square)| {
                j["grid"]["values"]
                    .as_array()
                    .map(|x| {
                        x.iter()
                            .filter_map(|y| y.as_str().and_then(parse_value))
                            .collect::<Vec<char>>()
                    })
                    .map(|grid_values| (height, width, square, grid_values))
            })
            .and_then(|(height, width, square, grid_values)| {
                j["values"]
                    .as_array()
                    .map(|x| {
                        x.iter()
                            .map(|y| y.as_str().and_then(parse_value))
                            .collect::<Vec<Option<char>>>()
                    })
                    .map(|values| (height, width, square, grid_values, values))
            })
            .map(
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
            .ok_or("could not parse".to_string())
    }
    fn validate_problem(p: sudoku::Problem) -> Result<sudoku::Problem, String> {
        //need to ensure that the given values is complete
        let size = p.grid.width * p.grid.height;
        if p.values.len() == size {
            Ok(p)
        } else {
            Err(format!("Given grid information spesifies a grid of {} by {} requiring {} values, number of values given is {}.",p.grid.width,p.grid.height,size,p.values.len()))
        }
    }
    pub fn read_problem(json_text: &str) -> Result<sudoku::Problem, String> {
        use serde_json::Value;
        serde_json::from_str::<Value>(&json_text)
            .map_err(json_error_to_str)
            .and_then(validate_json)
            .and_then(parse)
            .and_then(validate_problem)
    }

    fn grid_to_json()
    pub fn write_solution(p:&Problem,s:Vec<char>)->String{

    }
}*/

fn main() {
    use clap::{App, Arg, SubCommand};
    use std::io::ErrorKind;
    match App::new("sudoku")
        .version("0.1.0")
        .author("Hannah Ellis <hannahellis4242@gmail.com>")
        .about("Solves Sudoku problems")
        .subcommand(
            SubCommand::with_name("solve")
                .about("solves the given problem")
                .version("0.1.0")
                .author("Hannah Ellis <hannahellis4242@gmail.com>")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("input")
                        .value_name("FILE")
                        .help("problem file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
        .subcommand()
    {
        ("solve", Some(matches)) => matches
            .value_of("file")
            .ok_or::<std::io::Error>(std::io::Error::new(
                ErrorKind::Other,
                "missing file argument",
            ))
            .and_then(|file_name| std::fs::read_to_string(file_name))
            //.map(|text| println!("{}", text))
            .and_then(|text| {
                sudoku::json::read_problem(&text)
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            })
            .map(|p| (p.clone(), sudoku::solve(p)))
            .map(|(_, solutions)| {
                solutions
                    .iter()
                    .for_each(|solution| println!("{:?}", solution))
            })
            .map_err(|e| println!("{}", e))
            .unwrap_or(()), // solve was used
        _ => println!("{}", "missing subcommand"), // Either no subcommand or one not tested for...
    }
}
