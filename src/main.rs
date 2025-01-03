#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use avian3d::{
    // prelude::PhysicsDebugPlugin, 
    PhysicsPlugins
};

use bevy::{
    prelude::*,
    window::{
        WindowResolution, 
        // WindowMode
    }
};

mod shared;
mod camera;
mod river;
mod transporter;
mod env;
mod generator;

// ---

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Game
}

#[derive(Component)]
pub struct NotReady;

#[derive(Component)]
pub struct ShowGismos;

// ---

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window : Some(Window {
                    canvas: Some("#game-canvas".into()),
                    resolution : WindowResolution::new(1400., 900.),
                    // mode: WindowMode::BorderlessFullscreen,
                    position: WindowPosition::Centered(MonitorSelection::Primary),

                    ..default()
                }),
                ..default()
            },

        ),
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
        river::RiverPlugin,
        camera::CameraPlugin,
        transporter::TransporterPlugin,
        env::EnvPlugin,
        generator::GeneratorPlugin,
    ))
    .init_state::<GameState>()
    .add_systems(Update, check_ready.run_if(in_state(GameState::Loading)))
    // .add_systems(Update, show_gizmos)
    .run()
    
    ;
}

// ---

fn check_ready(
    not_ready_q: Query<&NotReady>,
    mut next: ResMut<NextState<GameState>>     

) {
    if not_ready_q.is_empty() {
        println!("GAME!");
        next.set(GameState::Game);
    }
}

// ---

#[allow(dead_code)]
fn show_gizmos ( 
    mut gismos: Gizmos,
    t_q: Query<&GlobalTransform, With<ShowGismos>>
) {
    for t in t_q.iter()   {
        gismos.axes(*t, 10.);
    }
}