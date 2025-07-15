use eframe::egui::{ColorImage, Context, TextureHandle};
use std::collections::HashMap;

// Embed all skill icons as byte slices at compile time
macro_rules! embed_icons {
    ($($name:ident),* $(,)?) => {
        $(
            #[allow(non_upper_case_globals)]
            pub static $name: &[u8] = include_bytes!(concat!("../../assets/skills/", stringify!($name), ".png"));
        )*
    };
}

embed_icons!(
    attack, defence, strength, hitpoints, ranged, prayer, magic,
    cooking, woodcutting, fletching, fishing, firemaking, crafting,
    smithing, mining, herblore, agility, thieving, slayer, farming,
    runecraft, hunter, construction, overall
);

// Load a single icon texture from embedded bytes
fn load_texture(ctx: &Context, name: &str, bytes: &[u8]) -> TextureHandle {
    let image = image::load_from_memory(bytes).expect("Failed to load embedded image").to_rgba8();
    let resized = image::imageops::resize(&image, 24, 24, image::imageops::FilterType::Lanczos3);
    let size = [resized.width() as usize, resized.height() as usize];
    let pixels = resized.into_vec();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    ctx.load_texture(name.to_owned(), color_image, Default::default())
}

// Load all embedded icons into a HashMap<String, TextureHandle>
pub fn load_all_icons(ctx: &Context) -> HashMap<String, TextureHandle> {
    let mut map = HashMap::new();

    macro_rules! load {
        ($($name:ident),* $(,)?) => {
            $(
                map.insert(stringify!($name).to_string(), load_texture(ctx, stringify!($name), $name));
            )*
        };
    }

    load!(
        attack, defence, strength, hitpoints, ranged, prayer, magic,
        cooking, woodcutting, fletching, fishing, firemaking, crafting,
        smithing, mining, herblore, agility, thieving, slayer, farming,
        runecraft, hunter, construction, overall
    );

    map
}
