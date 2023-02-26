use crate::players::{Bot, Player, Turns, *};
use crate::units::*;
use bevy::prelude::*;

pub mod players;
pub mod units;

const MISSING_TEXTURE: Color = Color::rgb(0.9, 0.0, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const CELL_SIZE: Vec3 = Vec3::new(90.0, 90.0, 0.0);
const UNIT_SIZE: Vec3 = Vec3::new(60.0, 60.0, 0.0);
const FRIENDLY_COLOR: Color = Color::rgb(0.3, 0.7, 0.3);
const ENEMY_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(Turns::init())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(Player)
        .add_plugin(Bot)
        .add_system(Turns::increase_turns)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Map

    // Units
    let (start_x, start_y, start_z) = (0.0, 10.0, 0.0);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(start_x, start_y, start_z),
                scale: UNIT_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: FRIENDLY_COLOR,
                ..default()
            },
            ..default()
        },
        Soldier,
    ));
}
