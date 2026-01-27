use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::state::{GameState, InputMode, LastAction, MoveDir};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Move(MoveDir),
    MoveSelectorUp,
    MoveSelectorDown,
    Digit(u8),
    ToggleNotes,
    NewGame,
    ToggleGivens,
    ToggleHelp,
    ZoomIn,
    ZoomOut,
    Validate,
    Action(LastAction),
    SelectDifficulty,
    Quit,
}

pub fn command_from_key_event(event: KeyEvent) -> Option<Command> {
    if matches!(event.kind, KeyEventKind::Release) {
        return None;
    }

    match event.code {
        KeyCode::Left => Some(Command::Move(MoveDir::Left)),
        KeyCode::Right => Some(Command::Move(MoveDir::Right)),
        KeyCode::Up => Some(Command::Move(MoveDir::Up)),
        KeyCode::Down => Some(Command::Move(MoveDir::Down)),
        // Match the ui-mock.md footer: "Arrows/HJKL Move".
        KeyCode::Char('H') => Some(Command::Move(MoveDir::Left)),
        KeyCode::Char('L') => Some(Command::Move(MoveDir::Right)),
        KeyCode::Char('K') => Some(Command::Move(MoveDir::Up)),
        KeyCode::Char('J') => Some(Command::Move(MoveDir::Down)),
        // Selector navigation (can be used on level selector)
        KeyCode::Char('P') => Some(Command::MoveSelectorUp),
        KeyCode::Char('N') => Some(Command::MoveSelectorDown),
        // Enter to select difficulty
        KeyCode::Enter => Some(Command::SelectDifficulty),
        KeyCode::Char('n') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Command::NewGame)
        }
        KeyCode::Char('n') => Some(Command::ToggleNotes),
        KeyCode::Char('g') => Some(Command::ToggleGivens),
        KeyCode::Char('?') => Some(Command::ToggleHelp),
        KeyCode::Char('+') => Some(Command::ZoomIn),
        KeyCode::Char('-') => Some(Command::ZoomOut),
        KeyCode::Char('v') => Some(Command::Validate),
        KeyCode::Char('u') => Some(Command::Action(LastAction::Undo)),
        KeyCode::Char('r') => Some(Command::Action(LastAction::Redo)),
        KeyCode::Char('h') if event.modifiers.contains(KeyModifiers::CONTROL) => None,
        KeyCode::Char('h') => Some(Command::Action(LastAction::Hint)),
        KeyCode::Char('c') => Some(Command::Action(LastAction::Clear)),
        KeyCode::Char('s') => Some(Command::Action(LastAction::Save)),
        KeyCode::Char('o') => Some(Command::Action(LastAction::Load)),
        KeyCode::Char('q') => Some(Command::Quit),
        KeyCode::Char(d) if d.is_ascii_digit() => {
            let digit = d.to_digit(10)? as u8;
            if (1..=9).contains(&digit) {
                Some(Command::Digit(digit))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn apply_command(state: &mut GameState, command: Command) {
    match command {
        Command::Move(dir) => state.move_selection(dir),
        Command::MoveSelectorUp => {
            use crate::state::{DifficultyOption, Screen};
            if state.screen == Screen::LevelSelector {
                state.selector_selection = match state.selector_selection {
                    DifficultyOption::Easy => DifficultyOption::Expert,
                    DifficultyOption::Medium => DifficultyOption::Easy,
                    DifficultyOption::Hard => DifficultyOption::Medium,
                    DifficultyOption::Expert => DifficultyOption::Hard,
                    DifficultyOption::Resume => DifficultyOption::Expert,
                };
            }
        }
        Command::MoveSelectorDown => {
            use crate::state::{DifficultyOption, Screen};
            if state.screen == Screen::LevelSelector {
                state.selector_selection = match state.selector_selection {
                    DifficultyOption::Easy => DifficultyOption::Medium,
                    DifficultyOption::Medium => DifficultyOption::Hard,
                    DifficultyOption::Hard => DifficultyOption::Expert,
                    DifficultyOption::Expert => DifficultyOption::Easy,
                    DifficultyOption::Resume => DifficultyOption::Easy,
                };
            }
        }
        Command::SelectDifficulty => {
            use crate::state::{Difficulty, DifficultyOption, Screen};
            if state.screen == Screen::LevelSelector {
                match state.selector_selection {
                    DifficultyOption::Resume => {
                        if let Ok(loaded) = GameState::load_default() {
                            *state = loaded;
                            state.screen = Screen::Playing;
                        }
                    }
                    DifficultyOption::Easy => state.new_game(Difficulty::Easy),
                    DifficultyOption::Medium => state.new_game(Difficulty::Medium),
                    DifficultyOption::Hard => state.new_game(Difficulty::Hard),
                    DifficultyOption::Expert => state.new_game(Difficulty::Expert),
                }
            }
        }
        Command::Digit(digit) => {
            state.enter_digit(digit);
            use crate::state::Screen;
            if state.screen == Screen::Playing && state.check_win() {
                state.game_completed = true;
                if let Ok(mut leaderboard) = crate::leaderboard::Leaderboard::load() {
                    let entry = crate::leaderboard::LeaderboardEntry {
                        difficulty: state.difficulty,
                        time_seconds: state.started_at.elapsed().as_secs(),
                        completed_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
                    };
                    leaderboard.add_entry(entry);
                    let _ = leaderboard.save();
                }
                state.screen = Screen::Win;
            }
        }
        Command::ToggleNotes => {
            state.input_mode = match state.input_mode {
                InputMode::Normal => InputMode::Notes,
                InputMode::Notes => InputMode::Normal,
            };
        }
        Command::NewGame => state.new_game(state.difficulty),
        Command::ToggleGivens => state.show_givens = !state.show_givens,
        Command::ToggleHelp => state.help_visible = !state.help_visible,
        Command::ZoomIn => state.ui_zoom = state.ui_zoom.zoom_in(),
        Command::ZoomOut => state.ui_zoom = state.ui_zoom.zoom_out(),
        Command::Validate => state.validate_and_count_mistakes(),
        Command::Action(action) => {
            state.last_action = Some(action);
            if action == LastAction::Clear {
                state.clear_selected();
            }
            if action == LastAction::Hint
                && state.hints_left > 0
                && crate::puzzle::apply_hint(state)
            {
                state.hints_left -= 1;
            }
            if action == LastAction::Save {
                let _ = state.save_default();
            }
            if action == LastAction::Load {
                if let Ok(loaded) = GameState::load_default() {
                    *state = loaded;
                }
            }
        }
        Command::Quit => state.should_quit = true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn maps_navigation_shortcuts() {
        assert_eq!(
            command_from_key_event(key(KeyCode::Left)),
            Some(Command::Move(MoveDir::Left))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Right)),
            Some(Command::Move(MoveDir::Right))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Up)),
            Some(Command::Move(MoveDir::Up))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Down)),
            Some(Command::Move(MoveDir::Down))
        );

        assert_eq!(
            command_from_key_event(key(KeyCode::Char('H'))),
            Some(Command::Move(MoveDir::Left))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('J'))),
            Some(Command::Move(MoveDir::Down))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('K'))),
            Some(Command::Move(MoveDir::Up))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('L'))),
            Some(Command::Move(MoveDir::Right))
        );
    }

    #[test]
    fn maps_action_shortcuts() {
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('u'))),
            Some(Command::Action(LastAction::Undo))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('r'))),
            Some(Command::Action(LastAction::Redo))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('h'))),
            Some(Command::Action(LastAction::Hint))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('c'))),
            Some(Command::Action(LastAction::Clear))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('s'))),
            Some(Command::Action(LastAction::Save))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('o'))),
            Some(Command::Action(LastAction::Load))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('q'))),
            Some(Command::Quit)
        );
    }

    #[test]
    fn maps_digit_entry_and_toggles() {
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('1'))),
            Some(Command::Digit(1))
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('9'))),
            Some(Command::Digit(9))
        );
        assert_eq!(command_from_key_event(key(KeyCode::Char('0'))), None);

        assert_eq!(
            command_from_key_event(key(KeyCode::Char('n'))),
            Some(Command::ToggleNotes)
        );
        assert_eq!(
            command_from_key_event(key_mod(KeyCode::Char('n'), KeyModifiers::CONTROL)),
            Some(Command::NewGame)
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('g'))),
            Some(Command::ToggleGivens)
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('?'))),
            Some(Command::ToggleHelp)
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('v'))),
            Some(Command::Validate)
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('+'))),
            Some(Command::ZoomIn)
        );
        assert_eq!(
            command_from_key_event(key(KeyCode::Char('-'))),
            Some(Command::ZoomOut)
        );
    }

    #[test]
    fn zoom_commands_are_bounded() {
        let mut state = GameState::new(crate::state::Difficulty::Easy);
        state.ui_zoom = crate::state::UiZoom::Small;
        apply_command(&mut state, Command::ZoomOut);
        assert_eq!(state.ui_zoom, crate::state::UiZoom::Small);

        state.ui_zoom = crate::state::UiZoom::XLarge;
        apply_command(&mut state, Command::ZoomIn);
        assert_eq!(state.ui_zoom, crate::state::UiZoom::XLarge);
    }

    #[test]
    fn navigation_stays_within_bounds() {
        let mut state = GameState::new(crate::state::Difficulty::Easy);
        state.selection.row = 0;
        state.selection.col = 0;
        apply_command(&mut state, Command::Move(MoveDir::Left));
        apply_command(&mut state, Command::Move(MoveDir::Up));
        assert_eq!(state.selection.row, 0);
        assert_eq!(state.selection.col, 0);

        state.selection.row = 8;
        state.selection.col = 8;
        apply_command(&mut state, Command::Move(MoveDir::Right));
        apply_command(&mut state, Command::Move(MoveDir::Down));
        assert_eq!(state.selection.row, 8);
        assert_eq!(state.selection.col, 8);
    }
}
