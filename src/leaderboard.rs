use serde::{Deserialize, Serialize};
use std::{env, fs, io, path::PathBuf};

use crate::state::Difficulty;

#[allow(dead_code)]
pub const LEADERBOARD_SIZE: usize = 20;
#[allow(dead_code)]
pub const TOP_DISPLAY_COUNT: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub difficulty: Difficulty,
    pub time_seconds: u64,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn load() -> io::Result<Self> {
        let path = leaderboard_path()?;
        if !path.exists() {
            return Ok(Self::new());
        }

        let json = fs::read_to_string(&path)?;
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    #[allow(dead_code)]
    pub fn save(&self) -> io::Result<()> {
        let path = leaderboard_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(&path, json)
    }

    #[allow(dead_code)]
    pub fn add_entry(&mut self, entry: LeaderboardEntry) {
        self.entries.push(entry);

        self.entries.sort_by(|a, b| {
            if a.difficulty != b.difficulty {
                return std::cmp::Ordering::Equal;
            }
            a.time_seconds.cmp(&b.time_seconds)
        });

        let mut per_difficulty_count = [0usize; 4];
        self.entries.retain(|entry| {
            let idx = match entry.difficulty {
                Difficulty::Easy => 0,
                Difficulty::Medium => 1,
                Difficulty::Hard => 2,
                Difficulty::Expert => 3,
            };
            per_difficulty_count[idx] += 1;
            per_difficulty_count[idx] <= LEADERBOARD_SIZE
        });
    }

    pub fn get_top_for_difficulty(
        &self,
        difficulty: Difficulty,
        n: usize,
    ) -> Vec<&LeaderboardEntry> {
        self.entries
            .iter()
            .filter(|e| e.difficulty == difficulty)
            .take(n)
            .collect()
    }
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::new()
    }
}

fn leaderboard_path() -> io::Result<PathBuf> {
    if let Some(xdg_data_home) = env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg_data_home)
            .join("sudokui")
            .join("leaderboard.json"));
    }

    if let Some(home) = env::var_os("HOME") {
        return Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("sudokui")
            .join("leaderboard.json"));
    }

    #[cfg(windows)]
    if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
        return Ok(PathBuf::from(local_app_data)
            .join("sudokui")
            .join("leaderboard.json"));
    }

    Ok(PathBuf::from("sudokui-leaderboard.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entry() {
        let mut leaderboard = Leaderboard::new();

        let entry1 = LeaderboardEntry {
            difficulty: Difficulty::Easy,
            time_seconds: 100,
            completed_at: "2026-01-26T00:00:00Z".to_string(),
        };
        leaderboard.add_entry(entry1);

        let entry2 = LeaderboardEntry {
            difficulty: Difficulty::Easy,
            time_seconds: 50,
            completed_at: "2026-01-26T01:00:00Z".to_string(),
        };
        leaderboard.add_entry(entry2);

        let top = leaderboard.get_top_for_difficulty(Difficulty::Easy, 5);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].time_seconds, 50);
        assert_eq!(top[1].time_seconds, 100);
    }

    #[test]
    fn test_per_difficulty_limit() {
        let mut leaderboard = Leaderboard::new();

        for i in 0..25 {
            leaderboard.add_entry(LeaderboardEntry {
                difficulty: Difficulty::Easy,
                time_seconds: 100 + i as u64,
                completed_at: "2026-01-26T00:00:00Z".to_string(),
            });
        }

        let easy_entries = leaderboard.get_top_for_difficulty(Difficulty::Easy, 100);
        assert_eq!(easy_entries.len(), LEADERBOARD_SIZE);
    }

    #[test]
    fn test_multiple_difficulties() {
        let mut leaderboard = Leaderboard::new();

        leaderboard.add_entry(LeaderboardEntry {
            difficulty: Difficulty::Easy,
            time_seconds: 100,
            completed_at: "2026-01-26T00:00:00Z".to_string(),
        });

        leaderboard.add_entry(LeaderboardEntry {
            difficulty: Difficulty::Hard,
            time_seconds: 200,
            completed_at: "2026-01-26T01:00:00Z".to_string(),
        });

        let easy_top = leaderboard.get_top_for_difficulty(Difficulty::Easy, 5);
        let hard_top = leaderboard.get_top_for_difficulty(Difficulty::Hard, 5);

        assert_eq!(easy_top.len(), 1);
        assert_eq!(hard_top.len(), 1);
    }
}
