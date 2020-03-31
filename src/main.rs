#[macro_use]
extern crate log;
extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate regex;

use chrono::Local;
use clap::{App, Arg};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::time::Instant;

use sudoku_solver::{parse_sudokus, solve_sudoku};

fn main() {
    let matches = App::new("Sudoku Solver")
        .version("0.1")
        .author("baurst")
        .about("Fast Sudoku solver.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sudoku input file to use. One problem per line.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let loglevel = match matches.occurrences_of("v") {
        0 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}: {}",
                record.level(),
                Local::now().format("%Y-%m-%d-%H:%M:%S"),
                record.args()
            )
        })
        .filter(None, loglevel)
        .init();

    let prob = matches.value_of("INPUT").unwrap();

    let sudoku_problems = parse_sudokus(prob);

    let num_sudokus = sudoku_problems.len();
    let mut num_unsolvable_sudokus = 0;
    let time_start = Instant::now();

    for prob in sudoku_problems {
        info!("Starting with problem: {}", prob);
        let solution = solve_sudoku(Some(prob), 0);

        if let Some(solved) = solution {
            info!("Problem solved:{}", solved);
        } else {
            warn!("Problem unsolvable!");
            num_unsolvable_sudokus += 1;
        }
    }

    let duration = Instant::now() - time_start;

    info!(
        "{:?} for {} sudokus, on average {:?} per problem.",
        duration,
        num_sudokus,
        duration / num_sudokus as u32
    );

    if num_unsolvable_sudokus > 0 {
        warn!(
            "Failed to solve {} out of {} sudokus.",
            num_unsolvable_sudokus, num_sudokus,
        );
    }
}
