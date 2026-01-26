mod app;
mod history;
mod input;
mod leaderboard;
mod puzzle;
mod state;
mod ui;

fn main() -> std::io::Result<()> {
    app::run()
}
