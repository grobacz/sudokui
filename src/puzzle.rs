use crate::state::{Cell, Difficulty};

pub fn generate_puzzle(difficulty: Difficulty) -> [[Cell; 9]; 9] {
    let mut board = generate_complete_board();
    let cells_to_remove = match difficulty {
        Difficulty::Easy => 45,
        Difficulty::Medium => 51,
        Difficulty::Hard => 55,
        Difficulty::Expert => 60,
    };
    remove_cells(&mut board, cells_to_remove);
    board
}

fn generate_complete_board() -> [[Cell; 9]; 9] {
    let mut board = [[Cell::empty(); 9]; 9];
    fill_cell(&mut board, 0, 0);
    board
}

fn fill_cell(board: &mut [[Cell; 9]; 9], row: usize, col: usize) -> bool {
    if row == 9 {
        return true;
    }

    let next_row = if col == 8 { row + 1 } else { row };
    let next_col = if col == 8 { 0 } else { col + 1 };

    let mut numbers: Vec<u8> = (1..=9).collect();
    shuffle(&mut numbers);

    for num in numbers {
        if is_safe(board, row, col, num) {
            board[row][col].value = Some(num);
            board[row][col].given = true;
            if fill_cell(board, next_row, next_col) {
                return true;
            }
            board[row][col].value = None;
            board[row][col].given = false;
        }
    }

    false
}

fn is_safe(board: &[[Cell; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    #[allow(clippy::needless_range_loop)]
    for i in 0..9 {
        if board[row][i].value == Some(num) {
            return false;
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..9 {
        if board[i][col].value == Some(num) {
            return false;
        }
    }

    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[box_row + i][box_col + j].value == Some(num) {
                return false;
            }
        }
    }

    true
}

#[allow(clippy::ptr_arg)]
fn shuffle<T>(vec: &mut Vec<T>) {
    use rand::seq::SliceRandom;
    vec.shuffle(&mut rand::thread_rng());
}

fn remove_cells(board: &mut [[Cell; 9]; 9], cells_to_remove: usize) {
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            indices.push((i, j));
        }
    }
    shuffle(&mut indices);

    let mut removed = 0;
    for (row, col) in indices {
        if removed >= cells_to_remove {
            break;
        }

        let old_value = board[row][col].value;
        board[row][col].value = None;
        board[row][col].given = false;

        if count_solutions(board) == 1 {
            removed += 1;
        } else {
            board[row][col].value = old_value;
            board[row][col].given = true;
        }
    }
}

fn count_solutions(board: &[[Cell; 9]; 9]) -> u32 {
    let mut board_copy = *board;
    let mut count = 0;
    let mut row = 0;
    let mut col = 0;
    count_solutions_helper(&mut board_copy, &mut row, &mut col, &mut count, 2);
    count
}

fn count_solutions_helper(
    board: &mut [[Cell; 9]; 9],
    row: &mut usize,
    col: &mut usize,
    count: &mut u32,
    limit: u32,
) {
    if *count >= limit {
        return;
    }

    if *row == 9 {
        *count += 1;
        return;
    }

    let next_row = if *col == 8 { *row + 1 } else { *row };
    let next_col = if *col == 8 { 0 } else { *col + 1 };

    if board[*row][*col].value.is_some() {
        *row = next_row;
        *col = next_col;
        count_solutions_helper(board, &mut *row, &mut *col, count, limit);
        *row = if *col == 0 { *row - 1 } else { *row };
        *col = if *col == 0 { 8 } else { *col - 1 };
    } else {
        for num in 1..=10u8 {
            let num = num % 10;
            if num == 0 {
                continue;
            }
            if is_safe(board, *row, *col, num) {
                board[*row][*col].value = Some(num);
                *row = next_row;
                *col = next_col;
                count_solutions_helper(board, &mut *row, &mut *col, count, limit);
                *row = if *col == 0 { *row - 1 } else { *row };
                *col = if *col == 0 { 8 } else { *col - 1 };
                board[*row][*col].value = None;
            }
        }
    }
}

pub fn get_solution(board: &[[Cell; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let mut solution = [[0u8; 9]; 9];
    let mut board_copy = *board;

    if solve_board(&mut board_copy) {
        for row in 0..9 {
            for col in 0..9 {
                solution[row][col] = board_copy[row][col].value?;
            }
        }
        Some(solution)
    } else {
        None
    }
}

pub fn get_correct_value_for_cell(board: &[[Cell; 9]; 9], row: usize, col: usize) -> Option<u8> {
    let solution = get_solution(board)?;
    Some(solution[row][col])
}

fn solve_board(board: &mut [[Cell; 9]; 9]) -> bool {
    solve_helper(board, 0, 0)
}

fn solve_helper(board: &mut [[Cell; 9]; 9], row: usize, col: usize) -> bool {
    if row == 9 {
        return true;
    }

    let next_row = if col == 8 { row + 1 } else { row };
    let next_col = if col == 8 { 0 } else { col + 1 };

    if board[row][col].value.is_some() {
        return solve_helper(board, next_row, next_col);
    }

    for num in 1..=9 {
        if is_safe(board, row, col, num) {
            board[row][col].value = Some(num);
            if solve_helper(board, next_row, next_col) {
                return true;
            }
            board[row][col].value = None;
        }
    }

    false
}

pub fn apply_hint(state: &mut crate::state::GameState) -> bool {
    let cell = state.selected_cell();
    if cell.given || cell.value.is_some() {
        return false;
    }

    if let Some(correct_value) =
        get_correct_value_for_cell(&state.grid, state.selection.row, state.selection.col)
    {
        state.selected_cell_mut().value = Some(correct_value);
        true
    } else {
        false
    }
}
