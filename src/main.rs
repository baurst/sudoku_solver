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

            problem.eliminate_conflicting_occurences(row_idx, col_idx, *item);
        }
        problem
    }
    fn eliminate_conflicting_occurences(&mut self, el_row_idx: usize, el_col_idx: usize, el: u8) {
        // clean row
        for col_idx in 0..9 {
            if col_idx == el_col_idx {
                continue;
            }
            let dupl_idx = self.grid[el_row_idx][col_idx].iter().position(|x| *x == el);
            if let Some(x) = dupl_idx {
                self.grid[el_row_idx][col_idx].remove(x);
            }
        }

        // clean row
        for row_idx in 0..9 {
            if row_idx == el_row_idx {
                continue;
            }
            let dupl_idx = self.grid[row_idx][el_col_idx].iter().position(|x| *x == el);
            if let Some(x) = dupl_idx {
                self.grid[row_idx][el_col_idx].remove(x);
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
                let dupl_idx = self.grid[row_idx][el_col_idx].iter().position(|x| *x == el);
                if let Some(x) = dupl_idx {
                    self.grid[row_idx][col_idx].remove(x);
                }
            }
        }
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

fn main() {
    let sudoku_problem = parse_sudoku("assets/problem.txt");
    println!("{}", sudoku_problem);
}
