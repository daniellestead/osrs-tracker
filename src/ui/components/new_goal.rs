use eframe::egui::{self};
use crate::models::{goal::{Goal, GoalKind}, player::Player};

pub fn render_new_goal(
    ui: &mut egui::Ui,
    player: &mut Player,
    new_goal: &mut Goal,
    goals: &mut Vec<Goal>,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("kind", "")
            .width(150.0)
            .height(150.0)
            .selected_text(format!("{:?}", new_goal.kind))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut new_goal.kind, GoalKind::Daily, "Daily");
                ui.selectable_value(&mut new_goal.kind, GoalKind::Skill, "Skill");
                ui.selectable_value(&mut new_goal.kind, GoalKind::Diary, "Diary");
                ui.selectable_value(&mut new_goal.kind, GoalKind::Drop, "Drop");
                ui.selectable_value(&mut new_goal.kind, GoalKind::Resource, "Resource");
            });

        // Input for goal name
        if new_goal.kind == GoalKind::Skill {
            egui::ComboBox::new("skill", "")
                .width(150.0)
                .height(150.0)
                .selected_text(new_goal.name.clone())
                .show_ui(ui, |ui| {
                    for skill in player.skills.iter() {
                        ui.selectable_value(&mut new_goal.name, skill.name.clone(), &skill.name);
                    }
                });
        } else {
            ui.add(egui::TextEdit::singleline(&mut new_goal.name)
                .hint_text("Name")
            );
        }

        if ui.button("Add").clicked() && !new_goal.name.is_empty() {
            goals.push(Goal {
                name: new_goal.name.clone(),
                kind: new_goal.kind,
                quantity_goal: new_goal.quantity_goal,
                purpose: new_goal.purpose.clone(),
                level_goal: new_goal.level_goal,
                ..Default::default()
            });
            // Clear the goal input fields after adding
            new_goal.name.clear();
            new_goal.purpose = None;
            new_goal.quantity_goal = None;
            new_goal.level_goal = None;
        }
    });

    // Optional purpose input
    ui.add(egui::TextEdit::singleline(new_goal.purpose.get_or_insert_with(String::new))
        .hint_text("Purpose (optional)")
        .desired_width(300.0)
    );

    // Show quantity goal input if required
    if new_goal.kind.show_quantity_goal() {
        let mut quantity = new_goal.quantity_goal.unwrap_or(1);
        let drag = egui::DragValue::new(&mut quantity).prefix("Quantity: ");
        if ui.add(drag).changed() {
            new_goal.quantity_goal = Some(quantity);
        }
    }

    // Show skill level goal input if required
    if new_goal.kind.show_level_goal() {
        let mut level = new_goal.level_goal.unwrap_or(1);
        let drag = egui::DragValue::new(&mut level).prefix("Level: ");
        if ui.add(drag).changed() {
            level = level.clamp(1, 99);
            new_goal.level_goal = Some(level);
        }
    }
}