use rand::seq::SliceRandom;

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

            try_insert_and_eliminate_conflicting_occurences(&mut problem, row_idx, col_idx, *item);
        }
        problem
    }
}
fn try_insert_and_eliminate_conflicting_occurences(
    problem: &mut SudokuCandidates,
    el_row_idx: usize,
    el_col_idx: usize,
    el: u8,
) -> (bool, SudokuCandidates) {
    let problem_tmp = problem.clone();

    // clean row
    for col_idx in 0..9 {
        if col_idx == el_col_idx {
            continue;
        }
        let dupl_idx = problem.grid[el_row_idx][col_idx]
            .iter()
            .position(|x| *x == el);
        if let Some(x) = dupl_idx {
            problem.grid[el_row_idx][col_idx].remove(x);
            if problem.grid[el_row_idx][col_idx].is_empty() {
                return (false, problem_tmp); // conflict detected
            }
        }
    }

    // clean row
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
                return (false, problem_tmp); // conflict detected
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
            let dupl_idx = problem.grid[row_idx][el_col_idx]
                .iter()
                .position(|x| *x == el);
            if let Some(x) = dupl_idx {
                problem.grid[row_idx][col_idx].remove(x);
                if problem.grid[row_idx][col_idx].is_empty() {
                    return (false, problem_tmp); // conflict detected
                }
            }
        }
    }
    (true, problem.clone())
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

fn _solution_has_conflicts(solution: SudokuCandidates) -> bool {
    for col_idx in 0..9 {
        let mut all_col_elems = vec![];
        for row_idx in 0..9 {
            all_col_elems.push(solution.grid[row_idx][col_idx].iter());
        }

        /*
        if let Some(x) = dupl_idx {
            self.grid[el_row_idx][col_idx].remove(x);
        }
        */
    }
    true
}

fn check_solution(solution: &SudokuCandidates) -> bool {
    for row in &solution.grid {
        for col in row {
            if col.len() != 1 {
                return false;
            }
        }
    }
    true
}

fn get_best_place_and_number_to_insert(problem: &SudokuCandidates) -> (usize, usize, u8) {
    // get place with least options, but more than one option

    let mut best_row = 0;
    let mut best_col = 0;
    let mut best_el = 10;
    let mut shortest_len = 10;

    'outer: for row_idx in 0..9 {
        for col_idx in 0..9 {
            let current_prob_len = problem.grid[row_idx][col_idx].len();
            if current_prob_len == 1 {
                continue;
            } else if current_prob_len == 2 {
                best_row = row_idx;
                best_col = col_idx;
                best_el = *problem.grid[row_idx][col_idx]
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                break 'outer;
            } else if current_prob_len > 2 && current_prob_len < shortest_len {
                best_row = row_idx;
                best_col = col_idx;
                best_el = *problem.grid[row_idx][col_idx]
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                shortest_len = current_prob_len;
            }
        }
    }
    (best_row, best_col, best_el)
}

fn solve_sudoku(problem: &mut SudokuCandidates, recursion_depth: usize) -> SudokuCandidates {
    println!("Recursion depth {}", recursion_depth);
    if check_solution(problem) {
        // base case: only one possible number in each cell, solution found
        problem.clone()
    } else {
        let (row_idx, col_idx, el) = get_best_place_and_number_to_insert(problem);

        let mut updated_problem =
            try_insert_and_eliminate_conflicting_occurences(problem, row_idx, col_idx, el);
        if updated_problem.0 {
            updated_problem.1.grid[row_idx][col_idx] = vec![el];
            let recursion_depth = recursion_depth + 1;
            solve_sudoku(&mut updated_problem.1, recursion_depth)
        } else {
            let dupl_idx = problem.grid[row_idx][col_idx].iter().position(|x| *x == el);
            if let Some(x) = dupl_idx {
                problem.grid[row_idx][col_idx].remove(x);
                println!("Num options left: {}", problem.grid[row_idx][col_idx].len());
            }
            let recursion_depth = recursion_depth + 1;
            solve_sudoku(problem, recursion_depth)
        }
    }
}

fn main() {
    let mut sudoku_problem = parse_sudoku("assets/problem.txt");

    println!("{}", sudoku_problem);
    let solution = solve_sudoku(&mut sudoku_problem, 0);
    println!("{}", solution);
}
