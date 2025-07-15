use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Goal {
    pub name: String,
    pub kind: GoalKind,
    pub editing: bool,
    pub purpose: Option<String>,
    pub complete: Option<bool>,
    pub quantity_goal: Option<u32>, 
    pub quantity_progress: Option<u32>,
    pub level_goal: Option<u32>,
    pub level_progress: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Copy)]
pub enum GoalKind {
    #[default]
    Skill,
    Daily,
    Diary,
    Drop,
    Resource,
    Quest,
}

impl GoalKind {
    pub fn show_quantity_goal(&self) -> bool {
        matches!(self, GoalKind::Resource)
    }

    pub fn show_level_goal(&self) -> bool {
        matches!(self, GoalKind::Skill)
    }
}
