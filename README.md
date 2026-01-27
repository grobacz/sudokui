# SUDOKUI

A feature-rich terminal-based Sudoku game with puzzle generation, difficulty levels, leaderboards, and persistent game state.

![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Features

### 🎮 Core Gameplay
- **4 Difficulty Levels**: Easy, Medium, Hard, and Expert
- **Infinite Puzzle Generation**: Unique puzzles generated on-demand
- **Candidate/Notes Mode**: Mark possible values in cells
- **Mistakes Tracking**: Visual feedback for incorrect entries
- **Hint System**: Get help when stuck (2 hints per game)
- **Input Validation**: Check your work at any time

### 📊 Difficulty Levels
- **Easy**: 36-38 givens (45 cells removed)
- **Medium**: 30-32 givens (51 cells removed)
- **Hard**: 24-27 givens (55 cells removed)
- **Expert**: 17-22 givens (60 cells removed)

All puzzles are guaranteed to have a unique solution.

### 🏆 Leaderboard System
- Tracks your best times for each difficulty level
- Saves top 20 entries per difficulty
- Displays completion date alongside time
- Persists across sessions
- Shows top 5 best times on win screen

### 💾 Auto-Save/Resume
- Auto-saves game state on exit
- Auto-resumes last game if within 8 hours
- Preserves:
  - Current puzzle state
  - Mistakes count
  - Hints remaining
  - Time elapsed
  - Input mode and zoom settings

### 🎯 User Interface
- **Level Selector Screen**: Choose difficulty or resume last game
- **Playing Screen**: Main game interface with grid and side panel
- **Win Screen**: Congratulations display with leaderboard
- **Zoom Control**: 4 zoom levels (Small, Medium, Large, XLarge)
- **Help Screen**: Quick reference for all controls
- **Responsive Layout**: Adapts to terminal size

## Installation

### Prerequisites
- Rust 1.80 or later
- Cargo (comes with Rust)

### Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd sudokui

# Build in release mode for optimal performance
cargo build --release

# The binary will be at target/release/sudokui
```

### Install Locally
```bash
cargo install --path .
```

## Usage

### Starting the Game
```bash
sudokui
```

On first launch (or when no recent save exists), you'll see the **Level Selector** screen.

### Level Selector
Use arrow keys to select a difficulty and press Enter to start:
- **Resume Game**: Only appears if you have a recent saved game (within 8 hours)
- **Easy/Medium/Hard/Expert**: Start a new game at selected difficulty

### Playing Screen

#### Controls

| Key | Action |
|-----|--------|
| **Arrow Keys** or **H/J/K/L** | Move selection |
| **1-9** | Enter digit in selected cell |
| **N** | Toggle notes mode |
| **Ctrl+N** | New game (reset current difficulty) |
| **G** | Toggle givens visibility |
| **V** | Validate and check mistakes |
| **+/-** | Zoom in/out |
| **?** | Toggle help screen |
| **H** | Hint (fills selected cell) |
| **C** | Clear selected cell |
| **S** | Save game |
| **O** | Load saved game |
| **Q** | Quit (auto-saves) |

#### Screen Elements

**Header**:
- Current difficulty
- Elapsed time (HH:MM:SS)
- Mistakes count (X/3)
- Max mistakes warning when applicable

**Main Grid**:
- Row labels (A-I) on left
- Column labels (1-9) on top
- Given cells shown in bold
- Wrong values shown in red
- Selected cell highlighted (reversed)
- Matching values highlighted in gray

**Side Panel**:
- Selected cell info
- Current mode (Normal/Notes)
- Hints remaining
- Action quick reference

#### Notes Mode
Press `N` to toggle notes mode. In notes mode:
- Digits add/remove candidates (pencil marks)
- Multiple candidates can be marked per cell
- Candidates shown in side panel

### Win Screen
When you complete a puzzle correctly:
- Congratulations message
- Final time and difficulty
- Number of mistakes made
- Top 5 leaderboard entries for that difficulty
- Press Enter to return to level selector
- Press Q to quit

## Save System

### Auto-Save Locations
Game state is saved to:
- **Linux**: `$XDG_STATE_HOME/sudokui/session.json` or `~/.local/state/sudokui/session.json`
- **Fallback**: `./sudokui-save.json` in current directory

### Auto-Resume
- Games are auto-resumed if quit within 8 hours
- Auto-resume only occurs for incomplete games
- After 8 hours, the save is considered stale and not auto-loaded

### Leaderboard Storage
Leaderboard is saved to:
- **Linux**: `$XDG_DATA_HOME/sudokui/leaderboard.json` or `~/.local/share/sudokui/leaderboard.json`
- **Fallback**: `./sudokui-leaderboard.json` in current directory

## Development

### Project Structure
```
src/
├── main.rs          # Entry point
├── app.rs           # Main application loop
├── state.rs         # Game state and core logic
├── input.rs         # Input handling and commands
├── ui.rs            # UI rendering (all screens)
├── puzzle.rs        # Puzzle generation algorithm
├── leaderboard.rs   # Leaderboard persistence
└── history.rs       # Action history (stub)
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Check compilation
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Technical Details

### Puzzle Generation Algorithm
1. Start with empty 9×9 grid
2. Fill diagonal 3×3 boxes independently
3. Solve the rest using backtracking
4. Remove cells symmetrically based on difficulty
5. Verify unique solution (count max 2 solutions)
6. Return puzzle and solution

### Difficulty Tuning
- Cells are removed in symmetrical pairs
- Higher difficulties remove more cells
- All puzzles guaranteed solvable by logic
- Expert puzzles may require advanced techniques

### State Management
- Game state serialized to JSON
- Includes full grid state (givens and user entries)
- Preserves candidates/notes
- Tracks timing and mistakes
- Stores last played timestamp

## Roadmap

### Completed ✅
- [x] Core game loop with grid rendering
- [x] Puzzle generation for all difficulties
- [x] Leaderboard system with persistence
- [x] Auto-save/resume functionality
- [x] Level selector screen
- [x] Win screen with leaderboard
- [x] Hint system
- [x] Mistakes tracking and validation
- [x] Zoom controls
- [x] Help screen

### Potential Future Enhancements
- [ ] Undo/Redo functionality (currently stubbed)
- [ ] Puzzle import/export
- [ ] Custom puzzle entry
- [ ] Statistics tracking (games played, win rate, etc.)
- [ ] Multiple color themes
- [ ] Advanced solving techniques visualization
- [ ] Puzzle notes sync with solving
- [ ] Multiplayer modes

## License

MIT License - feel free to use this project for learning or as a base for your own projects.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Areas for Contribution
1. **Undo/Redo System**: The history module exists but needs proper implementation
2. **Testing**: More comprehensive test coverage
3. **Documentation**: Inline code comments and examples
4. **Performance**: Optimization for puzzle generation
5. **Features**: Any of the roadmap items above

## Acknowledgments

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [Rand](https://github.com/rust-random/rand) - Random number generation
- [Serde](https://github.com/serde-rs/serde) - Serialization framework
- [Chrono](https://github.com/chronotope/chrono) - Date and time handling

## Author

Created as a terminal-based Sudoku game with focus on clean code and good UX.

---

**Enjoy your Sudoku experience!** 🎮
