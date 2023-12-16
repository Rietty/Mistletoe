// This module contains a custom written grid container.
// The container is a single-dimensional character vector, alongside a width of the size of each row.
use transpose::transpose as external_transpose;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[allow(unused)]
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[allow(unused)]
impl Grid {
    // The base new function, it takes a vector of characters and a width.
    pub fn new(grid: Vec<char>, width: usize) -> Grid {
        Grid { grid, width }
    }

    // Create a new grid from a vector of strings, where each string is a row.
    pub fn from_rows(grid: Vec<String>, width: usize) -> Grid {
        Grid::new(grid.iter().flat_map(|s| s.chars()).collect(), width)
    }

    // Create a new grid from a vector of strings, where each string is a column.
    pub fn from_columns(grid: Vec<String>, width: usize) -> Grid {
        let mut chars = Vec::with_capacity(grid.len() * width);
        for i in 0..width {
            for j in 0..grid.len() {
                chars.push(grid[j].chars().nth(i).unwrap());
            }
        }
        Grid::new(chars, width)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.height() || col >= self.width() {
            None
        } else {
            Some(self.grid[row * self.width + col])
        }
    }

    pub fn get_pos(&self, pos: Position) -> Option<char> {
        self.get(pos.y, pos.x)
    }

    pub fn is_in_bounds(&self, pos: Position) -> bool {
        pos.x < self.width() && pos.y < self.height()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    // Get the rows of the grid, as a series of slices.
    pub fn rows(&self) -> Vec<Vec<char>> {
        self.grid
            .chunks(self.width)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    // Get the columns of the grid, as a series of slices.
    pub fn columns(&self) -> Vec<Vec<char>> {
        (0..self.width)
            .map(|i| {
                self.grid
                    .iter()
                    .skip(i)
                    .step_by(self.width)
                    .cloned()
                    .collect::<Vec<char>>()
            })
            .collect()
    }

    // Function that will output the grid in a readable format as a giant string.
    pub fn to_string(&self, newlines: bool) -> String {
        if newlines {
            self.grid
                .chunks(self.width)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            self.grid.iter().collect::<String>()
        }
    }

    // Transpose the 1d grid using the transpose crate.
    pub fn transpose(&mut self) {
        // Get the width and height of the grid.
        let width = self.width;
        let height = self.grid.len() / width;

        // Create a new vector of characters of same length as the grid.
        let mut new_grid: Vec<char> = vec![' '; self.grid.len()];

        // Perform the transpose.
        external_transpose(&self.grid, &mut new_grid, width, height);

        // Set the grid to the new grid.
        self.grid = new_grid;

        // Set the width to the new height.
        self.width = height;
    }

    // Flip the grid horizontally, (on the x-axis).
    pub fn flip_x(&mut self) {
        let height = self.grid.len() / self.width;
        for row in 0..height / 2 {
            for col in 0..self.width {
                let top_index = row * self.width + col;
                let bottom_index = (height - 1 - row) * self.width + col;
                self.grid.swap(top_index, bottom_index);
            }
        }
    }

    // Flip the grid vertically, (on the y-axis).
    pub fn flip_y(&mut self) {
        let height = self.grid.len() / self.width;
        for row in 0..height {
            let start = row * self.width;
            let end = start + self.width;
            self.grid[start..end].reverse();
        }
    }
}
