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
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io::Write;

#[derive(Clone, Debug)]
struct SudokuCandidates {
    grid: Vec<Vec<Vec<u8>>>,
}

impl SudokuCandidates {
    fn initial() -> SudokuCandidates {
        SudokuCandidates {
            grid: vec![vec![(1..10).collect::<Vec<u8>>(); 9]; 9],
        }
    }
    fn from_vec(numbers: Vec<u8>) -> SudokuCandidates {
        assert!(numbers.len() == 81);
        let mut problem = SudokuCandidates::initial();

        for (i, item) in numbers.iter().enumerate() {
            if *item == 0 as u8 {
                continue;
            }
            let row_idx = i / 9;
            let col_idx = i % 9;

            problem.grid[row_idx][col_idx] = vec![*item];
            problem = remove_from_neighbors(&problem, row_idx, col_idx, *item).unwrap();
        }

        problem
    }
}

fn parse_sudoku(filepath: &str) -> Vec<SudokuCandidates> {
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut candidates = vec![];
    for line in lines {
        // replace any non numeric characters in the line with 0
        let re = Regex::new(r"[^0-9]").unwrap();
        let result = re.replace_all(line, "0");
        let problem_raw: Vec<u8> = result
            .split("")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let cand = SudokuCandidates::from_vec(problem_raw);
        candidates.push(cand);
    }
    candidates
}

fn _check_conflict_for_element(problem: &SudokuCandidates, row_idx: usize, col_idx: usize) -> bool {
    // check row
    for col_test_idx in 0..9 {
        if col_test_idx != col_idx
            && problem.grid[row_idx][col_test_idx] == problem.grid[row_idx][col_idx]
        {
            return false;
        }
    }

    // check col
    for row_test_idx in 0..9 {
        if row_idx != row_test_idx
            && problem.grid[row_test_idx][col_idx] == problem.grid[row_idx][col_idx]
        {
            return false;
        }
    }

    // check cell
    let cell_row_idx = row_idx / 3;
    let cell_col_idx = col_idx / 3;

    for row_idx_in_cell in 0..3 {
        for col_idx_in_cell in 0..3 {
            let row_test_idx = cell_row_idx * 3 + row_idx_in_cell;
            let col_test_idx = cell_col_idx * 3 + col_idx_in_cell;
            if row_idx == row_test_idx || col_idx == col_test_idx {
                // we have already elminated the row and colum here above
                continue;
            }
            if problem.grid[row_test_idx][col_test_idx] == problem.grid[row_idx][col_idx] {
                return false;
            }
        }
    }
    true
}

fn remove_from_neighbors(
    problem: &SudokuCandidates,
    el_row_idx: usize,
    el_col_idx: usize,
    el: u8,
) -> Option<SudokuCandidates> {
    //debug!("removing {} {} {}", el_row_idx, el_col_idx, el );
    // clean row
    let mut problem = problem.clone();
    for col_idx in 0..9 {
        // if col_idx == 6 && el_row_idx == 8 && el == 2{
        //     debug!("{}", problem);
        //     debug!("{:?}", problem.grid[el_row_idx][col_idx]);
        // }
        if col_idx == el_col_idx {
            continue;
        }
        let dupl_idx = problem.grid[el_row_idx][col_idx]
            .iter()
            .position(|x| *x == el);
        if let Some(x) = dupl_idx {
            problem.grid[el_row_idx][col_idx].remove(x);
            if problem.grid[el_row_idx][col_idx].is_empty() {
                return None; // conflict detected
            } else if problem.grid[el_row_idx][col_idx].len() == 1 {
                let new_prob_opt = remove_from_neighbors(
                    &problem,
                    el_row_idx,
                    col_idx,
                    problem.grid[el_row_idx][col_idx][0],
                );
                if let Some(new_prob) = new_prob_opt {
                    problem = new_prob;
                } else {
                    return None;
                }
                //return check_conflict_for_element(problem, el_row_idx, col_idx);
            }
        }
    }

    // clean col
    for row_idx in 0..9 {
        if row_idx == el_row_idx {
            continue;
        }
        let dupl_idx = problem.grid[row_idx][el_col_idx]
            .iter()
            .position(|x| *x == el);
        if let Some(x) = dupl_idx {
            problem.grid[row_idx][el_col_idx].remove(x);
            if problem.grid[row_idx][el_col_idx].is_empty() {
                return None; // conflict detected
            } else if problem.grid[row_idx][el_col_idx].len() == 1 {
                let new_prob_opt = remove_from_neighbors(
                    &problem,
                    row_idx,
                    el_col_idx,
                    problem.grid[row_idx][el_col_idx][0],
                );
                if let Some(new_prob) = new_prob_opt {
                    problem = new_prob;
                } else {
                    return None;
                }
            }
        }
    }

    // clean cell
    let cell_row_idx = el_row_idx / 3;
    let cell_col_idx = el_col_idx / 3;

    for row_idx_in_cell in 0..3 {
        for col_idx_in_cell in 0..3 {
            let row_idx = cell_row_idx * 3 + row_idx_in_cell;
            let col_idx = cell_col_idx * 3 + col_idx_in_cell;
            if row_idx == el_row_idx || col_idx == el_col_idx {
                // we have already elminated the row and colum here above
                continue;
            }
            let dupl_idx = problem.grid[row_idx][col_idx].iter().position(|x| *x == el);
            if let Some(x) = dupl_idx {
                problem.grid[row_idx][col_idx].remove(x);
                if problem.grid[row_idx][col_idx].is_empty() {
                    return None; // conflict detected
                } else if problem.grid[row_idx][col_idx].len() == 1 {
                    let new_prob_opt = remove_from_neighbors(
                        &problem,
                        row_idx,
                        col_idx,
                        problem.grid[row_idx][col_idx][0],
                    );
                    if let Some(new_prob) = new_prob_opt {
                        problem = new_prob;
                    } else {
                        return None;
                    }
                    //return check_conflict_for_element(problem, row_idx, col_idx);
                }
            }
        }
    }
    Some(problem)
}

impl std::fmt::Display for SudokuCandidates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut some_str = "".to_string();
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                let cand_str = self.grid[row_idx][col_idx]
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<String>();
                let sym = format!("{: >9},", cand_str);
                some_str.push_str(&sym);
            }
            some_str.push_str(&"\n".to_string());
        }

        write!(f, "\n{}", some_str)
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn solution_has_unresolvable_conflicts(solution: &SudokuCandidates) -> bool {
    // rows okay
    for row_idx in 0..9 {
        let mut elems = vec![];
        for col_idx in 0..9 {
            if solution.grid[row_idx][col_idx].len() == 1 {
                elems.push(solution.grid[row_idx][col_idx][0]);
            }
        }
        if !has_unique_elements(elems) {
            debug!("Unresolvable conflict at row: {}", row_idx);
            return true;
        }
    }

    // cols okay
    for col_idx in 0..9 {
        let mut elems = vec![];
        for row_idx in 0..9 {
            if solution.grid[row_idx][col_idx].len() == 1 {
                elems.push(solution.grid[row_idx][col_idx][0]);
            }
        }
        if !has_unique_elements(elems) {
            debug!("Unresolvable conflict at col: {}", col_idx);
            return true;
        }
    }

    // cells okay
    for meta_col_idx in 0..3 {
        for meta_row_idx in 0..3 {
            let mut cell_elems = vec![];
            for col_idx_in_cell in 0..3 {
                for row_idx_in_cell in 0..3 {
                    let row_idx = meta_row_idx * 3 + row_idx_in_cell;
                    let col_idx = meta_col_idx * 3 + col_idx_in_cell;
                    if solution.grid[row_idx][col_idx].len() == 1 {
                        cell_elems.push(solution.grid[row_idx][col_idx][0]);
                    }
                }
            }
            if !has_unique_elements(cell_elems) {
                debug!(
                    "Unresolvable conflict at cell: meta_row_idx {} meta_col_idx {}",
                    meta_row_idx, meta_col_idx
                );
                return true;
            }
        }
    }

    false
}

fn solution_is_correct(solution: &SudokuCandidates) -> bool {
    // only one element per cell
    for row in &solution.grid {
        for col in row {
            if col.len() != 1 {
                return false;
            }
        }
    }
    for col_idx in 0..9 {
        let mut sum_col_elems = 0;
        for row_idx in 0..9 {
            sum_col_elems += solution.grid[row_idx][col_idx][0];
        }
        if sum_col_elems != 9 * 10 / 2 {
            return false;
        }
    }
    for row_idx in 0..9 {
        let mut sum_row_elems = 0;
        for col_idx in 0..9 {
            sum_row_elems += solution.grid[row_idx][col_idx][0];
        }
        if sum_row_elems != 9 * 10 / 2 {
            return false;
        }
    }
    true
}

#[derive(Clone, Debug)]
struct InsertionCandidate {
    row_idx: usize,
    col_idx: usize,
    candidates: Vec<u8>,
}

fn single_element_in_col(
    problem: &SudokuCandidates,
    row_idx: usize,
    col_idx: usize,
    el: u8,
) -> bool {
    for row_inner_idx in 0..9 {
        if row_idx == row_inner_idx {
            continue;
        }
        if problem.grid[row_inner_idx][col_idx]
            .iter()
            .any(|x| x == &el)
        {
            return false;
        }
    }
    true
}

fn single_element_in_row(
    problem: &SudokuCandidates,
    row_idx: usize,
    col_idx: usize,
    el: u8,
) -> bool {
    for col_inner_idx in 0..9 {
        if col_idx == col_inner_idx {
            continue;
        }
        if problem.grid[row_idx][col_inner_idx]
            .iter()
            .any(|x| x == &el)
        {
            return false;
        }
    }
    true
}

fn single_element_in_cell(
    problem: &SudokuCandidates,
    row_idx: usize,
    col_idx: usize,
    el: u8,
) -> bool {
    let cell_row_idx = row_idx / 3;
    let cell_col_idx = col_idx / 3;

    for row_idx_in_cell in (cell_row_idx * 3)..((cell_row_idx + 1) * 3) {
        for col_idx_in_cell in (cell_col_idx * 3)..((cell_col_idx + 1) * 3) {
            if row_idx == row_idx_in_cell && col_idx == col_idx_in_cell {
                continue;
            }
            if problem.grid[row_idx_in_cell][col_idx_in_cell]
                .iter()
                .any(|x| x == &el)
            {
                return false;
            }
        }
    }
    true
}

fn get_best_place_and_number_to_insert(problem: &SudokuCandidates) -> Option<InsertionCandidate> {
    // get place with least options, but more than one option

    let mut best_row = 0;
    let mut best_col = 0;
    let mut best_els = vec![];
    let mut shortest_len = 100;

    // check for single option in row/col/cell
    // elemnt mindestens länge 2
    'single_el_search: for row_idx in 0..9 {
        for col_idx in 0..9 {
            let current_prob_len = problem.grid[row_idx][col_idx].len();
            if current_prob_len == 1 {
                continue;
            }
            for el in &problem.grid[row_idx][col_idx] {
                // check if single possible el
                if single_element_in_col(problem, row_idx, col_idx, *el)
                    || single_element_in_row(problem, row_idx, col_idx, *el)
                    || single_element_in_cell(problem, row_idx, col_idx, *el)
                {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = vec![*el];
                    break 'single_el_search;
                }
            }
        }
    }

    if best_els.is_empty() {
        'outer: for row_idx in 0..9 {
            for col_idx in 0..9 {
                let current_prob_len = problem.grid[row_idx][col_idx].len();
                if current_prob_len == 1 {
                    continue;
                } else if current_prob_len == 2 {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = problem.grid[row_idx][col_idx].clone();
                    break 'outer;
                } else if current_prob_len > 2 && current_prob_len < shortest_len {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = problem.grid[row_idx][col_idx].clone();
                    shortest_len = current_prob_len;
                }
            }
        }
    }
    if best_els.is_empty() {
        return None;
    }

    Some(InsertionCandidate {
        row_idx: best_row,
        col_idx: best_col,
        candidates: best_els,
    })
}

fn solve_sudoku(
    problem_opt: Option<SudokuCandidates>,
    recursion_depth: usize,
) -> Option<SudokuCandidates> {
    let recursion_depth = recursion_depth + 1;

    if let Some(problem) = problem_opt {
        debug!("Depth: {}\n {}", recursion_depth, problem);

        if solution_is_correct(&problem) {
            // base case: only one possible number in each cell, solution found
            Some(problem)
        } else if solution_has_unresolvable_conflicts(&problem) {
            None
        } else {
            let insertion_cand_opt = get_best_place_and_number_to_insert(&problem);

            debug!(
                "Depth: {} Found insertion candidate {:?}",
                recursion_depth, insertion_cand_opt
            );

            if let Some(insertion_candidate) = insertion_cand_opt {
                // try all possible solutions
                let mut solution_candidates = vec![];

                for el in insertion_candidate.candidates {
                    let mut problem_bkp = problem.clone();

                    problem_bkp.grid[insertion_candidate.row_idx][insertion_candidate.col_idx] =
                        vec![el];
                    let prob_tmp = remove_from_neighbors(
                        &problem_bkp,
                        insertion_candidate.row_idx,
                        insertion_candidate.col_idx,
                        el,
                    );
                    if prob_tmp.is_some() {
                        solution_candidates.push(prob_tmp);
                    }
                }

                debug!(
                    "Found {} candidates at depth {}",
                    solution_candidates.len(),
                    recursion_depth
                );

                let solution = solution_candidates
                    .iter()
                    .cloned()
                    .find_map(|x| solve_sudoku(x, recursion_depth));

                match solution {
                    Some(x) => solve_sudoku(Some(x), recursion_depth),
                    _ => {
                        debug!("No solution found at depth {}", recursion_depth);
                        None
                    }
                }
            } else {
                debug!("No candidates found at depth {}", recursion_depth);
                None
            }
        }
    } else {
        debug!("Received None: {}", recursion_depth);
        None
    }
}

fn main() {
    let matches = App::new("Sudoku Solver")
        .version("0.1")
        .author("Stefan B. <stefan.a.baur@gmail.com>")
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

    let sudoku_problems = parse_sudoku(prob);

    for prob in sudoku_problems {
        info!("Starting with problem: {}", prob);
        let solution = solve_sudoku(Some(prob), 0);

        if let Some(solved) = solution {
            info!("Problem solved:{}", solved);
        } else {
            warn!("Problem unsolvable!");
        }
    }
}
