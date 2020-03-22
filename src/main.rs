#[derive(Clone, Copy)]
struct SubGrid {
    cells: [[Option<usize>; 3]; 3],
}
impl SubGrid {
    fn empty() -> SubGrid {
        SubGrid {
            cells: [[None; 3]; 3],
        }
    }
}

struct Sudoku {
    grid: [[SubGrid; 3]; 3],
}

impl Sudoku {
    fn empty() -> Sudoku {
        Sudoku {
            grid: [[SubGrid::empty(); 3]; 3],
        }
    }
    fn from_vec(numbers: Vec<usize>) -> Sudoku {
        assert!(numbers.len() == 81);
        let mut problem = Sudoku::empty();

        for (i, item) in numbers.iter().enumerate() {
            if *item == 0 as usize {
                continue;
            }
            let row_idx = i / 9;
            let col_idx = i % 9;

            let subgrid_row_idx = row_idx / 3;
            let subgrid_col_idx = col_idx / 3;

            let num_row_idx_within_subgrid = row_idx - subgrid_row_idx * 3;
            let num_col_idx_within_subgrid = col_idx - subgrid_col_idx * 3;

            problem.grid[subgrid_row_idx][subgrid_col_idx].cells[num_row_idx_within_subgrid]
                [num_col_idx_within_subgrid] = Some(*item);
        }

        problem
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut some_str = "".to_string();

        for row_idx in 0..9 {
            for col_idx in 0..9 {
                let subgrid_row_idx = row_idx / 3;
                let subgrid_col_idx = col_idx / 3;
                let num_row_idx_within_subgrid = row_idx - subgrid_row_idx * 3;
                let num_col_idx_within_subgrid = col_idx - subgrid_col_idx * 3;
                let sudoku_cell = self.grid[subgrid_row_idx][subgrid_col_idx].cells
                    [num_row_idx_within_subgrid][num_col_idx_within_subgrid];
                let sym = match sudoku_cell {
                    Some(nbr) => format!("{},", nbr),
                    None => "_,".to_string(),
                };
                some_str.push_str(&sym);
            }
            some_str.push_str(&"\n".to_string());
        }

        write!(f, "{}", some_str)
    }
}

fn parse_sudoku(filepath: &str) -> Sudoku {
    use std::fs;
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let problem_raw: Vec<usize> = lines
        .next()
        .unwrap()
        .split("")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    Sudoku::from_vec(problem_raw)
}

fn main() {
    let sudoku_problem = parse_sudoku("assets/problem.txt");
    println!("{}", sudoku_problem);
}
