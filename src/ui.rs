use std::time::Duration;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[allow(unused_imports)]
use crate::state::{Difficulty, DifficultyOption, GameState, InputMode, UiZoom};

pub fn render(frame: &mut Frame, state: &GameState) {
    let header_line = header_line(state);
    let footer_line =
        "Arrows/HJKL Move  1-9 Enter  n Notes  Ctrl+n New  +/- Zoom  ? Help".to_string();

    let grid_w = grid_width(state.ui_zoom) as u16;
    let grid_h = grid_height(state.ui_zoom) as u16;
    let side_w = side_panel_width(state) as u16;
    let side_h = side_panel_height() as u16;

    let body_w = grid_w + 1 + side_w;
    let body_h = grid_h.max(side_h);

    let inner_w = body_w
        .max(header_line.chars().count() as u16)
        .max(footer_line.chars().count() as u16);
    let inner_h = 2 + body_h + 2;

    let outer_w = inner_w.saturating_add(2);
    let outer_h = inner_h.saturating_add(2);

    let outer_area = top_left_rect_exact(outer_w, outer_h, frame.area());
    frame.render_widget(Block::default().borders(Borders::ALL), outer_area);

    let inner = Rect {
        x: outer_area.x + 1,
        y: outer_area.y + 1,
        width: outer_area.width.saturating_sub(2),
        height: outer_area.height.saturating_sub(2),
    };

    let grid_w = grid_w.min(inner.width);
    let (_sep_w, side_w) = match inner.width.saturating_sub(grid_w) {
        0 => (0, 0),
        1 => (1, 0),
        remaining => (1, side_w.min(remaining - 1)),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    render_header(frame, &header_line, chunks[0]);
    render_body(frame, state, chunks[1], grid_w, side_w);
    render_footer(frame, &footer_line, chunks[2]);

    if state.help_visible {
        render_help(frame, inner);
    }
}

fn header_line(state: &GameState) -> String {
    let elapsed = state.started_at.elapsed();
    let timer = format_hhmmss(elapsed);
    let warning = if state.max_mistakes_warning {
        " [MAX MISTAKES!]"
    } else {
        ""
    };
    format!(
        "SUDOKUI  {}  {}  Mistakes: {}/{}{}",
        state.difficulty, timer, state.mistakes, state.mistakes_max, warning
    )
}

fn render_header(frame: &mut Frame, header: &str, area: Rect) {
    let sep = "─".repeat(area.width as usize);
    let text = Text::from(vec![Line::from(header.to_string()), Line::from(sep)]);
    frame.render_widget(Paragraph::new(text), area);
}

fn render_footer(frame: &mut Frame, footer: &str, area: Rect) {
    let sep = "─".repeat(area.width as usize);
    let text = Text::from(vec![Line::from(sep), Line::from(footer.to_string())]);
    frame.render_widget(Paragraph::new(text), area);
}

fn render_body(frame: &mut Frame, state: &GameState, area: Rect, grid_w: u16, side_w: u16) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(grid_w),
            Constraint::Min(1),
            Constraint::Length(side_w),
        ])
        .split(area);

    render_grid(frame, state, body[0]);
    if side_w > 0 {
        render_side_panel(frame, state, body[2]);
    }
}

fn render_grid(frame: &mut Frame, state: &GameState, area: Rect) {
    frame.render_widget(Paragraph::new(Text::from(grid_text(state))), area);
}

fn grid_text(state: &GameState) -> Vec<Line<'static>> {
    let mut lines: Vec<Line> = Vec::new();

    let cell_w = state.ui_zoom.cell_w();
    let cell_h = state.ui_zoom.cell_h();

    lines.push(Line::from(column_header_line(cell_w)));
    lines.push(Line::from(top_border_line(cell_w)));

    let selected_cell = state.selected_cell();
    let selected_visible_value = if selected_cell.given && !state.show_givens {
        None
    } else {
        selected_cell.value
    };
    let highlight_value = selected_visible_value;

    for row in 0..9 {
        let digit_line = cell_h / 2;
        for subrow in 0..cell_h {
            if subrow == digit_line {
                lines.push(row_value_line(state, row, highlight_value, cell_w));
            } else {
                lines.push(row_fill_line(state, row, highlight_value, cell_w));
            }
        }

        if row == 2 || row == 5 {
            lines.push(Line::from(mid_border_line(cell_w)));
        }
    }

    lines.push(Line::from(bottom_border_line(cell_w)));

    lines
}

fn column_header_line(cell_w: usize) -> String {
    let mut out = String::from("    ");
    for col in 0..9 {
        let digit = (col + 1).to_string();
        out.push_str(&format!("{digit:^width$}", width = cell_w));
        if col == 2 || col == 5 {
            out.push_str("   ");
        } else if col != 8 {
            out.push(' ');
        }
    }
    out
}

fn top_border_line(cell_w: usize) -> String {
    border_line('┌', '┬', '┐', cell_w)
}

fn mid_border_line(cell_w: usize) -> String {
    border_line('├', '┼', '┤', cell_w)
}

fn bottom_border_line(cell_w: usize) -> String {
    border_line('└', '┴', '┘', cell_w)
}

fn border_line(left: char, mid: char, right: char, cell_w: usize) -> String {
    let block_w = (cell_w * 3) + 4;
    let mut out = String::from("  ");
    out.push(left);
    out.push_str(&"─".repeat(block_w));
    out.push(mid);
    out.push_str(&"─".repeat(block_w));
    out.push(mid);
    out.push_str(&"─".repeat(block_w));
    out.push(right);
    out
}

fn cell_visible_value(state: &GameState, row: usize, col: usize) -> Option<u8> {
    let cell = &state.grid[row][col];
    if cell.given && !state.show_givens {
        None
    } else {
        cell.value
    }
}

fn cell_style(
    state: &GameState,
    row: usize,
    col: usize,
    highlight_value: Option<u8>,
    value_line: bool,
) -> Style {
    let cell = &state.grid[row][col];
    let selected = row == state.selection.row && col == state.selection.col;
    let visible_value = cell_visible_value(state, row, col);

    let mut style = Style::default();
    if value_line && cell.given && state.show_givens {
        style = style.add_modifier(Modifier::BOLD);
    }
    if value_line && cell.wrong {
        style = style.fg(Color::Red);
    }
    if !selected && highlight_value.is_some() && visible_value == highlight_value {
        style = style.bg(Color::DarkGray);
    }
    if selected {
        style = style.add_modifier(Modifier::REVERSED);
    }
    style
}

fn row_value_line(
    state: &GameState,
    row: usize,
    highlight_value: Option<u8>,
    cell_w: usize,
) -> Line<'static> {
    let row_label = (b'A' + row as u8) as char;
    let mut spans = Vec::<Span>::new();
    spans.push(Span::raw(format!("{row_label} │ ")));

    for col in 0..9 {
        let visible_value = cell_visible_value(state, row, col);
        let ch = visible_value.map(|d| char::from(b'0' + d)).unwrap_or('.');
        let style = cell_style(state, row, col, highlight_value, true);
        spans.push(Span::styled(format!("{ch:^width$}", width = cell_w), style));

        if col == 2 || col == 5 {
            spans.push(Span::raw(" │ "));
        } else if col != 8 {
            spans.push(Span::raw(" "));
        } else {
            spans.push(Span::raw(" │"));
        }
    }

    Line::from(spans)
}

fn row_fill_line(
    state: &GameState,
    row: usize,
    highlight_value: Option<u8>,
    cell_w: usize,
) -> Line<'static> {
    let mut spans = Vec::<Span>::new();
    spans.push(Span::raw("  │ ".to_string()));

    for col in 0..9 {
        let style = cell_style(state, row, col, highlight_value, false);
        spans.push(Span::styled(" ".repeat(cell_w), style));

        if col == 2 || col == 5 {
            spans.push(Span::raw(" │ "));
        } else if col != 8 {
            spans.push(Span::raw(" "));
        } else {
            spans.push(Span::raw(" │"));
        }
    }

    Line::from(spans)
}

fn render_side_panel(frame: &mut Frame, state: &GameState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(7),
        ])
        .split(area);

    let selected = state.selected_cell();
    let visible_value = if selected.given && !state.show_givens {
        None
    } else {
        selected.value
    };
    let value = visible_value
        .map(|d| d.to_string())
        .unwrap_or_else(|| ".".into());
    let candidates = selected.candidates();
    let candidates_line = if candidates.is_empty() {
        "Candidates:".to_string()
    } else {
        format!(
            "Candidates: {}",
            candidates
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    };

    let selected_block = Block::default().borders(Borders::ALL);
    let selected_text = Text::from(vec![
        Line::from(format!(
            "Selected: R{} C{}",
            state.selection.row + 1,
            state.selection.col + 1
        )),
        Line::from(format!("Value: {value}")),
        Line::from(candidates_line),
    ]);
    frame.render_widget(
        Paragraph::new(selected_text).block(selected_block),
        chunks[0],
    );

    let mode = match state.input_mode {
        InputMode::Normal => "Normal",
        InputMode::Notes => "Notes",
    };

    let input_line = if matches!(state.input_mode, InputMode::Notes) && !candidates.is_empty() {
        format!(
            "Input: {}",
            candidates
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        "Input:".to_string()
    };

    let mode_block = Block::default().borders(Borders::ALL);
    let mode_text = Text::from(vec![
        Line::from(format!("Mode: {mode}")),
        Line::from(input_line),
        Line::from(format!("Hints left: {}", state.hints_left)),
    ]);
    frame.render_widget(Paragraph::new(mode_text).block(mode_block), chunks[1]);

    let actions_block = Block::default().borders(Borders::ALL).title("Actions");
    let actions_text = Text::from(vec![
        Line::from("u Undo   r Redo"),
        Line::from("h Hint   c Clear"),
        Line::from("v Check  s Save"),
        Line::from("o Load   q Quit"),
        Line::from("Ctrl+n New"),
    ]);
    frame.render_widget(Paragraph::new(actions_text).block(actions_block), chunks[2]);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_lines: [&str; 19] = [
        "Shortcuts",
        "",
        "Arrows / H J K L : Move",
        "1-9             : Enter digit",
        "n               : Notes mode",
        "Ctrl+n          : New game (reset)",
        "g               : Toggle givens",
        "v               : Check mistakes",
        "+ / -           : Zoom in/out",
        "?               : Toggle this help",
        "u/r             : Undo / Redo",
        "h               : Hint",
        "c               : Clear",
        "s/o             : Save / Load",
        "q               : Quit",
        "",
        "Persistence",
        "Startup         : Auto-resume last session",
        "Quit            : Auto-save session",
    ];
    let help_text = Text::from(
        help_lines
            .iter()
            .map(|s| Line::from((*s).to_string()))
            .collect::<Vec<_>>(),
    );
    let popup_area = help_rect(&help_lines, area);
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Help")
        .title_alignment(Alignment::Center);
    frame.render_widget(Paragraph::new(help_text).block(block), popup_area);
}

fn help_rect(lines: &[&str], area: Rect) -> Rect {
    let max_line = lines
        .iter()
        .map(|s| s.chars().count() as u16)
        .max()
        .unwrap_or(0);
    let required_w = max_line.saturating_add(4);
    let required_h = (lines.len() as u16).saturating_add(2);
    centered_rect_exact(required_w, required_h, area)
}

fn format_hhmmss(duration: Duration) -> String {
    let secs = duration.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

fn centered_rect_exact(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}

fn top_left_rect_exact(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

fn grid_width(zoom: UiZoom) -> usize {
    // Row width is: 18 + (9 * cell_w).
    18 + (9 * zoom.cell_w())
}

fn grid_height(zoom: UiZoom) -> usize {
    // 1 col header + 1 top border + (9 * cell_h) + 2 inner borders + 1 bottom border
    5 + (9 * zoom.cell_h())
}

fn side_panel_height() -> usize {
    // Selected (3) + Mode (3) + Actions (5), each with 2 border rows.
    (3 + 2) + (3 + 2) + (5 + 2)
}

fn side_panel_width(state: &GameState) -> usize {
    let selected = state.selected_cell();
    let visible_value = if selected.given && !state.show_givens {
        None
    } else {
        selected.value
    };
    let value = visible_value
        .map(|d| d.to_string())
        .unwrap_or_else(|| ".".into());
    let candidates = selected.candidates();
    let candidates_line = if candidates.is_empty() {
        "Candidates:".to_string()
    } else {
        format!(
            "Candidates: {}",
            candidates
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    };

    let input_line = if matches!(state.input_mode, InputMode::Notes) && !candidates.is_empty() {
        format!(
            "Input: {}",
            candidates
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        "Input:".to_string()
    };

    let mode_str = match state.input_mode {
        InputMode::Normal => "Normal",
        InputMode::Notes => "Notes",
    };

    let max_inner = [
        format!(
            "Selected: R{} C{}",
            state.selection.row + 1,
            state.selection.col + 1
        ),
        format!("Value: {value}"),
        candidates_line,
        format!("Mode: {mode_str}"),
        input_line,
        format!("Hints left: {}", state.hints_left),
        "Actions".to_string(),
        "u Undo   r Redo".to_string(),
        "h Hint   c Clear".to_string(),
        "v Check  s Save".to_string(),
        "o Load   q Quit".to_string(),
        "Ctrl+n New".to_string(),
    ]
    .into_iter()
    .map(|s| s.chars().count())
    .max()
    .unwrap_or(0);

    // Add borders.
    max_inner.saturating_add(2)
}

pub fn render_selector(frame: &mut Frame, state: &GameState) {
    use crate::state::DifficultyOption;

    let mut options_list: Vec<DifficultyOption> = vec![
        DifficultyOption::Easy,
        DifficultyOption::Medium,
        DifficultyOption::Hard,
        DifficultyOption::Expert,
    ];

    if state.has_recent_save {
        options_list.insert(0, DifficultyOption::Resume);
    }

    let options_map = |opt: &DifficultyOption| -> &'static str {
        match opt {
            DifficultyOption::Easy => "36-38 givens",
            DifficultyOption::Medium => "30-32 givens",
            DifficultyOption::Hard => "24-27 givens",
            DifficultyOption::Expert => "17-22 givens",
            DifficultyOption::Resume => "Resume Game",
        }
    };

    let selected_style = |opt: &DifficultyOption| -> Style {
        let mut style = Style::default();
        if *opt == state.selector_selection {
            style = style.add_modifier(Modifier::REVERSED);
        }
        style
    };

    let option_count = options_list.len();

    let box_w = 25u16;
    let box_h = 2u16 + option_count as u16;
    let box_x = 80u16.saturating_sub(box_w) / 2;
    let box_area = Rect {
        x: box_x,
        y: frame.area().height.saturating_sub(box_h) / 2,
        width: box_w,
        height: box_h,
    };

    frame.render_widget(Clear, box_area);

    for (i, opt) in options_list.iter().enumerate() {
        let is_resume = matches!(opt, DifficultyOption::Resume);
        let can_select = is_resume || !state.has_recent_save;

        let label = options_map(opt);
        let label = format!(
            "  {}  {}",
            if *opt == state.selector_selection {
                ">"
            } else {
                " "
            },
            label
        );

        let style = if can_select {
            selected_style(opt)
        } else {
            Style::default()
        };

        frame.render_widget(
            Paragraph::new(Line::from(label)).style(style),
            Rect {
                x: box_x,
                y: box_area.y + 2 + i as u16,
                width: box_w,
                height: 1,
            },
        );
    }

    let footer_lines = [
        Line::from("Arrows to move, Enter to select, q to quit"),
        Line::from(""),
    ];

    for (i, line) in footer_lines.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(line.clone()),
            Rect {
                x: box_x,
                y: box_area.y + 2 + option_count as u16 + 1 + i as u16,
                width: box_w,
                height: 1,
            },
        );
    }
}

pub fn render_win(frame: &mut Frame, state: &GameState) {
    #[allow(unused_imports)]
    use crate::state::Difficulty;

    let elapsed = state.started_at.elapsed();
    let timer = format_hhmmss(elapsed);
    let difficulty = state.difficulty.to_string();

    let stats_lines = [
        Line::from("Congratulations! Puzzle Solved!"),
        Line::from(""),
        Line::from(format!("Time:      {}", timer)),
        Line::from(format!("Difficulty: {}", difficulty)),
        Line::from(format!(
            "Mistakes:   {}/{}",
            state.mistakes, state.mistakes_max
        )),
        Line::from(""),
    ];

    let leaderboard_lines = match crate::leaderboard::Leaderboard::load() {
        Ok(leaderboard) => {
            let top = leaderboard.get_top_for_difficulty(state.difficulty, 5);
            if top.is_empty() {
                vec![
                    Line::from("Top 5 - This difficulty:".to_string()),
                    Line::from(" (no completed games yet)"),
                ]
            } else {
                let mut lines = vec![Line::from(format!("Top 5 - {}:", difficulty))];
                for (i, entry) in top.iter().enumerate() {
                    let time_str = format_hhmmss(Duration::from_secs(entry.time_seconds));
                    let date_str = &entry.completed_at;
                    lines.push(Line::from(format!(
                        "{}.  {}   {}",
                        i + 1,
                        time_str,
                        date_str
                    )));
                }
                lines
            }
        }
        Err(_) => {
            vec![
                Line::from(format!("Top 5 - {}:", difficulty)),
                Line::from("(leaderboard unavailable)"),
            ]
        }
    };

    let footer_lines = vec![
        Line::from("Press Enter to continue"),
        Line::from("Press q to quit"),
    ];

    let header_w = stats_lines
        .iter()
        .chain(leaderboard_lines.iter())
        .map(|l| l.width())
        .max()
        .unwrap_or(0)
        .max(60)
        .saturating_add(4);

    let body_w = header_w.saturating_add(2);
    let body_h = stats_lines.len() as u16 + leaderboard_lines.len() as u16;

    let popup_w = body_w.saturating_add(4);
    let popup_h = body_h.saturating_add(2);

    let popup_area = Rect {
        x: (frame.area().width.saturating_sub(popup_w as u16) / 2),
        y: (frame.area().height.saturating_sub(popup_h as u16) / 2),
        width: popup_w as u16,
        height: popup_h as u16,
    };

    let inner_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + 1,
        width: popup_area.width.saturating_sub(4),
        height: popup_area.height.saturating_sub(2),
    };

    let max_inner_w = inner_area.width;

    for (i, line) in stats_lines
        .iter()
        .chain(leaderboard_lines.iter())
        .enumerate()
    {
        let text = Text::from(vec![line.clone()]);
        frame.render_widget(
            Paragraph::new(text).alignment(Alignment::Left),
            Rect {
                x: inner_area.x,
                y: inner_area.y + i as u16,
                width: max_inner_w,
                height: 1,
            },
        );
    }

    let footer_block = Block::default().title("Press Enter to continue");
    let footer_text = Text::from(footer_lines);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(footer_block, popup_area);
    frame.render_widget(
        Paragraph::new(footer_text).alignment(Alignment::Center),
        inner_area,
    );
}
