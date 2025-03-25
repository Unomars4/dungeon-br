use std::collections::HashSet;

//All available components
pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Player {
    pub map_level: i32,
}

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

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Item;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AmuletOfYala;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
