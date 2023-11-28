use pyo3::prelude::*;
use pyo3::exceptions;
use std::collections::HashMap;

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

        // if move is not valid then return error
        if !self.is_valid_move(row, col)? {
            return Err(PyErr::new::<exceptions::PyValueError, _>(
                "invalid move",
            ));
        }

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

    fn has_liberties(&self, row: i32, col: i32, color: i32) -> PyResult<bool> {
        // check if the piece has liberties
        // init liberties as 4
        let mut liberties = 4;
        // if the piece is on the edge, then it has less liberties
        if row == 0 || row == self.board.board_size - 1 {
            liberties -= 1;
        }
        // if its not an edge piece, then check if the piece above and below are the same color
        if row > 0 && row < self.board.board_size - 1 {
            if self.board.board[row as usize - 1][col as usize] == color {
                liberties -= 1;
            }
            if self.board.board[row as usize + 1][col as usize] == color {
                liberties -= 1;
            }
        }
        
        if col == 0 || col == self.board.board_size - 1 {
            liberties -= 1;
        }

        // if the liberties are 0, then the piece is dead
        if liberties <= 0 {
            return Ok(false);
        }


        Ok(true)
    }


    // capture stones
    fn capture_stones(&mut self, row: i32, col: i32, color: i32) -> PyResult<()> {
        
        // we need to recursively map through adjacent pieces of different colors
        // and find all islands of the same color and if they have no liberties, then
        // we remove them from the board

        // create an array to store the visited pieces, do col * board_size + row, for the idx of is checked
        // initialize it wth all false of size board_size * board_size
        let mut visited = HashMap::new();

        // create a vector to store the pieces we need to remove
        let mut remove: Vec<(i32, i32)> = Vec::new();

        // call the recursive function
        self.capture_stones_recursive(row, col, color, &mut visited, remove)?;

        // remove the pieces from the board
        for (row, col) in remove {
            self.board.board[row as usize][col as usize] = EMPTY;
        } 

        return Ok(())
    }

    fn capture_stones_recursive(&mut self, row: i32, col: i32, color: i32, visited: &mut HashMap<(i32, i32), bool>, remove: &mut Vec<(i32, i32)>) -> PyResult<()> {
        if self.board.board[row as usize][col as usize] != color {
            return Ok(())
        }

        // if the piece is the same color, then add it to the visited vec
        visited.insert((row, col), true);

        // check if the piece has liberties
        if self.has_liberties(row, col, color)? {
            return Ok(())
        }

        // if the piece has no liberties, then add it to the remove vector
        remove.push((row, col));

        // check if the piece above is the same color and has not been visited
        if row > 0 && self.board.board[row as usize - 1][col as usize] == color && !visited.contains_key(&(row - 1, col)) {
            self.capture_stones_recursive(row - 1, col, color, visited, remove)?;
        }

        // check if the piece below is the same color and has not been visited
        if row < self.board.board_size - 1 && self.board.board[row as usize + 1][col as usize] == color && !visited.contains_key(&(row + 1, col)) {
            self.capture_stones_recursive(row + 1, col, color, visited, remove)?;
        }

        // check if the piece to the left is the same color and has not been visited
        if col > 0 && self.board.board[row as usize][col as usize - 1] == color && !visited.contains_key(&(row, col - 1)) {
            self.capture_stones_recursive(row, col - 1, color, visited, remove)?;
        }

        // check if the piece to the right is the same color and has not been visited
        if col < self.board.board_size - 1 && self.board.board[row as usize][col as usize + 1] == color && !visited.contains_key(&(row, col + 1)) {
            self.capture_stones_recursive(row, col + 1, color, visited, remove)?;
        }

        return Ok(())
    }

    // need function to check if move is valid
    fn is_valid_move(&self, row: i32, col: i32) -> PyResult<bool> {

        // if the location is not empty, return false 
        if self.board.board[row as usize][col as usize] != EMPTY {
            return Ok(false);
        }

        // if the location is empty, check if it has liberties
        if !self.has_liberties(row, col, self.current_player)? {
            return Ok(false);
        }


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

    // get observation space
    fn get_observation_space(&self) -> PyResult<i32> {
        Ok(self.observation_space)
    }

    // get action space
    fn get_action_space(&self) -> PyResult<i32> {
        Ok(self.action_space)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn go_gym(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GoEnv>()?;
    Ok(())
}

