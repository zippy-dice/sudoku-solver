use clap::Parser;
use colored::*;
use std::{collections::HashSet, fs};

#[derive(Parser)]
struct Cli {
    input: String,
}

struct Sudoku {
    is_fixed_cell: Vec<Vec<bool>>,
    field: Vec<Vec<Option<u32>>>,
    used_nums_in_row: Vec<HashSet<u32>>,
    used_nums_in_col: Vec<HashSet<u32>>,
    used_nums_in_block: Vec<Vec<HashSet<u32>>>,
}

impl Sudoku {
    fn new() -> Sudoku {
        let is_fixed_cell = vec![vec![false; 9]; 9];
        let field = vec![vec![None; 9]; 9];
        let used_nums_in_row = vec![HashSet::new(); 9];
        let used_nums_in_col = vec![HashSet::new(); 9];
        let used_nums_in_block = vec![vec![HashSet::new(); 3]; 3];
        Sudoku {
            is_fixed_cell,
            field,
            used_nums_in_row,
            used_nums_in_col,
            used_nums_in_block,
        }
    }

    fn from_file(input_file: String) -> Sudoku {
        let mut sudoku = Sudoku::new();
        let lines = fs::read_to_string(input_file).expect("Failed reading file.");

        for (i_idx, line) in lines.split('\n').enumerate() {
            for (j_idx, ch) in line.chars().enumerate() {
                let val = match ch {
                    '1'..='9' => Some(ch.to_digit(10).unwrap()),
                    '*' => None,
                    _ => panic!("Invalid value. {}", ch),
                };

                if val.is_some() {
                    sudoku.put(i_idx, j_idx, val);
                    sudoku.is_fixed_cell[i_idx][j_idx] = true;
                }
            }
        }

        sudoku
    }

    fn put(&mut self, r: usize, c: usize, x: Option<u32>) {
        if let Some(x) = x {
            assert!(self.field[r][c].is_none());
            self.field[r][c] = Some(x);
            self.used_nums_in_row[r].insert(x);
            self.used_nums_in_col[c].insert(x);
            self.used_nums_in_block[r / 3][c / 3].insert(x);
        } else {
            assert!(self.field[r][c].is_some());
            let x = self.field[r][c].unwrap();
            self.field[r][c] = None;
            self.used_nums_in_row[r].remove(&x);
            self.used_nums_in_col[c].remove(&x);
            self.used_nums_in_block[r / 3][c / 3].remove(&x);
        }
    }

    fn can_put(&self, r: usize, c: usize, x: Option<u32>) -> bool {
        if let Some(x) = x {
            if self.field[r][c].is_some() {
                return false;
            }

            if self.used_nums_in_row[r].contains(&x) {
                return false;
            }
            if self.used_nums_in_col[c].contains(&x) {
                return false;
            }
            if self.used_nums_in_block[r / 3][c / 3].contains(&x) {
                return false;
            }

            true
        } else {
            assert!(self.field[r][c].is_some());
            self.field[r][c].is_some()
        }
    }

    fn dfs(&mut self, r: usize, c: usize) {
        if r == 9 {
            println!("--------------------");
            println!("answer");
            self.print();
            return;
        }

        let (nr, nc);
        if c < 8 {
            nr = r;
            nc = c + 1;
        } else {
            nr = r + 1;
            nc = 0;
        }
        if self.field[r][c].is_none() {
            for x in 1..=9 {
                if self.can_put(r, c, Some(x)) {
                    self.put(r, c, Some(x));
                    self.dfs(nr, nc);
                    self.put(r, c, None);
                }
            }
        } else {
            self.dfs(nr, nc);
        }
    }

    fn print(&self) {
        for r_idx in 0..9 {
            for c_idx in 0..9 {
                let ch = self.field[r_idx][c_idx]
                    .map_or('*', |x| std::char::from_digit(x, 10).expect("num is 1 ~ 9"));
                let mut ch: ColoredString = ch.to_string().into();
                if self.is_fixed_cell[r_idx][c_idx] {
                    ch = ch.black();
                } else {
                    ch = ch.red();
                    ch = ch.bold();
                    ch = ch.underline();
                }
                if ( (r_idx / 3) % 2 ) ^ ( (c_idx / 3) % 2 ) == 1 {
                    ch = ch.on_green();
                } else {
                    ch = ch.on_cyan();
                }
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let input_file = cli.input;
    println!("input file: {}", input_file);

    let mut sdk = Sudoku::from_file(input_file);
    sdk.print();

    sdk.dfs(0, 0);
}
