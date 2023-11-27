use pyo3::prelude::*;
use pyo3::exceptions;

// set constants we can use for types
const EMPTY: i32 = 0;
const BLACK: i32 = 1;
const WHITE: i32 = 2;

// need to create a board that is a 2d array of size board_size * board_size
#[pyclass]
struct Board {
    // board will hold  0, 1, 2 for empty, black, white
    board_size: i32,
    board: Vec<Vec<i32>>,
}


#[pyclass]
struct GoEnv {
    // neet to at the state of the game:
    action_space: i32,
    observation_space: i32,
    board: Board,
    current_player: i32,
}


#[pymethods]
impl GoEnv {

    #[new]
    fn py_new(board_size: i32) -> Self {
        GoEnv {
            action_space: board_size * board_size,
            observation_space: board_size * board_size,
            board: Board {
                board_size: board_size,
                board: vec![vec![0; board_size as usize]; board_size as usize],
            },
            current_player: BLACK,
        }
    }

    fn reset(&self) -> PyResult<()> {
        Ok(())
    }

    // returns observation, reward, done, info
    fn step(&mut self, action: i32) -> PyResult<(i32, f32, bool, i32)> {

       if action > self.action_space || action < 0 {
            return Err(PyErr::new::<exceptions::PyValueError, _>(
                "action size out of bounds",
            ));
        }

        let row = action / self.board.board_size;
        let col = action % self.board.board_size;

        // make the move
        self.board.board[row as usize][col as usize] = 1;

        // change turn
        self.change_turn()?;

        Ok((0, 0.0, false, 0))
    }

    fn change_turn(&mut self) -> PyResult<()> {
        if self.current_player == BLACK {
            self.current_player = WHITE;
        } else {
            self.current_player = BLACK;
        }
        Ok(())
    }

    // need function to check if move is valid
    fn is_valid_move(&self, action: i32) -> PyResult<bool> {
        Ok(true)
    }

    fn render(&self) -> PyResult<()> {
        // Define characters for board elements
        let horizontal_line = "---";
        let vertical_line = " | ";
        let intersection = "+";
        let empty_space = " ";
        let stone_black = "B";
        let stone_white = "W";
    
        // Get the number of rows and columns
        let rows = self.board.board.len();
        let cols = if rows > 0 { self.board.board[0].len() } else { 0 };
    
        // Top border
        print!("{}", intersection);
        for _ in 0..cols {
            print!("{}{}", horizontal_line, intersection);
        }
        println!();
        for row in &self.board.board {
            print!("{}", "| ");
            for &col in row {
                // Determine the character to print based on board value
                let symbol = match col {
                    0 => empty_space,
                    1 => stone_black,
                    2 => stone_white,
                    _ => "?"        
                };
                print!("{}{}", symbol, vertical_line);
            }
            println!();
            print!("{}", intersection);
            for _ in 0..cols {
                print!("{}{}", horizontal_line, intersection);
            }
            println!();
        }
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn go_gym(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GoEnv>()?;
    Ok(())
}

