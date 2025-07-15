use eframe::egui::{self, TextureHandle};
use std::collections::HashMap;
use crate::{
    models::goal::{Goal, GoalKind},
    models::player::{Player, get_xp_for_level},
    ui::style::Theme,
};

pub enum GoalAction {
    StartEdit(usize),
    CancelEdit,
    Save,
    Remove(usize),
}

pub fn render_goal_card(
    ui: &mut egui::Ui,
    skill_icons: &HashMap<String, TextureHandle>,
    player: &Player,
    i: usize,
    goal: &mut Goal,
) -> Option<GoalAction> {
    let mut action = None;
    egui::Frame::group(ui.style())
        .fill(Theme::CARD_FILL)
        .stroke(Theme::CARD_BORDER)
        .rounding(Theme::CARD_ROUNDING)
        .inner_margin(Theme::CARD_MARGIN)
        .show(ui, |ui| {
            ui.set_width(Theme::CARD_WIDTH);
            ui.set_height(Theme::CARD_HEIGHT);

            action = if goal.editing {
                render_goal_edit(ui, goal)
            } else {
                render_goal_display(ui, skill_icons, player, i, goal)
            };
        });
    action
}

fn render_goal_edit(ui: &mut egui::Ui, goal: &mut Goal) -> Option<GoalAction> {
    let mut action = None;
    ui.vertical(|ui| {
        ui.label(egui::RichText::new(goal.name.clone()).color(Theme::BLACK));

        ui.add(egui::TextEdit::singleline(goal.purpose.get_or_insert_with(String::new))
            .hint_text("Purpose (optional)"),
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

        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("Save").clicked() {
                action = Some(GoalAction::Save);
            }
            if ui.button("Cancel").clicked() {
                action = Some(GoalAction::CancelEdit);
            }
        });
    });

    action
}

fn render_goal_display(
    ui: &mut egui::Ui,
    skill_icons: &HashMap<String, TextureHandle>,
    player: &Player,
    i: usize,
    goal: &Goal,
) -> Option<GoalAction> {
    let mut action = None;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if goal.kind == GoalKind::Skill && !goal.name.is_empty() {
                if let Some(tex) = skill_icons.get(&goal.name.to_lowercase()) {
                    ui.image(tex);
                } else {
                    ui.label("No icon");
                }
            }

            let title = if let Some(lvl) = goal.level_goal {
                format!("{} {}", lvl, goal.name)
            } else {
                goal.name.clone()
            };

            ui.label(egui::RichText::new(title).color(Theme::BLACK));

            if let Some(amount) = goal.quantity_goal {
                ui.label(format!("Quantity: {}", amount));
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.button("Edit").clicked() {
                    action = Some(GoalAction::StartEdit(i));
                }
                if ui.button("Clear").clicked() {
                    action = Some(GoalAction::Remove(i));
                }
            });
        });

        // Level progress
        if let Some(lvl) = goal.level_goal {
            if let Some(skill) = player.skills.iter().find(|s| s.name == goal.name) {
                // Show XP left progress
                let xp_needed = get_xp_for_level(lvl as u8).unwrap_or(0);
                let xp_progress = skill.xp as f32 / xp_needed as f32;
                let colour = if xp_progress >= 1.0 {
                    Theme::LIGHT_GREEN
                } else if xp_progress >= 0.75 {
                    Theme::KHAKI
                } else if xp_progress >= 0.5{
                    Theme::LIGHT_SALMON
                } else {
                    Theme::LIGHT_CORAL
                };
                ui.add(egui::ProgressBar::new(xp_progress)
                    .desired_width(Theme::CARD_WIDTH)
                    .show_percentage()
                    .fill(colour)
                );
            }
        }

        if let Some(purpose) = &goal.purpose {
            if !purpose.is_empty() {
                ui.label(egui::RichText::new(purpose).color(Theme::DARK_ORCHID));
            }
        }
    });

    action
}