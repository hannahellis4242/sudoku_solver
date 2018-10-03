extern crate futures;
extern crate hyper;
extern crate itertools;
#[macro_use]
extern crate serde_json;
extern crate valico;

use hyper::server::{Request, Response, Service};
use hyper::Method::Get;
use hyper::{Chunk, StatusCode};

use futures::future::{Future, FutureResult};
use futures::Stream;

mod sudoku;

mod json_to_sudoku {
    use serde_json;
    fn parse_value(s: &str) -> Option<char> {
        s.chars()
            .fold(None, |acc, x| if acc.is_none() { Some(x) } else { acc })
            .and_then(|x| if x == '-' { None } else { Some(x) })
    }
    pub fn validate_json(j: serde_json::Value) -> Result<serde_json::Value, String> {
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
    pub fn parse(j: serde_json::Value) -> Result<sudoku::Problem, String> {
        j["grid"]["height"]
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
            }).map(
                |(height, width, square, grid_values, values)| sudoku::Problem {
                    grid: sudoku::GridInfo {
                        height: height,
                        width: width,
                        square: square,
                        values: grid_values,
                    },
                    values: values,
                },
            ).ok_or("could not parse".to_string())
    }
    pub fn validate_problem(p: sudoku::Problem) -> Result<sudoku::Problem, String> {
        //need to ensure that the given values is complete
        let size = p.grid.width * p.grid.height;
        if p.values.len() == size {
            Ok(p)
        } else {
            Err(format!("Given grid information spesifies a grid of {} by {} requiring {} values, number of values given is {}.",p.grid.width,p.grid.height,size,p.values.len()))
        }
    }
}

fn parse_form(form_chunk: Chunk) -> FutureResult<String, hyper::Error> {
    use serde_json::Value;
    match serde_json::from_slice::<Value>(&form_chunk)
        .map_err(|x| format!("{}", x))
        .and_then(json_to_sudoku::validate_json)
        .and_then(json_to_sudoku::parse)
        .and_then(json_to_sudoku::validate_problem)
        .map(sudoku::solve)
        .and_then(move |solutions| match serde_json::to_string(&solutions)
        {
            Ok(v)=>Ok(v),
            Err(e) => Err(format!("{}",e)),

        })
    {
        Ok(result) => futures::future::ok(result),
        Err(e) => futures::future::ok(e),
    }
}

fn make_error_response(error_message: &str) -> FutureResult<hyper::Response, hyper::Error> {
    use hyper::header::ContentLength;
    use hyper::header::ContentType;
    let payload = json!({ "error": error_message }).to_string();
    let response = Response::new()
        .with_status(StatusCode::InternalServerError)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    futures::future::ok(response)
}

fn make_post_response(
    result: Result<String, hyper::Error>,
) -> FutureResult<hyper::Response, hyper::Error> {
    use hyper::header::ContentLength;
    use hyper::header::ContentType;
    use std::error::Error;
    match result {
        Ok(payload) => {
            let response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);
            futures::future::ok(response)
        }
        Err(error) => make_error_response(error.description()),
    }
}

struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        match (request.method(), request.path()) {
            (&Get, "/") => {
                let future = request
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .then(make_post_response);
                Box::new(future)
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn main() {
    let _s = "127.0.0.1:8080".parse().map(|address| {
        hyper::server::Http::new()
            .bind(&address, || Ok(Microservice {}))
            .map(|server| {
                println!("Running microservice at {}", address);
                server.run().unwrap();
            })
    });
}
