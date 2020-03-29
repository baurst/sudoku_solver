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
use std::time::Instant;

#[derive(Clone, Debug)]
struct SudokuCandidates {
    grid: Vec<Vec<Vec<u8>>>,
}

impl SudokuCandidates {
    fn initial() -> SudokuCandidates {
        SudokuCandidates {
            // 9 x 9 grid of u8s
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
            problem = remove_duplicates(&problem, row_idx, col_idx, *item).unwrap();
        }

        problem
    }

    fn get_duplicates_in_column(
        &self,
        element: u8,
        el_row_idx: usize,
        el_col_idx: usize,
    ) -> HashSet<(usize, usize)> {
        let mut duplicates: HashSet<(usize, usize)> = HashSet::new();
        // find duplicates in column
        for row_idx in 0..9 {
            if row_idx == el_row_idx {
                continue;
            }
            if self.grid[row_idx][el_col_idx].iter().any(|x| *x == element) {
                duplicates.insert((row_idx, el_col_idx));
            }
        }
        duplicates
    }

    fn get_duplicates_in_row(
        &self,
        element: u8,
        el_row_idx: usize,
        el_col_idx: usize,
    ) -> HashSet<(usize, usize)> {
        let mut duplicates: HashSet<(usize, usize)> = HashSet::new();
        // find duplicates in row
        for col_idx in 0..9 {
            if col_idx == el_col_idx {
                continue;
            }
            if self.grid[el_row_idx][col_idx].iter().any(|x| *x == element) {
                duplicates.insert((el_row_idx, col_idx));
            }
        }
        duplicates
    }

    fn get_duplicates_in_cell(
        &self,
        element: u8,
        el_row_idx: usize,
        el_col_idx: usize,
    ) -> HashSet<(usize, usize)> {
        let mut duplicates: HashSet<(usize, usize)> = HashSet::new();
        // find duplicates in cell
        let cell_row_idx = el_row_idx / 3;
        let cell_col_idx = el_col_idx / 3;

        for row_idx_in_cell in 0..3 {
            for col_idx_in_cell in 0..3 {
                let row_idx = cell_row_idx * 3 + row_idx_in_cell;
                let col_idx = cell_col_idx * 3 + col_idx_in_cell;
                if row_idx == el_row_idx && col_idx == el_col_idx {
                    continue;
                }
                if self.grid[row_idx][col_idx].iter().any(|x| *x == element) {
                    duplicates.insert((row_idx, col_idx));
                }
            }
        }
        duplicates
    }

    fn get_duplicates(
        &self,
        element: u8,
        el_row_idx: usize,
        el_col_idx: usize,
    ) -> HashSet<(usize, usize)> {
        let mut duplicates: HashSet<(usize, usize)> = HashSet::new();

        let row_duplicates = self.get_duplicates_in_row(element, el_row_idx, el_col_idx);
        duplicates.extend(row_duplicates);

        let col_duplicates = self.get_duplicates_in_column(element, el_row_idx, el_col_idx);
        duplicates.extend(&col_duplicates);

        let cell_duplicates = self.get_duplicates_in_cell(element, el_row_idx, el_col_idx);
        duplicates.extend(&cell_duplicates);

        duplicates
    }

    fn is_correct(&self) -> bool {
        for row in &self.grid {
            for col in row {
                if col.len() != 1 {
                    return false;
                }
            }
        }
        for col_idx in 0..9 {
            let mut sum_col_elems = 0;
            for row_idx in 0..9 {
                sum_col_elems += self.grid[row_idx][col_idx][0];
            }
            if sum_col_elems != 9 * 10 / 2 {
                return false;
            }
        }
        for row_idx in 0..9 {
            let mut sum_row_elems = 0;
            for col_idx in 0..9 {
                sum_row_elems += self.grid[row_idx][col_idx][0];
            }
            if sum_row_elems != 9 * 10 / 2 {
                return false;
            }
        }
        return !self.has_unresolvable_conflicts();
    }

    fn has_unresolvable_conflicts(&self) -> bool {
        // rows okay
        for row_idx in 0..9 {
            let mut elems = vec![];
            for col_idx in 0..9 {
                if self.grid[row_idx][col_idx].len() == 1 {
                    elems.push(self.grid[row_idx][col_idx][0]);
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
                if self.grid[row_idx][col_idx].len() == 1 {
                    elems.push(self.grid[row_idx][col_idx][0]);
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
                        if self.grid[row_idx][col_idx].len() == 1 {
                            cell_elems.push(self.grid[row_idx][col_idx][0]);
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

    fn get_best_place_and_number_to_insert(&self) -> Option<InsertionCandidate> {
        // get place with least options, but more than one option

        let mut best_row = 0;
        let mut best_col = 0;
        let mut best_els = vec![];
        let mut shortest_len = 100;

        // check for single option in row/col/cell
        // elemnt mindestens länge 2
        'single_el_search: for row_idx in 0..9 {
            for col_idx in 0..9 {
                let current_prob_len = self.grid[row_idx][col_idx].len();
                if current_prob_len == 1 {
                    continue;
                }
                for el in &self.grid[row_idx][col_idx] {
                    // check if single possible el
                    if is_single_element_in_col(self, row_idx, col_idx, *el)
                        || is_single_element_in_row(self, row_idx, col_idx, *el)
                        || is_single_element_in_cell(self, row_idx, col_idx, *el)
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
                    let current_prob_len = self.grid[row_idx][col_idx].len();
                    if current_prob_len == 1 {
                        continue;
                    } else if current_prob_len == 2 {
                        best_row = row_idx;
                        best_col = col_idx;
                        best_els = self.grid[row_idx][col_idx].clone();
                        break 'outer;
                    } else if current_prob_len > 2 && current_prob_len < shortest_len {
                        best_row = row_idx;
                        best_col = col_idx;
                        best_els = self.grid[row_idx][col_idx].clone();
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
            some_str.push_str("\n");
        }

        write!(f, "\n{}", some_str)
    }
}

fn parse_sudokus(filepath: &str) -> Vec<SudokuCandidates> {
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut candidates = vec![];
    for line in lines {
        // replace any non numeric characters in the line with 0
        let non_numeric_chars = Regex::new(r"[^0-9]").unwrap();
        let line_no_commas = line.replace(",", "");
        let result = non_numeric_chars.replace_all(&line_no_commas, "0");
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

fn remove_duplicates(
    problem: &SudokuCandidates,
    el_row_idx: usize,
    el_col_idx: usize,
    el: u8,
) -> Option<SudokuCandidates> {
    debug!("removing {} {} {}", el_row_idx, el_col_idx, el);
    let mut problem = problem.clone();
    let duplicates = problem.get_duplicates(el, el_row_idx, el_col_idx);
    for (dup_row_idx, dup_col_idx) in duplicates {
        let dupl_idx = problem.grid[dup_row_idx][dup_col_idx]
            .iter()
            .position(|x| *x == el);
        if let Some(x) = dupl_idx {
            problem.grid[dup_row_idx][dup_col_idx].remove(x);
            if problem.grid[dup_row_idx][dup_col_idx].is_empty() {
                // conflict detected
                return None;
            } else if problem.grid[dup_row_idx][dup_col_idx].len() == 1 {
                let new_prob_opt = remove_duplicates(
                    &problem,
                    dup_row_idx,
                    dup_col_idx,
                    problem.grid[dup_row_idx][dup_col_idx][0],
                );
                if let Some(new_prob) = new_prob_opt {
                    problem = new_prob;
                } else {
                    return None;
                }
            }
        }
    }
    Some(problem)
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[derive(Clone, Debug)]
struct InsertionCandidate {
    row_idx: usize,
    col_idx: usize,
    candidates: Vec<u8>,
}

fn is_single_element_in_col(
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

fn is_single_element_in_row(
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

fn is_single_element_in_cell(
    problem: &SudokuCandidates,
    row_idx: usize,
    col_idx: usize,
    el: u8,
) -> bool {
    let cell_row_idx = row_idx / 3;
    let cell_col_idx = col_idx / 3;

    for row_idx_within_cell in 0..3 {
        for col_idx_within_cell in 0..3 {
            let row_idx_abs = 3 * cell_row_idx + row_idx_within_cell;
            let col_idx_abs = 3 * cell_col_idx + col_idx_within_cell;
            if row_idx_abs == row_idx && col_idx_abs == col_idx {
                continue;
            }
            if problem.grid[row_idx_abs][col_idx_abs]
                .iter()
                .any(|x| x == &el)
            {
                return false;
            }
        }
    }
    true
}

fn solve_sudoku(
    problem_opt: Option<SudokuCandidates>,
    recursion_depth: usize,
) -> Option<SudokuCandidates> {
    let recursion_depth = recursion_depth + 1;

    if let Some(problem) = problem_opt {
        debug!("Depth: {}\n {}", recursion_depth, problem);

        if problem.is_correct() {
            // base case: only one possible number in each cell, solution found
            Some(problem)
        } else if problem.has_unresolvable_conflicts() {
            None
        } else {
            let insertion_cand_opt = problem.get_best_place_and_number_to_insert();

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
                    let prob_tmp = remove_duplicates(
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
