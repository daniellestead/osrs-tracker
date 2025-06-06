use eframe::{egui, App};

use crate::model::goal::{Goal, GoalKind};
use crate::core::GoalApp;
use crate::ui::{render_goal_card, GoalAction};

impl App for GoalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OSRS Goals");

            // New goal name inputs
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Input for goal name
                    ui.add(egui::TextEdit::singleline(&mut self.goal.name)
                        .hint_text("Name")
                    );

                    // Optional purpose input
                    ui.add(egui::TextEdit::singleline(self.goal.purpose.get_or_insert_with(String::new))
                        .hint_text("Purpose (optional)")
                    );

                    // Show quantity goal input if required
                    if self.goal.kind.show_quantity_goal() {
                        let mut quantity = self.goal.quantity_goal.unwrap_or(1);
                        let drag = egui::DragValue::new(&mut quantity).prefix("Quantity: ");
                        if ui.add(drag).changed() {
                            self.goal.quantity_goal = Some(quantity);
                        }
                    }

                    // Show skill level goal input if required
                    if self.goal.kind.show_level_goal() {
                        let mut level = self.goal.level_goal.unwrap_or(1);
                        let drag = egui::DragValue::new(&mut level).prefix("Level: ");
                        if ui.add(drag).changed() {
                            level = level.clamp(1, 99);
                            self.goal.level_goal = Some(level);
                        }
                    }
                });

                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.goal.kind))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.goal.kind, GoalKind::Daily, "Daily");
                        ui.selectable_value(&mut self.goal.kind, GoalKind::Skill, "Skill");
                        ui.selectable_value(&mut self.goal.kind, GoalKind::Diary, "Diary");
                        ui.selectable_value(&mut self.goal.kind, GoalKind::Drop, "Drop");
                        ui.selectable_value(&mut self.goal.kind, GoalKind::Resource, "Resource");
                    }
                );

            
                if ui.button("Add").clicked() && !self.goal.name.is_empty() {
                    self.goals.push(Goal {
                        name: self.goal.name.clone(),
                        kind: self.goal.kind,
                        quantity_goal: self.goal.quantity_goal,
                        purpose: self.goal.purpose.clone(),
                        level_goal: self.goal.level_goal,
                        ..Default::default()
                    });
                    // Clear the goal input fields after adding
                    self.goal.name.clear();
                    self.goal.purpose = None;
                    self.goal.quantity_goal = None;
                    self.goal.level_goal = None;
                    self.save_to_file();
                }
            });

            // Render all goals by kind
            for kind in &[GoalKind::Daily, GoalKind::Skill, GoalKind::Diary, GoalKind::Drop, GoalKind::Resource] {
                ui.separator();
                ui.heading(format!("{:?}", kind));

                let mut to_remove = None;

                for i in 0..self.goals.len() {
                    if self.goals[i].kind != *kind {
                        continue;
                    }

                    let is_editing = self.editing == Some(i);
                    let action = render_goal_card(ui, i, &mut self.goals[i], is_editing);

                    match action {
                        Some(GoalAction::StartEdit(index)) => self.editing = Some(index),
                        Some(GoalAction::CancelEdit) => self.editing = None,
                        Some(GoalAction::Save) => {
                            self.editing = None;
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

