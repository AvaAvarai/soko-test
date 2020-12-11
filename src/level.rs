use super::board;

pub fn load_level(_level_num: i32) -> board::Board {
    // Test Level
    let mut test_level: Vec< Vec<char>> = vec![vec!['.'; 11]; 11];
    
    test_level[2][2] = 'P';
    test_level[6][2] = 'B';
    test_level[3][2] = 'B';

    for num_a in 0..11 as usize {
        for num_b in 0..11 as usize {
            if num_a == 0 || num_a == 10 || num_b == 0 || num_b == 10 {
                test_level[num_a][num_b] = '#';
            }
        }   
    }
    board::Board::new(test_level, (2, 2), vec![(4, 4), (6, 4)], 11, 0, false)
}
