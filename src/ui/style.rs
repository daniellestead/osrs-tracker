use eframe::egui::{Color32, Margin, Rounding, Stroke};

pub struct Theme;

impl Theme {
    pub const BG: Color32 = Color32::from_rgb(79, 79, 79);
    //pub const TEXT_PURPLE: Color32 = Color32::from_rgb(128, 0, 128);
    pub const BORDER: Stroke = Stroke { width: 1.0, color: Color32::LIGHT_GRAY };
    pub const CARD_MARGIN: Margin = Margin::same(8.0);
    pub const CARD_ROUNDING: Rounding = Rounding::same(10.0);
}