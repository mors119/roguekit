use bevy::prelude::Color;

#[derive(Clone, Copy)]
pub struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4],
    pub(crate) background: [f32; 4],
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        let fg = Color::WHITE.to_srgba();
        let bg = Color::BLACK.to_srgba();
        Self {
            glyph: 32,
            foreground: [fg.red, fg.green, fg.blue, fg.alpha],
            background: [bg.red, bg.green, bg.blue, bg.alpha],
        }
    }
}
