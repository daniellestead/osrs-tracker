use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use eframe::egui::{self, TextureHandle};
use std::collections::HashMap;
use crate::ui::icons;
use crate::models::goal::Goal;
use crate::models::player::Player;


#[derive(Serialize, Deserialize)]
pub struct GoalApp {
    pub player: Player,
    pub goal: Goal,
    pub goals: Vec<Goal>,
    pub view: GoalView,
    #[serde(skip)]
    pub skill_icons: HashMap<String, TextureHandle>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GoalView {
    Column,
    Row
}

impl Default for GoalApp {
    fn default() -> Self {
        GoalApp {
            player: Player::default(),
            goal: Goal::default(),
            goals: Vec::new(),
            view: GoalView::Column,
            skill_icons: HashMap::new(),
        }
    }
}

impl GoalApp {
    // Load player name and goals from file
    pub fn new(ctx: &egui::Context) -> Result<Self, String> {
        let skill_icons = icons::load_all_icons(ctx);
        if let Some(path) = Self::storage_path() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok((player_name, goals, view)) = serde_json::from_str::<(String, Vec<Goal>, GoalView)>(&data) {
                    let mut player = Player {
                        name: player_name.clone(),
                        editing: false,
                        syncing: false,
                        sync_error: None,
                        ..Default::default()
                    };

                    if player_name.is_empty() {
                        player.editing = true;
                    }

                    // Start syncing player
                    let (tx, rx) = std::sync::mpsc::channel();
                    if !player_name.is_empty() {
                        let name_clone = player_name.clone();
                        player.syncing = true;
                        std::thread::spawn(move || {
                            let result = tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(crate::api::get_player_stats(&name_clone))
                                .map_err(|e| e.to_string());
                            let _ = tx.send(result);
                        });
                        player.sync_rx = Some(rx);
                    }
                
                    return Ok(Self {
                        player,
                        goal: Goal::default(),
                        goals,
                        view,
                        skill_icons,
                    });
                } else if let Err(e) = serde_json::from_str::<(String, Vec<Goal>)>(&data) {
                    eprintln!("Failed to parse saved data: {}", e);
                }
            } else {
                eprintln!("Failed to read saved data file");
            }
        }

        Ok(Self {
            skill_icons,
            ..Default::default()
        })
    }

    // Save player name and goals to file
    pub fn save_to_file(&self) {
        if let Some(path) = Self::storage_path() {
            let data = serde_json::to_string(&(&self.player.name, &self.goals, &self.view)).unwrap_or_default();
            let _ = fs::write(path, data); // Ignore errors for now
        }
    }

    fn storage_path() -> Option<PathBuf> {
        dirs::data_local_dir().map(|mut path| {
            path.push("goals.json");
            path
        })
    }
}
