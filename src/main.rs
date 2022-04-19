mod sudoku_board;
use sudoku_board::SudokuBoard;

fn main() {
    let problem = SudokuBoard::from_str(
        ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
    );
    problem.print();

    let solution = problem.solve();
    solution.print();
}
