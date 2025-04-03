pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check the main diagonal (top-left to bottom-right)
    let mut main_diag = true;
    for i in 0..3 {
        if table[i][i] != player {
            main_diag = false;
            break;
        }
    }
    if main_diag {
        return true;
    }
    // Check the anti-diagonal (top-right to bottom-left)
    let mut anti_diag = true;
    for i in 0..3 {
        if table[i][2 - i] != player {
            anti_diag = false;
            break;
        }
    }
    anti_diag
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in 0..3 {
        let mut win = true;
        for col in 0..3 {
            if table[row][col] != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        let mut win = true;
        for row in 0..3 {
            if table[row][col] != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}