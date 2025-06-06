// use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Goal {
    pub name: String,
    pub kind: GoalKind,

    pub purpose: Option<String>,
    pub complete: Option<bool>,
    pub quantity_goal: Option<u32>, 
    pub quantity_progress: Option<u32>,
    pub level_goal: Option<u32>,
    pub level_progress: Option<u32>,
   // pub due_date: Option<NaiveDate>,
}

impl Goal {
    // pub fn new(name: String, kind: GoalKind) -> Self {
    //     Self {
    //         name,
    //         kind,
    //         ..Default::default()
    //     }
    // }

    // pub fn set_purpose(&mut self, purpose: String) {
    //     self.purpose = Some(purpose);
    // }

    // pub fn set_complete(&mut self) {
    //     self.complete = Some(true);
    // }

    // pub fn is_completed(&self) -> bool {
    //     self.complete.unwrap_or(false)
    // }

    // pub fn set_quantity_goal(&mut self, amount: u32) {
    //     self.quantity_goal = Some(amount);
    // }

    // pub fn update_quantity_progress(&mut self, amount: u32) {
    //     self.quantity_progress = Some(amount);
    // }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Copy)]
pub enum GoalKind {
    #[default]
    Daily,
    Skill,
    Diary,
    Drop,
    Resource,
}

impl GoalKind {
    pub fn show_quantity_goal(&self) -> bool {
        matches!(self, GoalKind::Resource)
    }

    pub fn show_level_goal(&self) -> bool {
        matches!(self, GoalKind::Skill)
    }
}
