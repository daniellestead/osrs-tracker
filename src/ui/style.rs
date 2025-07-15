use eframe::egui::{style::{Widgets, WidgetVisuals}, Color32, Margin, Rounding, Stroke, Visuals};

pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Colours all from https://oldschool.runescape.wiki/w/RuneScape:Colour_codes
    pub const PURPLE: Color32 = Color32::from_rgb(128, 0, 128);
    pub const POWDER_BLUE: Color32 = Color32::from_rgb(176, 224, 230);
    pub const ROYAL_BLUE: Color32 = Color32::from_rgb(65, 105, 225); 
    pub const LIGHT_BLUE: Color32 = Color32::from_rgb(173, 216, 230);
    pub const SKY_BLUE: Color32 = Color32::from_rgb(135, 206, 235);
    pub const STEEL_BLUE: Color32 = Color32::from_rgb(70, 130, 180);
    pub const YELLOW_GREEN: Color32 = Color32::from_rgb(154, 205, 50);
    pub const PALE_GREEN: Color32 = Color32::from_rgb(152, 251, 152); 
    pub const TAN: Color32 = Color32::from_rgb(210, 180, 140);
    pub const SLATE_BLUE: Color32 = Color32::from_rgb(112, 128, 144);
    pub const DARK_SLATE_BLUE: Color32 = Color32::from_rgb(72, 61, 139);
    pub const PAPAYA_WHIP: Color32 = Color32::from_rgb(255, 239, 213);
    // Orchid colours
    pub const LIGHT_ORCHID: Color32 = Color32::from_rgb(230, 230, 250);
    pub const ORCHID: Color32 = Color32::from_rgb(218, 112, 214);
    pub const MEDIUM_ORCHID: Color32 = Color32::from_rgb(186, 85, 211);
    pub const DARK_ORCHID: Color32 = Color32::from_rgb(153, 50, 204);

    // White colours
    pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    pub const SNOW: Color32 = Color32::from_rgb(255, 250, 250);
    pub const HONEYDEW: Color32 = Color32::from_rgb(240, 255, 240);
    pub const MINT_CREAM: Color32 = Color32::from_rgb(245, 255, 250);
    pub const AZURE: Color32 = Color32::from_rgb(240, 255, 255);
    pub const ALICE_BLUE: Color32 = Color32::from_rgb(240, 248, 255);
    pub const GHOST_WHITE: Color32 = Color32::from_rgb(248, 248, 255);
    pub const WHITE_SMOKE: Color32 = Color32::from_rgb(245, 245, 245);
    pub const SEASHELL: Color32 = Color32::from_rgb(255, 245, 238);
    pub const BEIGE: Color32 = Color32::from_rgb(245, 245, 220);
    pub const OLD_LACE: Color32 = Color32::from_rgb(253, 245, 230);
    pub const FLORAL_WHITE: Color32 = Color32::from_rgb(255, 250, 240);
    pub const IVORY: Color32 = Color32::from_rgb(255, 255, 240);
    pub const ANTIQUE_WHITE: Color32 = Color32::from_rgb(250, 235, 215);
    pub const LINEN: Color32 = Color32::from_rgb(250, 240, 230);
    pub const LAVENDER_BLUSH: Color32 = Color32::from_rgb(255, 240, 245);
    pub const MISTY_ROSE: Color32 = Color32::from_rgb(255, 228, 225);
    
    // Grey colours
    pub const GAINSBORO: Color32 = Color32::from_rgb(220, 220, 220);
    pub const LIGHT_GRAY: Color32 = Color32::from_rgb(211, 211, 211);
    pub const SILVER: Color32 = Color32::from_rgb(192, 192, 192);
    pub const DARK_GRAY: Color32 = Color32::from_rgb(169, 169, 169);
    pub const GRAY: Color32 = Color32::from_rgb(128, 128, 128);
    pub const DIM_GRAY: Color32 = Color32::from_rgb(105, 105, 105);
    pub const LIGHT_SLATE_GRAY: Color32 = Color32::from_rgb(119, 136, 153);
    pub const SLATE_GRAY: Color32 = Color32::from_rgb(112, 128, 144);
    pub const DARK_SLATE_GRAY: Color32 = Color32::from_rgb(47, 79, 79);
    pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);

    // Progress colours
    pub const LIGHT_CORAL: Color32 = Color32::from_rgb(240, 128, 128);
    pub const LIGHT_SALMON: Color32 = Color32::from_rgb(255, 160, 122);
    pub const KHAKI: Color32 = Color32::from_rgb(240, 230, 140);
    pub const LIGHT_GREEN: Color32 = Color32::from_rgb(144, 238, 144);

    // Card styles
    pub const CARD_BORDER: Stroke = Stroke { width: 1.0, color: Theme::DARK_ORCHID };
    pub const CARD_FILL: Color32 = Theme::LIGHT_ORCHID;
    pub const CARD_MARGIN: Margin = Margin::same(8.0);
    pub const CARD_ROUNDING: Rounding = Rounding::same(10.0);
    pub const CARD_WIDTH: f32 = 200.0;
    pub const CARD_HEIGHT: f32 = 50.0;

    // Button styles
    pub const BUTTON_BORDER: Stroke = Stroke { width: 1.0, color: Theme::DARK_ORCHID };
    pub const BUTTON_FILL: Color32 = Theme::SLATE_BLUE;
    pub const BUTTON_MARGIN: Margin = Margin::same(8.0);
    pub const BUTTON_ROUNDING: Rounding = Rounding::same(5.0);

    // Text
    pub const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT_HIGHLIGHT: Color32 = Color32::from_rgb(255, 215, 0);
}

pub fn get_visuals() -> Visuals {
    Visuals {
        // dark_mode: true,
        override_text_color: Some(Theme::TEXT_COLOR),
        // selection: Theme::TEXT_HIGHLIGHT,
        hyperlink_color: Theme::TEXT_HIGHLIGHT,
        faint_bg_color: Theme::LIGHT_ORCHID,
        extreme_bg_color: Theme::LIGHT_ORCHID,
        code_bg_color: Theme::LIGHT_ORCHID,
        warn_fg_color: Theme::DARK_ORCHID,
        error_fg_color: Theme::DARK_ORCHID,
        window_rounding: Rounding::same(10.0),
        // window_shadow: Shadow::new(0.0, 2.0, 10.0, Theme::BLACK),
        window_fill: Theme::ROYAL_BLUE,
        window_stroke: Stroke::new(1.0, Theme::BLACK),

        panel_fill: Theme::DARK_SLATE_BLUE,

        widgets: Widgets {
            // The style of an interactive widget as you are clicking or dragging it
            active: WidgetVisuals {
                bg_fill: Theme::BUTTON_FILL,
                weak_bg_fill: Theme::BUTTON_FILL,
                bg_stroke: Theme::BUTTON_BORDER,
                rounding: Theme::BUTTON_ROUNDING,
                fg_stroke: Stroke::new(1.0, Theme::TEXT_COLOR),
                expansion: 0.0,
            },
            // The style of an interactive widget, such as a button, at rest
            inactive: WidgetVisuals {
                bg_fill: Theme::BUTTON_FILL,
                weak_bg_fill: Theme::BUTTON_FILL,
                bg_stroke: Theme::BUTTON_BORDER,
                rounding: Theme::BUTTON_ROUNDING,
                fg_stroke: Stroke::new(1.0, Theme::TEXT_COLOR),
                expansion: 0.0,
            },
            // The style of a button that has an open menu beneath it (e.g. a combo-box)
            open: WidgetVisuals {
                bg_fill: Theme::BUTTON_FILL,
                weak_bg_fill: Theme::BUTTON_FILL,
                bg_stroke: Theme::BUTTON_BORDER,
                rounding: Theme::BUTTON_ROUNDING,
                fg_stroke: Stroke::new(1.0, Theme::TEXT_COLOR),
                expansion: 0.0,
            },
            // The style of an interactive widget while you hover it, or when it is highlighted
            hovered: WidgetVisuals {
                bg_fill: Theme::BUTTON_FILL,
                weak_bg_fill: Theme::BUTTON_FILL,
                bg_stroke: Theme::BUTTON_BORDER,
                rounding: Theme::BUTTON_ROUNDING,
                fg_stroke: Stroke::new(1.0, Theme::TEXT_COLOR),
                expansion: 0.0,
            },
            // The style of a widget that you cannot interact with
            noninteractive: WidgetVisuals {
                bg_fill: Theme::BUTTON_FILL, // the background color of windows
                weak_bg_fill: Theme::BUTTON_FILL,
                bg_stroke: Theme::BUTTON_BORDER, // the outline of windows
                rounding: Theme::BUTTON_ROUNDING,
                fg_stroke: Stroke::new(1.0, Theme::TEXT_COLOR), // the normal text color
                expansion: 0.0,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}