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
    pub fn take_turn(keyboard_input: Res<Input<KeyCode>>) {
        for key in keyboard_input.get_just_pressed() {
            match key {
                KeyCode::N => Player::next_unit(),
                KeyCode::C => Player::command_current(),
                KeyCode::E => Player::end_turn(),
                key => println!("{key:?}"),
            };
        }
    }

    fn spawn_player(mut commands: Commands) {
        commands.spawn((Player, Team::Alliance));
    }

    fn next_unit() {
        todo!()
    }

    fn command_current() {
        todo!()
    }

    fn end_turn() {
        todo!()
    }
}
impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Player::spawn_player)
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
