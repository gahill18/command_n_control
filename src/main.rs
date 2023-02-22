use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Player)
        .run();
}

// For units

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

// For players

#[derive(Component)]
pub struct Soldier;
#[derive(Component)]
pub struct Civilian;
#[derive(Component)]
pub struct Player;

// For the engine

#[derive(Resource)]
pub struct Turns(u32);

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

fn spawn_soldier(mut commands: Commands) {
    commands.spawn((
        Soldier,
        Name::new("Warrior"),
        Health::new(100),
        Strength::new(8),
        Range::new(2),
        Sight::new(2),
        Moves::new(2),
        Team::Alliance,
    ));
}
fn spawn_civilian(mut commands: Commands) {
    commands.spawn((
        Soldier,
        Name::new("Worker"),
        Health::new(10),
        Range::new(2),
        Sight::new(2),
        Team::Alliance,
    ));
}

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_soldier)
            .add_startup_system(spawn_civilian);
    }
}
