#[derive(Clone, Copy)]
struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn new(grid: [[u8; 9]; 9]) -> Self {
        Sudoku { grid }
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        // check row and column
        for i in 0..9 {
            if self.grid[row][i] == num || self.grid[i][col] == num {
                return false;
            }
        }

        // check 3x3 box
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[start_row + i][start_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    for num in 1..=9 {
                        if self.is_valid(row, col, num) {
                            self.grid[row][col] = num;
                            if self.solve() {
                                return true;
                            }
                            self.grid[row][col] = 0; // backtrack
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn display(&self) {
        for row in &self.grid {
            for &num in row {
                print!("{} ", num);
            }
            println!();
        }
    }
}

fn main() {
    // TODO: allow user to input grid
    let grid = [
        [0, 0, 3, 0, 2, 0, 6, 0, 0],
        [9, 0, 0, 3, 0, 5, 0, 0, 1],
        [0, 0, 1, 8, 0, 6, 4, 0, 0],
        [0, 0, 8, 1, 0, 2, 9, 0, 0],
        [7, 0, 0, 8, 0, 0, 0, 0, 8],
        [0, 0, 6, 7, 0, 8, 2, 0, 0],
        [0, 0, 2, 6, 0, 9, 5, 0, 0],
        [8, 0, 0, 2, 0, 3, 0, 0, 9],
        [0, 0, 5, 0, 1, 0, 3, 0, 0],
    ];

    let mut sudoku = Sudoku::new(grid);
    if sudoku.solve() {
        println!("Solved Sudoku:");
        sudoku.display();
    } else {
        println!("No solution exists.");
    }
}
