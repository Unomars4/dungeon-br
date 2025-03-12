//All available components
pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Player;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Enemy;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MovingRandomly;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChasingPlayer;
