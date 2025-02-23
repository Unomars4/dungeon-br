//All available components
pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Player;
