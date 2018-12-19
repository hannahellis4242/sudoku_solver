extern crate clap;

mod sudoku;

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
