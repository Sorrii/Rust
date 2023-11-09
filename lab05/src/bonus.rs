#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Gameboard {
    pub board: [[Cell; 11]; 11],
}

impl Gameboard {
    pub fn new_board(board: [[Cell; 11]; 11]) -> Gameboard {
        Gameboard { board }
    }

    pub fn get_neighbors_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        let directions = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for &(r, c) in &directions {
            let x = row as isize + r;
            let y = col as isize + c;

            if x >= 0
                && x < 10
                && y >= 0
                && y < 10
                && self.board[x as usize][y as usize] == Cell::Alive
            {
                count += 1;
            }
        }

        count
    }

    pub fn create_next_gen(&mut self) {
        let mut new_board = self.board.clone();

        for col in 0..10 {
            for row in 0..10 {
                let neighbors_count = self.get_neighbors_count(row, col);

                new_board[col][row] = if (self.board[col][row] == Cell::Alive
                    && (neighbors_count == 2 || neighbors_count == 3))
                    || (self.board[col][row] == Cell::Dead && neighbors_count == 3)
                {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }

        self.board = new_board;
    }

    pub fn print(&self) {
        for row in &self.board {
            for &cell in row {
                print!("{}", if cell == Cell::Alive { "1" } else { "0" });
            }
            print!("\n");
        }
    }
}
