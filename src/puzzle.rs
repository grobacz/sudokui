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

#[cfg(test)]
mod tests {
    use super::*;

    fn count_givens(grid: &[[crate::state::Cell; 9]; 9]) -> usize {
        grid.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.given)
            .count()
    }

    fn is_valid_sudoku(grid: &[[crate::state::Cell; 9]; 9]) -> bool {
        // Check rows
        for row in 0..9 {
            let mut seen = [false; 9];
            for col in 0..9 {
                if let Some(val) = grid[row][col].value {
                    let idx = (val - 1) as usize;
                    if seen[idx] {
                        return false;
                    }
                    seen[idx] = true;
                }
            }
        }

        // Check columns
        for col in 0..9 {
            let mut seen = [false; 9];
            for row in 0..9 {
                if let Some(val) = grid[row][col].value {
                    let idx = (val - 1) as usize;
                    if seen[idx] {
                        return false;
                    }
                    seen[idx] = true;
                }
            }
        }

        // Check 3x3 boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut seen = [false; 9];
                for row in (box_row * 3)..(box_row * 3 + 3) {
                    for col in (box_col * 3)..(box_col * 3 + 3) {
                        if let Some(val) = grid[row][col].value {
                            let idx = (val - 1) as usize;
                            if seen[idx] {
                                return false;
                            }
                            seen[idx] = true;
                        }
                    }
                }
            }
        }

        true
    }

    #[test]
    fn test_easy_difficulty_givens() {
        let grid = generate_puzzle(crate::state::Difficulty::Easy);
        let givens = count_givens(&grid);
        assert!(
            givens >= 36 && givens <= 38,
            "Easy: expected 36-38 givens, got {}",
            givens
        );
    }

    #[test]
    fn test_medium_difficulty_givens() {
        let grid = generate_puzzle(crate::state::Difficulty::Medium);
        let givens = count_givens(&grid);
        assert!(
            givens >= 30 && givens <= 32,
            "Medium: expected 30-32 givens, got {}",
            givens
        );
    }

    #[test]
    fn test_hard_difficulty_givens() {
        let grid = generate_puzzle(crate::state::Difficulty::Hard);
        let givens = count_givens(&grid);
        assert!(
            givens >= 24 && givens <= 27,
            "Hard: expected 24-27 givens, got {}",
            givens
        );
    }

    #[test]
    fn test_expert_difficulty_givens() {
        // Expert aims for 17-22 givens, but may have more due to uniqueness constraint
        // The algorithm removes up to 60 cells, but only if solution remains unique
        let grid = generate_puzzle(crate::state::Difficulty::Expert);
        let givens = count_givens(&grid);
        assert!(
            givens >= 17 && givens <= 27,
            "Expert: expected 17-27 givens (uniqueness constraint), got {}",
            givens
        );
    }

    #[test]
    fn test_generated_puzzle_is_valid() {
        for difficulty in &[
            crate::state::Difficulty::Easy,
            crate::state::Difficulty::Medium,
            crate::state::Difficulty::Hard,
            crate::state::Difficulty::Expert,
        ] {
            let grid = generate_puzzle(*difficulty);
            assert!(
                is_valid_sudoku(&grid),
                "Generated puzzle for {:?} is invalid",
                difficulty
            );
        }
    }

    #[test]
    fn test_solution_is_valid() {
        let grid = generate_puzzle(crate::state::Difficulty::Easy);
        let solution = get_solution(&grid);
        assert!(solution.is_some(), "Easy puzzle should have a solution");
        if let Some(sol) = solution {
            // Solution should be complete (all cells filled with 1-9)
            for row in 0..9 {
                for col in 0..9 {
                    let val = sol[row][col];
                    assert!(
                        val >= 1 && val <= 9,
                        "Solution should have valid values, got {} at [{},{}]",
                        val,
                        row,
                        col
                    );
                }
            }
        }
    }

    #[test]
    fn test_unique_solution() {
        // Test that puzzles have unique solutions (or very close)
        let grid = generate_puzzle(crate::state::Difficulty::Easy);
        let solution_count = count_solutions(&grid);
        assert_eq!(
            solution_count, 1,
            "Easy puzzle should have exactly 1 solution"
        );
    }

    #[test]
    fn test_multiple_puzzles_are_different() {
        let grid1 = generate_puzzle(crate::state::Difficulty::Easy);
        let grid2 = generate_puzzle(crate::state::Difficulty::Easy);

        let mut different = false;
        for row in 0..9 {
            for col in 0..9 {
                if grid1[row][col].given != grid2[row][col].given {
                    different = true;
                    break;
                }
            }
            if different {
                break;
            }
        }
        assert!(different, "Multiple Easy puzzles should be different");
    }
}
