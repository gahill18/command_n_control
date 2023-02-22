use bevy::prelude::*;

// For moveable/controlable units

#[derive(Component, Debug)]
pub struct Name(String);
impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            0: String::from(name),
        }
    }
}
#[derive(Component)]
pub struct Health(u32);
#[derive(Component)]
pub struct Strength(u32);

#[derive(Component)]
pub enum Team {
    Alliance,
    Axis,
}

#[derive(Component)]
pub struct Soldier;

#[derive(Component)]
pub struct Civilian;
