extern crate conv;
extern crate serde_json;
extern crate valico;

fn json_error_to_str(e: serde_json::error::Error) -> String {
    use sudoku::json::serde_json::error::Category;
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
    use sudoku::json::valico::json_dsl;
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
    use sudoku::json::serde_json::Value;
    serde_json::from_str::<Value>(&json_text)
        .map_err(json_error_to_str)
        .and_then(validate_json)
        .and_then(parse)
        .and_then(validate_problem)
}

fn char_to_json(cs: &char) -> sudoku::json::serde_json::Value {
    sudoku::json::serde_json::Value::String(cs.to_string())
}

fn usize_to_json(u: usize) -> Option<sudoku::json::serde_json::Value> {
    use sudoku::json::conv::*;
    f64::value_from(u)
        .ok()
        .and_then(|f| sudoku::json::serde_json::Number::from_f64(f))
        .map(|n| sudoku::json::serde_json::Value::Number(n))
}

fn grid_to_json(g: &sudoku::GridInfo) -> Option<sudoku::json::serde_json::Value> {
    Some(sudoku::json::serde_json::Map::new())
        .and_then(|mut map| {
            usize_to_json(g.width).map(|value| {
                map.insert(String::from("width"), value);
                map
            })
        })
        .and_then(|mut map| {
            usize_to_json(g.height).map(|value| {
                map.insert(String::from("height"), value);
                map
            })
        })
        .and_then(|mut map| {
            usize_to_json(g.square).map(|value| {
                map.insert(String::from("square"), value);
                map
            })
        })
        .map(|mut map| {
            map.insert(
                String::from("values"),
                sudoku::json::serde_json::Value::Array(
                    g.values
                        .iter()
                        .map(char_to_json)
                        .collect::<Vec<sudoku::json::serde_json::Value>>(),
                ),
            );
            map
        })
        .map(|map| sudoku::json::serde_json::Value::Object(map))
}

fn solution_to_json(
    problem: &sudoku::Problem,
    solution: &[char],
) -> Option<sudoku::json::serde_json::Value> {
    Some(sudoku::json::serde_json::Map::new())
        .and_then(|mut map| {
            grid_to_json(&problem.grid).map(|value| {
                map.insert(String::from("grid"), value);
                map
            })
        })
        .map(|mut map| {
            map.insert(
                String::from("values"),
                sudoku::json::serde_json::Value::Array(
                    solution
                        .iter()
                        .map(char_to_json)
                        .collect::<Vec<sudoku::json::serde_json::Value>>(),
                ),
            );
            map
        })
        .map(|map| sudoku::json::serde_json::Value::Object(map))
}
pub fn write_solution(p: &sudoku::Problem, s: &[char]) -> String {
    solution_to_json(p, s)
        .map(|json| json.to_string())
        .unwrap_or(String::new())
}
