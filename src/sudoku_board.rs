const SIZE: usize = 9;

/// Make unfilled cells >= 512?

#[derive(Clone, Copy)]
pub struct SudokuBoard {
    /// Init as a bunch of 16 bit ints.  Use bitmasking to track values
    board: [u16; SIZE * SIZE],
}

impl SudokuBoard {
    #[allow(dead_code)]
    pub fn blank() -> SudokuBoard {
        SudokuBoard {
            board: [0; SIZE * SIZE],
        }
    }

    /// Uses test data format. Blank cells are represented by '.'
    pub fn from_str(puzzle: &str) -> SudokuBoard {
        let mut board: [u16; SIZE * SIZE] = [0u16; SIZE * SIZE];
        puzzle
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(i, cell)| match cell.to_digit(10) {
                Some(n) => board[i] = u16::pow(2, n - 1),
                None => board[i] = 0b111111111 as u16,
            });

        //SudokuBoard::analyze(board);
        SudokuBoard { board }
    }

    /// Prints board in a readable format. Converts internal bit values to 0..9. Ambiguous values are represented as 0
    #[allow(dead_code)]
    pub fn print(&self) {
        println!();
        self.board.into_iter().enumerate().for_each(|(i, cell)| {
            if i % 9 == 0 {
                print!("\n");
            }
            print!("{:?}, ", SudokuBoard::get_sudoku_value(&cell));
        });
        print!("\n");
    }

    #[allow(dead_code)]
    pub fn print_raw(&self) {
        println!();

        self.board.into_iter().enumerate().for_each(|(i, cell)| {
            if i % 9 == 0 {
                print!("\n");
            }
            print!("{:?}, ", cell);
        });
        print!("\n")
    }

    /// Returns a solved copy of the origional board
    pub fn solve(&mut self) {
        let mut counter = 0;
        println!("solving...{:?}", counter);
        while self.update_cells() {
            counter += 1;
            println!("solving...{:?}", counter);
        }
    }

    fn update_cells(&mut self) -> bool {
        let mut board = self.board;

        for (i, _) in self
            .board
            .into_iter()
            .enumerate()
            .filter(|(_, c)| !c.is_power_of_two())
        {
            // Find naked singles - check against filled cells in row, column and box together
            // cell ^ (filled_row | filled_col | filled_box)
            let row = self.get_cell_row_filled_mask(i);
            let col = self.get_cell_col_filled_mask(i);
            let bxx = self.get_cell_box_filled_mask(i);

            board[i] = 0b111_111_111 & !(row | col | bxx);
            board[i] &= !self.get_cell_row_neighbor_mask(i);
            board[i] &= !self.get_cell_col_neighbor_mask(i);
            board[i] &= !self.get_cell_box_neighbor_mask(i);

            // Find hidden singles - check against all cells in row, column and box individually
            // maybe this doesn't work? Think about this.
            // (cell ^ neighbor_row) & (cell ^ neighbor_col) & (cell ^ neighbor_box)
        }

        let change = board != self.board;

        self.board = board;

        change
    }

    fn get_cell_row_neighbor_mask(&self, index: usize) -> u16 {
        let mut row: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (i / SIZE == index / SIZE) && i != &index)
            .for_each(|(_, c)| row |= c);

        row
    }
    fn get_cell_col_neighbor_mask(&self, index: usize) -> u16 {
        let mut col: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (i % SIZE == index % SIZE) && i != &index)
            .for_each(|(_, c)| col |= c);

        col
    }
    fn get_cell_box_neighbor_mask(&self, index: usize) -> u16 {
        let mut bxx: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                3 * ((i / SIZE) / 3) + (i % SIZE) / 3
                    == 3 * ((index / SIZE) / 3) + (index % SIZE) / 3
                    && i != &index
            })
            .for_each(|(_, c)| bxx |= c);

        bxx
    }

    fn get_cell_row_filled_mask(&self, index: usize) -> u16 {
        let mut row: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (i / SIZE == index / SIZE) && *i != index)
            .for_each(|(_, c)| row |= SudokuBoard::get_sudoku_value(&c));

        row
    }
    fn get_cell_col_filled_mask(&self, index: usize) -> u16 {
        let mut col: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (i % SIZE == index % SIZE) && *i != index)
            .for_each(|(_, c)| col |= SudokuBoard::get_sudoku_value(&c));

        col
    }
    fn get_cell_box_filled_mask(&self, index: usize) -> u16 {
        let mut bxx: u16 = 0;

        self.board
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                3 * ((i / SIZE) / 3) + (i % SIZE) / 3
                    == 3 * ((index / SIZE) / 3) + (index % SIZE) / 3
                    && i != &index
            })
            .for_each(|(_, c)| bxx |= SudokuBoard::get_sudoku_value(&c));

        bxx
    }

    fn get_sudoku_value(v: &u16) -> u16 {
        match v.is_power_of_two() {
            true => v.trailing_zeros() as u16 + 1u16,
            false => 0u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ARR: [u16; 81] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    ];

    #[test]
    fn get_row() {
        let board = SudokuBoard { board: TEST_ARR };
        let index = 7;
        let solution: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 8];

        assert_eq!(board.get_cell_row_neighbors(index), solution);
    }

    #[test]
    fn get_col() {
        let board = SudokuBoard { board: TEST_ARR };
        let index = 20;
        let solution: [u16; 8] = [2, 11, 29, 38, 47, 56, 65, 74];

        assert_eq!(board.get_cell_column_neighbors(index), solution);
    }

    #[test]
    fn get_box() {
        let board = SudokuBoard { board: TEST_ARR };
        let index = 10;
        let solution: [u16; 8] = [0, 1, 2, 9, 11, 18, 19, 20];

        assert_eq!(board.get_cell_box_neighbors(index), solution);
    }
}
