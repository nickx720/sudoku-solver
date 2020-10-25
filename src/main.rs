use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Sudoku {
    matrix: Vec<Vec<u32>>,
}
impl Sudoku {
    pub fn new(matrix: Vec<Vec<u32>>) -> Self {
        Sudoku { matrix }
    }
    fn number_unassigned(&self, mut row: usize, mut col: usize) -> Vec<u32> {
        let mut numunassign = 0;
        for i in 0..9 {
            for j in 0..9 {
                if self.matrix[i][j] == 0 {
                    row = i;
                    col = j;
                    numunassign = 1;
                    let a: Vec<u32> = vec![numunassign, row as u32, col as u32];
                    return a;
                }
            }
        }
        let a: Vec<u32> = vec![numunassign, 0, 0];
        a
    }
    fn is_safe(&self, n: u32, r: u32, c: u32) -> bool {
        for i in 0..9 {
            if self.matrix[r as usize][i] == n {
                return false;
            }
        }
        for i in 0..9 {
            if self.matrix[i][c as usize] == n {
                return false;
            }
        }
        let row_start = ((r / 3) * 3) as usize;
        let col_start = ((c / 3) * 3) as usize;
        for i in row_start..row_start + 3 {
            for j in col_start..col_start + 3 {
                if self.matrix[i][j] == n {
                    return false;
                }
            }
        }
        true
    }
    pub fn solve_sudoku(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let a = self.number_unassigned(row as usize, col as usize);
        if a[0] == 0 {
            return true;
        }
        row = a[1];
        col = a[2];
        for i in 1..=9 {
            if self.is_safe(i, row, col) {
                self.matrix[row as usize][col as usize] = i;
                if self.solve_sudoku() {
                    return true;
                }
                self.matrix[row as usize][col as usize] = 0;
            }
        }
        false
    }
}
fn main() {
    let filename = "data/sudoku1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sudoku_input: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        let mut new_row: Vec<u32> = Vec::new();
        for &i in line.iter() {
            let b = i.to_digit(10);
            match b {
                Some(b) => {
                    new_row.push(b);
                    if new_row.len() == 9 {
                        sudoku_input.push(new_row.clone());
                        new_row.clear();
                    }
                }
                None => {
                    new_row.push(0);
                    if new_row.len() == 9 {
                        sudoku_input.push(new_row.clone());
                        new_row.clear();
                    }
                    continue;
                }
            }
        }
    }

    let mut sudoku = Sudoku::new(sudoku_input);
    if sudoku.solve_sudoku() {
        println!("{:?}", sudoku);
    } else {
        println!("No solution");
    }
}
