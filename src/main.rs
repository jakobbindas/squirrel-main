mod credits;

use bevy::{prelude::*, window::PresentMode};
// use crate::credits::CreditPlugin;

const TITLE: &str = "squirrel";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(CreditPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement) // Register player movement system
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Define Player Struct
#[derive(Component)]
struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((SpriteBundle { 
            texture: asset_server.load("sprites/lil_man_placeholder.png"), 
            ..default()
        },
        Player {}
    )); 
}

fn player_movement(
    key_in: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    // Check to see if player exists
    if let Ok(mut transform) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if key_in.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if key_in.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        // Translate the movement by multiplying the direction by the speed by the time (this gives a distance vector that translates to movement)
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}