pub struct Board {
    board: Vec<Vec<char>>,
    player: (usize, usize),
    goals: Vec<(usize, usize)>
}

impl Board {
    pub fn new(board: Vec<Vec<char>>, player: (usize, usize), goals: Vec<(usize, usize)>) -> Self {
        Board {board, player, goals}
    }

    pub fn get_goals(&self) -> &Vec<(usize, usize)> {
        &self.goals
    }

    pub fn get_board(&self) -> &Vec<Vec<char>> {
        &self.board
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let test_coords = ((self.player.0 as i32 + dx) as usize, (self.player.1 as i32 + dy) as usize);
        let test_square = self.board[test_coords.0][test_coords.1];
        match test_square {
            '#' => return,
            'B' => {
                let box_test_coords = ((test_coords.0 as i32 + dx) as usize, (test_coords.1 as i32 + dy) as usize);
                let box_test_square = self.board[box_test_coords.0][box_test_coords.1];
                if box_test_square != '#' && box_test_square != 'B' {
                    self.board[self.player.0][self.player.1] = '.';
                    self.board[test_coords.0][test_coords.1] = 'P';
                    self.board[box_test_coords.0][box_test_coords.1] = 'B';
                    self.player = test_coords;
                }
            },
            _ => {
                self.board[self.player.0][self.player.1] = '.';
                self.board[test_coords.0][test_coords.1] = 'P';
                self.player = test_coords;
            }
        }
    }
}
