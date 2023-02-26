use bevy::prelude::*;

#[derive(Component)]
pub struct Name(String);
#[derive(Component)]
pub struct Health(u32);
#[derive(Component)]
pub struct Strength(u32);
#[derive(Component)]
pub struct Range(u32);
#[derive(Component)]
pub struct Sight(u32);
#[derive(Component)]
pub struct Moves(u32);
#[derive(Component)]
pub enum Team {
    Alliance,
    Axis,
}
#[derive(Component)]
pub struct Position(Vec2);

impl Name {
    fn new(name: &str) -> Self {
        Name {
            0: String::from(name),
        }
    }
}
impl Health {
    fn new(hp: u32) -> Self {
        Health { 0: hp }
    }
}
impl Strength {
    fn new(str: u32) -> Self {
        Strength { 0: str }
    }
}
impl Range {
    fn new(rng: u32) -> Self {
        Range { 0: rng }
    }
}
impl Sight {
    fn new(sight: u32) -> Self {
        Sight { 0: sight }
    }
}
impl Moves {
    fn new(mvs: u32) -> Self {
        Moves { 0: mvs }
    }
}
impl Position {
    fn new(x: u32, y: u32) -> Self {
        Position {
            0: Vec2::new(x as f32, y as f32),
        }
    }
}

#[derive(Component)]
pub struct Soldier;
#[derive(Component)]
pub struct Civilian;

impl Soldier {
    pub fn spawn_allied_soldier(mut commands: Commands) {
        commands.spawn((
            Soldier,
            Name::new("Warrior"),
            Health::new(100),
            Strength::new(8),
            Range::new(2),
            Sight::new(2),
            Moves::new(2),
            Team::Alliance,
            Position::new(0, 0),
        ));
    }
    pub fn spawn_axis_soldier(mut commands: Commands) {
        commands.spawn((
            Soldier,
            Name::new("Warrior"),
            Health::new(100),
            Strength::new(8),
            Range::new(2),
            Sight::new(2),
            Moves::new(2),
            Team::Axis,
            Position::new(0, 0),
        ));
    }
}
impl Civilian {
    pub fn spawn_allied_civilian(mut commands: Commands) {
        commands.spawn((
            Soldier,
            Name::new("Worker"),
            Health::new(10),
            Range::new(2),
            Sight::new(2),
            Team::Alliance,
            Position::new(0, 0),
        ));
    }
    pub fn spawn_axis_civilian(mut commands: Commands) {
        commands.spawn((
            Soldier,
            Name::new("Worker"),
            Health::new(10),
            Range::new(2),
            Sight::new(2),
            Team::Alliance,
            Position::new(0, 0),
        ));
    }
}
