use crate::units::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Turns(u32);
impl Turns {
    pub fn new(turns: u32) -> Self {
        Self { 0: turns }
    }
    pub fn init() -> Self {
        Turns::new(0)
    }
    pub fn increase_turns(mut commands: Commands) {}
}

#[derive(Component)]
pub struct Player;
impl Player {
    pub fn take_turn(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
        // get the player's inputs for this command!
    }
}
impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Soldier::spawn_allied_soldier)
            .add_startup_system(Civilian::spawn_allied_civilian)
            .add_system(Player::take_turn);
    }
}

#[derive(Component)]
pub struct Bot;
impl Bot {
    pub fn take_turn(mut commands: Commands) {
        // do something semi-intelligent
    }
}
impl Plugin for Bot {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Soldier::spawn_axis_soldier)
            .add_startup_system(Civilian::spawn_axis_civilian)
            .add_system(Bot::take_turn);
    }
}
