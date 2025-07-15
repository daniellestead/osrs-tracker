use eframe::{egui, App};

use crate::models::goal::{GoalKind};
use crate::core::{GoalApp, GoalView};
use crate::ui::{GoalAction, render_goal_card, render_summary, render_new_goal};
use crate::ui::style::{get_visuals};

impl App for GoalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = get_visuals();
        ctx.set_visuals(visuals);

        // Save to file every time the UI is updated
        self.save_to_file();

        // Check for player sync result
        if let Some(rx) = &self.player.sync_rx {
            if let Ok(result) = rx.try_recv() {
                self.player.syncing = false;
                self.player.sync_rx = None;

                match result {
                    Ok(player_skills) => {
                        self.player.skills = player_skills;
                        self.player.sync_error = None;
                    }
                    Err(err) => {
                        self.player.sync_error = Some(err);
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Render summary section with player name and skills
            render_summary(ui, &mut self.player, &mut self.skill_icons);

            ui.separator();

            ui.heading("New Goal");
            let _ = render_new_goal( ui, &mut self.player, &mut self.goal, &mut self.goals);

            ui.separator();

            // Render all goals by kind
            if ui.button("Toggle view").clicked() {
                self.view = match self.view {
                    GoalView::Column => GoalView::Row,
                    GoalView::Row => GoalView::Column,
                };
            }
            match self.view {
                GoalView::Row => self.render_goals_row(ui),
                GoalView::Column => self.render_goals_column(ui),
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Total Goals: ");
                ui.label(self.goals.len().to_string());
            });
        });
    }
}

impl GoalApp {
    fn render_goals_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for kind in &[GoalKind::Daily, GoalKind::Skill, GoalKind::Diary, GoalKind::Drop, GoalKind::Resource] {
                ui.vertical(|ui| {
                    // Only render if there are goals of this kind
                    if self.goals.iter().any(|g| g.kind == *kind) {
                        ui.heading(format!("{:?}", kind));
                    }

                    let mut to_remove = None;

                    for i in 0..self.goals.len() {
                        if self.goals[i].kind != *kind {
                            continue;
                        }

                        let action = render_goal_card(ui, &mut self.skill_icons, &self.player, i, &mut self.goals[i]);
                        match action {
                            Some(GoalAction::StartEdit(index)) => self.goals[index].editing = true,
                            Some(GoalAction::CancelEdit) => self.goals[i].editing = false,
                            Some(GoalAction::Save) => {
                                self.goals[i].editing = false;
                                self.save_to_file();
                            }
                            Some(GoalAction::Remove(index)) => to_remove = Some(index),
                            None => {}
                        }
                    }

                    if let Some(i) = to_remove {
                        self.goals.remove(i);
                        self.save_to_file();
                    }
                });
            }
        });
    }
    fn render_goals_column(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            for kind in &[GoalKind::Daily, GoalKind::Skill, GoalKind::Diary, GoalKind::Drop, GoalKind::Resource] {
                ui.vertical(|ui| {
                    // Only render if there are goals of this kind
                    if self.goals.iter().any(|g| g.kind == *kind) {
                        ui.heading(format!("{:?}", kind));
                    }

                    let mut to_remove = None;

                    for i in 0..self.goals.len() {
                        if self.goals[i].kind != *kind {
                            continue;
                        }

                        let action = render_goal_card(ui, &mut self.skill_icons, &self.player, i, &mut self.goals[i]);
                        match action {
                            Some(GoalAction::StartEdit(index)) => self.goals[index].editing = true,
                            Some(GoalAction::CancelEdit) => self.goals[i].editing = false,
                            Some(GoalAction::Save) => {
                                self.goals[i].editing = false;
                                self.save_to_file();
                            }
                            Some(GoalAction::Remove(index)) => to_remove = Some(index),
                            None => {}
                        }
                    }

                    if let Some(i) = to_remove {
                        self.goals.remove(i);
                        self.save_to_file();
                    }
                });
            }
        });
    }
}