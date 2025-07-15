use eframe::egui::{self, Frame, RichText, TextureHandle};
use std::collections::HashMap;
use crate::{models::player::Player, ui::style::Theme};
use crate::api::get_player_stats;

pub fn render_summary(
    ui: &mut egui::Ui,
    player: &mut Player,
    skill_icons: & mut HashMap<String, TextureHandle>,
) {
    // Render player name and sync button
    ui.horizontal(|ui| {
        ui.strong("Player:");
        if player.editing {
            ui.add(egui::TextEdit::singleline(&mut player.name)
                .hint_text("Enter your player name")
                .desired_width(200.0),
            );
            if ui.button("Save").clicked() && !player.name.is_empty() {
                player.editing = false;
            }
        } else {
            ui.strong(RichText::new(&player.name));
            if ui.button("Edit").clicked() {
                player.editing = true;
                player.name.clear();
            }
        }

        if ui.button("Sync").clicked() && !player.name.is_empty() && player.sync_rx.is_none() {
            let name = player.name.clone();
            player.syncing = true;

            let (tx, rx) = std::sync::mpsc::channel();
            player.sync_rx = Some(rx);

            std::thread::spawn(move || {
                let result = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(get_player_stats(&name))
                    .map_err(|e| e.to_string());
                let _ = tx.send(result);
            });
        }
    });
    
    // If still waiting on result, show empty loading state
    if player.syncing {
        ui.add(egui::Spinner::new()
            .size(20.0)
            .color(Theme::DARK_ORCHID)
        );
    }

    // Display skills in a grid-like layout        
    ui.vertical(|ui| {
        let columns = 12;
        let mut skill_iter = player.skills.iter();
        while let Some(_) = skill_iter.clone().next() {
            ui.horizontal(|ui| {
                for _ in 0..columns {
                    if let Some(skill) = skill_iter.next() {
                        // Box frame with some margin and border
                        Frame::group(ui.style())
                            .fill(Theme::WHITE)
                            .stroke(Theme::CARD_BORDER)
                            .rounding(Theme::CARD_ROUNDING)
                            .show(ui, |ui| {
                                ui.set_width(30.0);
                                ui.vertical_centered(|ui| {
                                    if let Some(tex) = skill_icons.get(&skill.name.to_lowercase()) {
                                        ui.image(tex);
                                    } else {
                                        ui.label("No icon");
                                    }
                                    ui.label(RichText::new(skill.level.to_string()).color(Theme::BLACK));
                                });
                            }
                        );
                    }
                }
            });
        }
    });
}