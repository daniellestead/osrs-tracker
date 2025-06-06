use eframe::egui;
use crate::{model::goal::Goal, ui::style::Theme};

pub enum GoalAction {
    StartEdit(usize),
    CancelEdit,
    Save,
    Remove(usize),
}

pub fn render_goal_card(
    ui: &mut egui::Ui,
    i: usize,
    goal: &mut Goal,
    is_editing: bool,
) -> Option<GoalAction> {
    let mut action = None;

    egui::Frame::group(ui.style())
        .fill(Theme::BG)
        .stroke(Theme::BORDER)
        .rounding(Theme::CARD_ROUNDING)
        .inner_margin(Theme::CARD_MARGIN)
        .show(ui, |ui| {
            if is_editing {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(goal.name.clone())
                        .color(egui::Color32::from_rgb(128, 0, 128)),
                    );

                    ui.add(egui::TextEdit::singleline(goal.purpose.get_or_insert_with(String::new))
                        .hint_text("Purpose (optional)")
                    );

                    if goal.kind.show_quantity_goal() {
                        let mut quantity = goal.quantity_goal.unwrap_or(1);
                        if ui
                            .add(egui::DragValue::new(&mut quantity).prefix("Quantity: "))
                            .changed()
                        {
                            goal.quantity_goal = Some(quantity);
                        }
                    }

                    if goal.kind.show_level_goal() {
                        let mut level = goal.level_goal.unwrap_or(1);
                        if ui
                            .add(egui::DragValue::new(&mut level).prefix("Level: "))
                            .changed()
                        {
                            goal.level_goal = Some(level.clamp(1, 99));
                        }
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            action = Some(GoalAction::Save);
                        }
                        if ui.button("Cancel").clicked() {
                            action = Some(GoalAction::CancelEdit);
                        }
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(format!(
                                "{}",
                                if let Some(lvl) = goal.level_goal {
                                    format!("{} {}", lvl, goal.name)
                                } else {
                                    goal.name.clone()
                                }
                            ))
                            .color(egui::Color32::from_rgb(128, 0, 128)),
                        );

                        if let Some(amount) = goal.quantity_goal {
                            ui.label(format!("Quantity: {}", amount));
                        }

                        if let Some(purpose) = &goal.purpose {
                            if !purpose.is_empty() {
                                ui.label(format!("Purpose: {}", purpose));
                            }
                        }
                    });

                    if ui.button("Edit").clicked() {
                        action = Some(GoalAction::StartEdit(i));
                    }

                    if ui.button("Clear").clicked() {
                        action = Some(GoalAction::Remove(i));
                    }
                });
            }
        });

    action
}

