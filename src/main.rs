//use rand::seq::SliceRandom;

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
            //println!("{}", *item);
            problem = remove_from_neighbors(&problem, row_idx, col_idx, *item).unwrap();
            // println!("{}", problem);
        }

        problem
    }
}

fn check_conflict_for_element(problem: &SudokuCandidates, row_idx: usize, col_idx: usize) -> bool {
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
    return true;
}

fn remove_from_neighbors(
    problem: &SudokuCandidates,
    el_row_idx: usize,
    el_col_idx: usize,
    el: u8,
) -> Option<SudokuCandidates> {
    //println!("removing {} {} {}", el_row_idx, el_col_idx, el );
    // clean row
    let mut problem = problem.clone();
    for col_idx in 0..9 {
        // if col_idx == 6 && el_row_idx == 8 && el == 2{
        //     println!("{}", problem);
        //     println!("{:?}", problem.grid[el_row_idx][col_idx]);
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
                //return check_conflict_for_element(problem, row_idx, el_col_idx);
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
    return Some(problem);
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

        write!(f, "{}", some_str)
    }
}

fn parse_sudoku(filepath: &str) -> SudokuCandidates {
    use std::fs;
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let problem_raw: Vec<u8> = lines
        .next()
        .unwrap()
        .split("")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    SudokuCandidates::from_vec(problem_raw)
}

use std::collections::HashSet;

use std::hash::Hash;

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
            println!("Unresolvable conflict at row: {}", row_idx);
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
            println!("Unresolvable conflict at col: {}", col_idx);
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
                println!(
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
    return true;
}

#[derive(Clone, Debug)]
struct InsertionCandidate {
    row_idx: usize,
    col_idx: usize,
    candidates: Vec<u8>,
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
                // check col
                let mut single_el: bool = true;
                for row_inner_idx in 0..9 {
                    if row_idx == row_inner_idx {
                        continue;
                    }
                    if problem.grid[row_inner_idx][col_idx].iter().any(|x| x == el) {
                        single_el = false;
                        break;
                    }
                }
                if single_el {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = vec![*el];
                    println!("single element in col: {} {}: {}", row_idx, col_idx, el);
                    break 'single_el_search;
                }

                single_el = true;
                // check row
                for col_inner_idx in 0..9 {
                    if col_idx == col_inner_idx {
                        continue;
                    }
                    if problem.grid[row_idx][col_inner_idx].iter().any(|x| x == el) {
                        single_el = false;
                        break;
                    }
                }
                if single_el {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = vec![*el];
                    println!("single element in row: {} {}: {}", row_idx, col_idx, el);
                    break 'single_el_search;
                }

                single_el = true;
                // check cell
                let cell_row_idx = row_idx / 3;
                let cell_col_idx = col_idx / 3;

                'outer_cell: for row_idx_in_cell in (cell_row_idx * 3)..((cell_row_idx + 1) * 3) {
                    for col_idx_in_cell in (cell_col_idx * 3)..((cell_col_idx + 1) * 3) {
                        if row_idx == row_idx_in_cell && col_idx == col_idx_in_cell {
                            continue;
                        }
                        if problem.grid[row_idx_in_cell][col_idx_in_cell]
                            .iter()
                            .any(|x| x == el)
                        {
                            single_el = false;
                            break 'outer_cell;
                        }
                    }
                }
                if single_el {
                    best_row = row_idx;
                    best_col = col_idx;
                    best_els = vec![*el];
                    println!(
                        "single element in cell {} {}: {} {}: {}",
                        cell_row_idx, cell_col_idx, row_idx, col_idx, el
                    );
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

    if problem_opt.is_none() {
        println!("Received None: {}", recursion_depth);
        return None;
    } else {
        let problem = problem_opt.unwrap();
        println!("Depth: {}\n {}", recursion_depth, problem);

        if solution_is_correct(&problem) {
            // base case: only one possible number in each cell, solution found
            return Some(problem);
        } else if solution_has_unresolvable_conflicts(&problem) {
            return None;
        } else {
            let insertion_cand_opt = get_best_place_and_number_to_insert(&problem);
            println!(
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
                // println!(
                //     "Found {} candidates at depth {}",
                //     solution_candidates.len(),
                //     recursion_depth
                // );

                let solution = solution_candidates
                    .iter()
                    .cloned()
                    .find_map(|x| solve_sudoku(x, recursion_depth));

                match solution {
                    Some(x) => return solve_sudoku(Some(x), recursion_depth),
                    _ => {
                        println!("No solution found at depth {}", recursion_depth);
                        return None;
                    }
                }
            } else {
                println!("No candidates found at depth {}", recursion_depth);
                return None;
            }

            // if prob_tmp.is_some() {
            //     problem = prob_tmp.unwrap();
            //     return solve_sudoku(problem, recursion_depth);
            // } else {
            //     return None;
            // }

            // // if update_worked {
            // //     println!(
            // //         "Update worked: row_idx {}, col_idx {}, el {}\n{}",
            // //         row_idx, col_idx, el, problem
            // //     );
            // return solve_sudoku(Some(problem), recursion_depth);
            // } else {
            //     let dupl_idx = problem_bkp.grid[row_idx][col_idx]
            //         .iter()
            //         .position(|x| *x == el);
            //     if let Some(x) = dupl_idx {
            //         problem_bkp.grid[row_idx][col_idx].remove(x);
            //         assert!(!problem_bkp.grid[row_idx][col_idx].is_empty());
            //         println!(
            //             "Recursion depth {}: Num options left: {}",
            //             problem_bkp.grid[row_idx][col_idx].len(),
            //             recursion_depth
            //         );
            //     }
        }
    }
    return None;
}

fn main() {
    let probs = vec!["assets/problem_easy.txt", "assets/problem_hard.txt"];

    for prob in probs {
        let sudoku_problem = parse_sudoku(prob);

        println!("Starting!");
        println!("{}", sudoku_problem);
        let solution = solve_sudoku(Some(sudoku_problem), 0);

        if let Some(solved) = solution {
            println!("Problem solved:");
            println!("{}", solved);
        } else {
            println!("Problem unsolvable!");
        }
    }
}
