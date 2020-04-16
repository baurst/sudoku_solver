use wasm_bindgen::prelude::*;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

#[wasm_bindgen]
pub fn wasm_solve_sudoku(input_str: &str) -> String {
    let input_str = input_str.trim();
    assert_eq!(input_str.len(), 81, "Incorrect length of input string!");
    let sudoku_problem = SudokuCandidates::from_vec(convert_problem_str(input_str));
    let solution_opt = solve_sudoku(sudoku_problem, 0);
    if let Some(solution) = solution_opt {
        solution.to_continuous_string()
    } else {
        println!("Failed to solve {}", input_str);
        input_str.to_owned()
    }
}

#[wasm_bindgen]
pub fn wasm_sudoku_contains_conflicts(input_str: &str) -> bool {
    let input_str = input_str.trim();
    assert_eq!(input_str.len(), 81, "Incorrect length of input string!");
    let sudoku_problem = SudokuCandidates::from_vec(convert_problem_str(input_str));
    if let Some(prob) = sudoku_problem {
        prob.has_unresolvable_conflicts()
    } else {
        true
    }
}

#[derive(Clone, Debug)]
pub struct SudokuCandidates {
    grid: Vec<Vec<Vec<u8>>>,
}

fn convert_problem_str(problem_raw_in: &str) -> Vec<u8> {
    problem_raw_in
        .split("")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

impl SudokuCandidates {
    fn initial() -> SudokuCandidates {
        SudokuCandidates {
            // 9 x 9 grid of u8s
            grid: vec![vec![(1..10).collect::<Vec<u8>>(); 9]; 9],
        }
    }

    fn from_vec(numbers: Vec<u8>) -> Option<SudokuCandidates> {
        assert!(numbers.len() == 81);
        let mut problem = SudokuCandidates::initial();

        for (i, item) in numbers.iter().enumerate() {
            if *item == 0 as u8 {
                continue;
            }
            let row_idx = i / 9;
            let col_idx = i % 9;

            problem.grid[row_idx][col_idx] = vec![*item];
            let prob_tmp_opt = remove_duplicates(&problem, row_idx, col_idx, *item);

            if let Some(prob) = prob_tmp_opt {
                problem = prob;
            } else {
                return None;
            }
        }
        Some(problem)
    }

    fn to_continuous_string(&self) -> String {
        let mut res_str: String = "".to_owned();
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                let el = self.grid[row_idx][col_idx][0].to_string();
                res_str += &el;
            }
        }
        res_str
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
        !self.has_unresolvable_conflicts()
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

pub fn parse_sudokus(filepath: &str) -> Vec<SudokuCandidates> {
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut candidates = vec![];
    for line in lines {
        // replace any non numeric characters in the line with 0
        let non_numeric_chars = Regex::new(r"[^0-9]").unwrap();
        let line_no_commas = line.replace(",", "");
        let problem_str_raw = non_numeric_chars
            .replace_all(&line_no_commas, "0")
            .to_owned();

        let problem_raw: Vec<u8> = convert_problem_str(&problem_str_raw);
        let cand = SudokuCandidates::from_vec(problem_raw);

        if let Some(cand) = cand {
            candidates.push(cand);
        } else {
            println!(
                "Failed to parse sudoku from {} - possibly containts conflicts.",
                &problem_str_raw
            );
        }
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

pub fn solve_sudoku(
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

#[wasm_bindgen]
pub fn wasm_get_sample_sudoku_string(random_number: f64) -> String {
    // tried using the rand method from cate rand but it panicks:
    // panicked at 'could not initialize thread_rng: getrandom: this target is not supported'
    // use rand::seq::SliceRandom;
    // use rand::thread_rng;

    let problems = vec![
        "015040002020560098300010007200000600940001000030680704458000000090872050600430900",
        "270600050000070406006059030040005600081000040029006173390000002000097800807140005",
        "020980040030047601019006080700490000800023907000605000904800006001000300350014020",
        "006030010300605000070029000020300984794000300000001005530008200069047000041200590",
        "040038500905000000000010460001650043000700901082300050830100074276000090000960002",
        "800000000003600000070090200050007000000045700000100030001000068008500010090000400",
    ];

    let random_idx = random_number * problems.len() as f64;
    problems[random_idx as usize].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_no_conflicts_js_interface() {
        assert!(!wasm_sudoku_contains_conflicts(
            "006037508700010900130050020002908000050020430600000090200005704003100060498600000"
        ));
    }

    #[test]
    fn test_has_conflicts_js_interface() {
        assert!(wasm_sudoku_contains_conflicts(
            "066037508700010900130050020002908000050020430600000090200005704003100060498600000"
        ));
    }

    #[test]
    fn test_solve_js_interface() {
        assert_eq!(
            wasm_solve_sudoku(
                "006037508700010900130050020002908000050020430600000090200005704003100060498600000"
            ),
            "926437518785216943134859627342968175859721436617543892261395784573184269498672351"
        );
    }

    #[test]
    fn test_unsolvable_solve_js_interface() {
        assert_eq!(
            wasm_solve_sudoku(
                "066037508700010900130050020002908000050020430600000090200005704003100060498600000"
            ),
            "066037508700010900130050020002908000050020430600000090200005704003100060498600000"
        );
    }

    #[test]
    fn test_trivial_problem_is_solvable() {
        let sudoku_vec = convert_problem_str(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(solve_sudoku(SudokuCandidates::from_vec(sudoku_vec), 0).is_some());
    }

    #[test]
    fn test_trivial_wrong_problem_is_unsolvable() {
        let sudoku_vec = convert_problem_str(
            "110000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        let sol_opt = solve_sudoku(SudokuCandidates::from_vec(sudoku_vec), 0);
        assert!(sol_opt.is_none());
    }
}
