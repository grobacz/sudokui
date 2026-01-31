use std::{io, time::{Duration, Instant}};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{input, state::GameState, ui};

const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(30);

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut state = GameState::load_or_show_selector()?;
    let mut last_autosave = Instant::now();

    loop {
        terminal.draw(|frame| match state.screen {
            crate::state::Screen::LevelSelector => ui::render_selector(frame, &state),
            crate::state::Screen::Playing => ui::render(frame, &state),
            crate::state::Screen::Win => ui::render_win(frame, &state),
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if let Some(command) = input::command_from_key_event(key_event) {
                    input::apply_command(&mut state, command);
                }
            }
        }

        if last_autosave.elapsed() >= AUTOSAVE_INTERVAL {
            if matches!(state.screen, crate::state::Screen::Playing) && !state.game_completed {
                let _ = state.save_default();
            }
            last_autosave = Instant::now();
        }

        if state.should_quit {
            let _ = state.save_default();
            return Ok(());
        }
    }
}
