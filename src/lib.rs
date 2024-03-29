use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub data: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
pub struct GameOfLife {
    pub steps: Vec<Matrix>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data: Vec<Vec<bool>> = vec![];
        for _i in 0..rows {
            let mut row: Vec<bool> = vec![];
            for _j in 0..cols {
                // Set random bool value for each cell
                let val = matches!(rand::thread_rng().gen_range(0..2), 0);
                row.push(val);
            }
            data.push(row);
        }

        Self { data, rows, cols }
    }

    fn neighbors_number(&self, _row: usize, _col: usize) -> u32 {
        // Get boundaries for potential neighbors
        let start_row = if _row == 0 { 0 } else { _row - 1 };
        let end_row = if _row == &self.data.len() - 1 {
            self.data.len() - 1
        } else {
            _row + 1
        };

        let start_col = if _col == 0 { 0 } else { _col - 1 };
        let end_col = if _col == self.cols - 1 {
            self.cols - 1
        } else {
            _col + 1
        };

        // Get neighbors number for the given cell
        let mut neighbors = 0;
        for i in start_row..=end_row {
            for j in start_col..=end_col {
                if !(i == _row && j == _col)
                    && *self
                        .data
                        .get(i)
                        .expect("Reason #1")
                        .get(j)
                        .expect("Reason #2")
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }
}

impl Iterator for Matrix {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Vec<Vec<bool>> = vec![];
        for (i, row) in self.data.iter().enumerate() {
            let mut next_row: Vec<bool> = vec![];
            for (j, cell) in row.iter().enumerate() {
                let next_cell: bool = match self.neighbors_number(i, j) {
                    3 => true,
                    2 => *cell,
                    _ => false,
                };
                next_row.push(next_cell);
            }
            next.push(next_row);
        }

        Some(Self {
            data: next,
            rows: self.rows,
            cols: self.cols,
        })
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let row_join = format!(
            "\r\n{:-^1$}\r\n",
            "-",
            (self.data.get(0).unwrap().len() * 4 + 1)
        );

        let content: String = self
            .data
            .iter()
            .map(|row| {
                let row_display: String = row
                    .iter()
                    .map(|&value| {
                        if value {
                            "o".to_string()
                        } else {
                            " ".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" | ");

                format!("| {} |", row_display)
            })
            .collect::<Vec<String>>()
            .join(&row_join);

        write!(f, "{}", &content)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for (i, r) in self.data.iter().enumerate() {
            if let Some(other_row) = other.data.get(i) {
                if other_row != r {
                    return false;
                }
            }
        }

        true
    }
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Self {
        let matrix = Matrix::new(rows, cols);
        let steps: Vec<Matrix> = vec![matrix; 1];

        Self { steps }
    }
}

impl Display for GameOfLife {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(matrix) = self.steps.last() {
            write!(f, "\n{}\n\nstep: {}\n", matrix, self.steps.len())
        } else {
            write!(f, "NA")
        }
    }
}

impl Iterator for GameOfLife {
    type Item = Matrix;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last) = self.steps.last() {
            let mut last = last.clone();
            if let Some(next) = last.next() {
                // TODO Check if next is equal to n-1 Matrix (infinite loop)
                if next == last {
                    None
                } else {
                    self.steps.push(next.clone());
                    Some(next)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
