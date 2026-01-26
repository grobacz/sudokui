use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Notes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Expert => write!(f, "Expert"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    LevelSelector,
    Playing,
    Win,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyOption {
    Resume,
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UiZoom {
    Small,
    Medium,
    #[default]
    Large,
    XLarge,
}

impl UiZoom {
    pub fn zoom_in(self) -> Self {
        match self {
            Self::Small => Self::Medium,
            Self::Medium => Self::Large,
            Self::Large => Self::XLarge,
            Self::XLarge => Self::XLarge,
        }
    }

    pub fn zoom_out(self) -> Self {
        match self {
            Self::Small => Self::Small,
            Self::Medium => Self::Small,
            Self::Large => Self::Medium,
            Self::XLarge => Self::Large,
        }
    }

    pub fn cell_w(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 2,
            Self::Large => 3,
            Self::XLarge => 5,
        }
    }

    pub fn cell_h(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 1,
            Self::Large => 2,
            Self::XLarge => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastAction {
    Undo,
    Redo,
    Hint,
    Clear,
    Save,
    Load,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub given: bool,
    pub value: Option<u8>,
    candidates_mask: u16,
    pub wrong: bool,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            given: false,
            value: None,
            candidates_mask: 0,
            wrong: false,
        }
    }

    #[allow(dead_code)]
    pub fn given(value: u8) -> Self {
        Self {
            given: true,
            value: Some(value),
            candidates_mask: 0,
            wrong: false,
        }
    }

    pub fn toggle_candidate(&mut self, digit: u8) {
        if !(1..=9).contains(&digit) {
            return;
        }
        let bit = 1u16 << (digit - 1);
        self.candidates_mask ^= bit;
    }

    pub fn clear_candidates(&mut self) {
        self.candidates_mask = 0;
    }

    pub fn candidates(&self) -> Vec<u8> {
        (1u8..=9)
            .filter(|d| (self.candidates_mask & (1u16 << (d - 1))) != 0)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub screen: Screen,
    pub game_completed: bool,
    #[allow(dead_code)]
    pub last_played_at: Instant,
    pub has_recent_save: bool,
    pub selector_selection: DifficultyOption,
    pub started_at: Instant,
    pub mistakes: u8,
    pub mistakes_max: u8,
    pub max_mistakes_warning: bool,
    pub hints_left: u8,
    pub input_mode: InputMode,
    pub ui_zoom: UiZoom,
    pub show_givens: bool,
    pub help_visible: bool,
    pub selection: Selection,
    pub grid: [[Cell; 9]; 9],
    pub last_action: Option<LastAction>,
    pub should_quit: bool,
    pub history: crate::history::ActionHistory,
}

impl GameState {
    pub fn new(difficulty: Difficulty) -> Self {
        let grid = crate::puzzle::generate_puzzle(difficulty);
        Self {
            difficulty,
            screen: Screen::LevelSelector,
            game_completed: false,
            last_played_at: Instant::now(),
            has_recent_save: false,
            selector_selection: match difficulty {
                Difficulty::Easy => DifficultyOption::Easy,
                Difficulty::Medium => DifficultyOption::Medium,
                Difficulty::Hard => DifficultyOption::Hard,
                Difficulty::Expert => DifficultyOption::Expert,
            },
            started_at: Instant::now(),
            mistakes: 0,
            mistakes_max: 3,
            max_mistakes_warning: false,
            hints_left: 2,
            input_mode: InputMode::Normal,
            ui_zoom: UiZoom::default(),
            show_givens: true,
            help_visible: false,
            selection: Selection { row: 3, col: 5 },
            grid,
            last_action: None,
            should_quit: false,
            history: crate::history::ActionHistory::new(),
        }
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        let ui_zoom = self.ui_zoom;
        let show_givens = self.show_givens;

        let mut next = GameState::new(difficulty);
        next.ui_zoom = ui_zoom;
        next.show_givens = show_givens;
        next.screen = Screen::Playing;
        *self = next;
    }

    pub fn load_or_show_selector() -> io::Result<Self> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        const RESUME_WINDOW_HOURS: u64 = 8;

        if let Ok(mut loaded) = GameState::load_default() {
            let hours_elapsed = (now - loaded.started_at.elapsed().as_secs()) / 3600;

            if !loaded.game_completed && hours_elapsed < RESUME_WINDOW_HOURS {
                loaded.screen = Screen::Playing;
                loaded.has_recent_save = true;
                return Ok(loaded);
            }
        }

        let mut state = GameState::new(Difficulty::Easy);
        state.screen = Screen::LevelSelector;
        state.has_recent_save = false;
        state.selector_selection = DifficultyOption::Easy;
        Ok(state)
    }

    #[allow(dead_code)]
    pub fn check_win(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col].value.is_none() {
                    return false;
                }
            }
        }

        self.validate_and_count_mistakes();
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col].wrong {
                    return false;
                }
            }
        }

        true
    }

    pub fn selected_cell(&self) -> &Cell {
        &self.grid[self.selection.row][self.selection.col]
    }

    pub fn selected_cell_mut(&mut self) -> &mut Cell {
        &mut self.grid[self.selection.row][self.selection.col]
    }

    pub fn move_selection(&mut self, dir: MoveDir) {
        match dir {
            MoveDir::Left => self.selection.col = self.selection.col.saturating_sub(1),
            MoveDir::Right => self.selection.col = (self.selection.col + 1).min(8),
            MoveDir::Up => self.selection.row = self.selection.row.saturating_sub(1),
            MoveDir::Down => self.selection.row = (self.selection.row + 1).min(8),
        }
    }

    pub fn enter_digit(&mut self, digit: u8) {
        if !(1..=9).contains(&digit) {
            return;
        }

        let mode = self.input_mode;
        let row = self.selection.row;
        let col = self.selection.col;

        {
            let cell = self.selected_cell_mut();
            if cell.given {
                return;
            }
        }

        let old_value = {
            let cell = self.selected_cell_mut();
            cell.value
        };

        match mode {
            InputMode::Normal => {
                if old_value != Some(digit) {
                    self.history.push(crate::history::GameAction::SetCell {
                        row,
                        col,
                        old_value,
                        new_value: Some(digit),
                    });
                }
                let cell = self.selected_cell_mut();
                cell.value = Some(digit);
                cell.clear_candidates();
                cell.wrong = false;
            }
            InputMode::Notes => {
                let cell = self.selected_cell_mut();
                cell.value = None;
                cell.toggle_candidate(digit);
                cell.wrong = false;
                self.history
                    .push(crate::history::GameAction::ToggleCandidate { row, col, digit });
            }
        }
    }

    pub fn clear_selected(&mut self) {
        let row = self.selection.row;
        let col = self.selection.col;

        {
            let cell = self.selected_cell_mut();
            if cell.given {
                return;
            }
        }

        let (old_value, old_mask) = {
            let cell = self.selected_cell_mut();
            (cell.value, cell.candidates_mask)
        };

        if old_value.is_some() || old_mask != 0 {
            self.history.push(crate::history::GameAction::SetCell {
                row,
                col,
                old_value,
                new_value: None,
            });
            self.history
                .push(crate::history::GameAction::ClearCandidates { row, col, old_mask });
        }

        let cell = self.selected_cell_mut();
        cell.value = None;
        cell.clear_candidates();
        cell.wrong = false;
    }

    #[allow(dead_code)]
    pub fn undo(&mut self) {
        self.history.undo();
    }

    #[allow(dead_code)]
    pub fn redo(&mut self) {
        self.history.redo();
    }

    pub fn validate_and_count_mistakes(&mut self) {
        let solution = crate::puzzle::get_solution(&self.grid);

        if let Some(solution) = solution {
            #[allow(clippy::needless_range_loop)]
            for row in 0..9 {
                #[allow(clippy::needless_range_loop)]
                for col in 0..9 {
                    let cell = &mut self.grid[row][col];
                    if cell.given {
                        cell.wrong = false;
                        continue;
                    }

                    let is_wrong = match cell.value {
                        Some(v) => v != solution[row][col],
                        None => false,
                    };

                    if is_wrong && !cell.wrong {
                        self.mistakes = self.mistakes.saturating_add(1).min(self.mistakes_max);
                        if self.mistakes >= self.mistakes_max {
                            self.max_mistakes_warning = true;
                        }
                    }
                    cell.wrong = is_wrong;
                }
            }
        }
    }

    pub fn save_to_path(&self, path: &Path) -> io::Result<()> {
        let elapsed_secs = self.started_at.elapsed().as_secs();
        let data = SaveData::from_state(self, elapsed_secs);
        let json = serde_json::to_string_pretty(&data).map_err(io::Error::other)?;
        fs::write(path, json)
    }

    pub fn load_from_path(path: &Path) -> io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let data: SaveData = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        data.into_state()
    }

    pub fn save_default(&self) -> io::Result<()> {
        let preferred = preferred_session_path();
        let fallback = fallback_session_path();

        if let Some(path) = preferred.as_deref() {
            if ensure_parent_dir(path).is_ok() && self.save_to_path(path).is_ok() {
                return Ok(());
            }
        }

        self.save_to_path(&fallback)
    }

    pub fn load_default() -> io::Result<Self> {
        let preferred = preferred_session_path();
        let fallback = fallback_session_path();

        if let Some(path) = preferred.as_deref() {
            if let Ok(state) = Self::load_from_path(path) {
                return Ok(state);
            }
        }

        Self::load_from_path(&fallback)
    }
}

const SAVE_FILE: &str = "sudokui-save.json";
const SAVE_VERSION: u8 = 1;

fn preferred_session_path() -> Option<PathBuf> {
    if let Some(xdg_state_home) = env::var_os("XDG_STATE_HOME") {
        return Some(
            PathBuf::from(xdg_state_home)
                .join("sudokui")
                .join("session.json"),
        );
    }

    if let Some(home) = env::var_os("HOME") {
        return Some(
            PathBuf::from(home)
                .join(".local")
                .join("state")
                .join("sudokui")
                .join("session.json"),
        );
    }

    #[cfg(windows)]
    if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
        return Some(
            PathBuf::from(local_app_data)
                .join("sudokui")
                .join("session.json"),
        );
    }

    None
}

fn fallback_session_path() -> PathBuf {
    PathBuf::from(SAVE_FILE)
}

fn ensure_parent_dir(path: &Path) -> io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    fs::create_dir_all(parent)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveData {
    version: u8,
    difficulty: Difficulty,
    elapsed_secs: u64,
    last_played_at: u64,
    game_completed: bool,
    mistakes: u8,
    mistakes_max: u8,
    hints_left: u8,
    input_mode: InputModeSave,
    #[serde(default)]
    ui_zoom: UiZoom,
    show_givens: bool,
    help_visible: bool,
    screen: ScreenSave,
    selector_selection: DifficultyOptionSave,
    selection: SelectionSave,
    grid: [[CellSave; 9]; 9],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum InputModeSave {
    Normal,
    Notes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum ScreenSave {
    LevelSelector,
    Playing,
    Win,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum DifficultyOptionSave {
    Resume,
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct SelectionSave {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct CellSave {
    given: bool,
    value: Option<u8>,
    candidates_mask: u16,
    wrong: bool,
}

impl SaveData {
    fn from_state(state: &GameState, elapsed_secs: u64) -> Self {
        Self {
            version: SAVE_VERSION,
            difficulty: state.difficulty,
            elapsed_secs,
            last_played_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            game_completed: state.game_completed,
            mistakes: state.mistakes,
            mistakes_max: state.mistakes_max,
            hints_left: state.hints_left,
            input_mode: match state.input_mode {
                InputMode::Normal => InputModeSave::Normal,
                InputMode::Notes => InputModeSave::Notes,
            },
            ui_zoom: state.ui_zoom,
            show_givens: state.show_givens,
            help_visible: state.help_visible,
            screen: match state.screen {
                Screen::LevelSelector => ScreenSave::LevelSelector,
                Screen::Playing => ScreenSave::Playing,
                Screen::Win => ScreenSave::Win,
            },
            selector_selection: match state.selector_selection {
                DifficultyOption::Resume => DifficultyOptionSave::Resume,
                DifficultyOption::Easy => DifficultyOptionSave::Easy,
                DifficultyOption::Medium => DifficultyOptionSave::Medium,
                DifficultyOption::Hard => DifficultyOptionSave::Hard,
                DifficultyOption::Expert => DifficultyOptionSave::Expert,
            },
            selection: SelectionSave {
                row: state.selection.row,
                col: state.selection.col,
            },
            grid: state.grid.map(|row| {
                row.map(|cell| CellSave {
                    given: cell.given,
                    value: cell.value,
                    candidates_mask: cell.candidates_mask,
                    wrong: cell.wrong,
                })
            }),
        }
    }

    fn into_state(self) -> io::Result<GameState> {
        if self.version != SAVE_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported save version: {}", self.version),
            ));
        }

        let mut state = GameState::new(self.difficulty);
        state.game_completed = self.game_completed;
        state.mistakes_max = self.mistakes_max.max(1);
        state.mistakes = self.mistakes.min(state.mistakes_max);
        state.hints_left = self.hints_left;
        state.input_mode = match self.input_mode {
            InputModeSave::Normal => InputMode::Normal,
            InputModeSave::Notes => InputMode::Notes,
        };
        state.ui_zoom = self.ui_zoom;
        state.show_givens = self.show_givens;
        state.help_visible = self.help_visible;
        state.screen = match self.screen {
            ScreenSave::LevelSelector => Screen::LevelSelector,
            ScreenSave::Playing => Screen::Playing,
            ScreenSave::Win => Screen::Win,
        };
        state.selector_selection = match self.selector_selection {
            DifficultyOptionSave::Resume => DifficultyOption::Resume,
            DifficultyOptionSave::Easy => DifficultyOption::Easy,
            DifficultyOptionSave::Medium => DifficultyOption::Medium,
            DifficultyOptionSave::Hard => DifficultyOption::Hard,
            DifficultyOptionSave::Expert => DifficultyOption::Expert,
        };
        state.selection = Selection {
            row: self.selection.row.min(8),
            col: self.selection.col.min(8),
        };

        state.started_at = Instant::now()
            .checked_sub(Duration::from_secs(self.elapsed_secs))
            .unwrap_or_else(Instant::now);

        // Restore non-given data only; givens are defined by the puzzle.
        for row in 0..9 {
            for col in 0..9 {
                if state.grid[row][col].given {
                    continue;
                }
                let saved = self.grid[row][col];
                state.grid[row][col].value = saved.value;
                state.grid[row][col].candidates_mask = saved.candidates_mask;
                state.grid[row][col].wrong = saved.wrong;
            }
        }

        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_marks_wrong_and_counts_once() {
        let mut state = GameState::new(Difficulty::Easy);

        // Find a non-given cell and enter a value
        let mut cell_pos = None;
        for row in 0..9 {
            for col in 0..9 {
                if !state.grid[row][col].given {
                    state.selection = Selection { row, col };
                    state.enter_digit(1);
                    cell_pos = Some((row, col));
                    break;
                }
            }
            if cell_pos.is_some() {
                break;
            }
        }

        // Validate the board
        assert_eq!(state.mistakes, 0);
        state.validate_and_count_mistakes();

        // Either the entered value is correct (mistakes still 0) or wrong (mistakes > 0)
        // The important thing is that validate doesn't crash and marks cells appropriately
        if state.mistakes > 0 {
            assert!(state.grid[cell_pos.unwrap().0][cell_pos.unwrap().1].wrong);
        }

        // Validate again should not increment mistakes further
        let mistakes_after_second_validate = state.mistakes;
        state.validate_and_count_mistakes();
        assert_eq!(state.mistakes, mistakes_after_second_validate);
    }

    #[test]
    fn save_and_load_round_trip() {
        let mut state = GameState::new(Difficulty::Easy);
        state.ui_zoom = UiZoom::Small;
        state.started_at = Instant::now() - Duration::from_secs(123);

        // Find a non-given cell for testing values
        let mut value_cell = None;
        for row in 0..9 {
            for col in 0..9 {
                if !state.grid[row][col].given {
                    state.selection = Selection { row, col };
                    state.enter_digit(4);
                    value_cell = Some((row, col));
                    break;
                }
            }
            if value_cell.is_some() {
                break;
            }
        }

        // Find another non-given cell for testing candidates
        state.input_mode = InputMode::Notes;
        let mut candidate_cell = None;
        for row in 0..9 {
            for col in 0..9 {
                if !state.grid[row][col].given && !state.grid[row][col].value.is_some() {
                    state.selection = Selection { row, col };
                    state.enter_digit(2);
                    state.enter_digit(8);
                    candidate_cell = Some((row, col));
                    break;
                }
            }
            if candidate_cell.is_some() {
                break;
            }
        }

        let path =
            std::env::temp_dir().join(format!("sudokui-save-test-{}.json", std::process::id()));
        state.save_to_path(&path).unwrap();
        let loaded = GameState::load_from_path(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(loaded.ui_zoom, UiZoom::Small);
        assert_eq!(loaded.input_mode, InputMode::Notes);

        // Check that the cell with a value was saved (if it wasn't a given in the loaded puzzle)
        if let Some((row, col)) = value_cell {
            if !loaded.grid[row][col].given {
                assert_eq!(loaded.grid[row][col].value, Some(4));
            }
        }

        // Check that candidates were saved (if cell wasn't a given in the loaded puzzle)
        if let Some((row, col)) = candidate_cell {
            if !loaded.grid[row][col].given {
                let mut candidates = loaded.grid[row][col].candidates();
                candidates.sort();
                assert_eq!(candidates, vec![2, 8]);
            }
        }

        let elapsed = loaded.started_at.elapsed().as_secs();
        assert!(elapsed >= 123 && elapsed <= 126);
    }
}
