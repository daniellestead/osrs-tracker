use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::model::goal::{Goal};

#[derive(Serialize, Deserialize, Default)]
pub struct GoalApp {
    pub goal: Goal,
    pub goals: Vec<Goal>,
     #[serde(skip)] // don't persist across app sessions
    pub editing: Option<usize>,
}

impl GoalApp {
    pub fn save_to_file(&self) {
        if let Ok(json) = serde_json::to_string(&self.goals) {
            if let Some(path) = Self::storage_path() {
                let _ = fs::write(path, json); // Ignore errors for now
            }
        }
    }

    pub fn load_from_file() -> Self {
        if let Some(path) = Self::storage_path() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(goals) = serde_json::from_str(&data) {
                    return GoalApp {
                        goals,
                        ..Default::default()
                    };
                }
            }
        }
        Self::default()
    }

    fn storage_path() -> Option<PathBuf> {
        dirs::data_local_dir().map(|mut path| {
            path.push("goals.json");
            path
        })
    }
}
