use bevy::prelude::*;

use crate::{camera_target::{CameraTarget, SetCameraTarget},  GameState};
pub struct ScenarioPlugin;
impl Plugin for  ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CamTargets>()
        .add_systems(OnEnter(GameState::Game), start)
        .add_systems(Update, switch.run_if(in_state(GameState::Game)))
        ;
    }
}

// ---

#[derive(Resource)]
pub struct CamTargets(Vec<CameraTarget>);

impl FromWorld for CamTargets {
    fn from_world(_world: &mut World) -> Self {
        CamTargets(
            vec![
                CameraTarget::from_position(Vec3::new(-157., 0., 31.))
                .with_translation_bias(Vec3::new(30., 20., -0.))
                .with_direction(-Dir3::Z).with_velocity(0.7),

                CameraTarget::from_position(Vec3::new(-60., 80., -60.0))
                .with_direction(Dir3::new(Vec3::new(-1., -0.8, 0.)).unwrap())
                .with_velocity(0.8),

                CameraTarget::from_position(Vec3::new(-60., 100., -150.0))
                .with_direction(Dir3::new(Vec3::new(-1., -0.8, 0.)).unwrap())
                .with_velocity(0.8),


                CameraTarget::from_position(Vec3::new(-220., 80., -60.0))
                .with_direction(Dir3::new(Vec3::new(1., -0.8, 0.)).unwrap())
                .with_velocity(0.8),

            ]
        )
    }
}



// ---

fn start(
    mut cmd: Commands,
    targets: Res<CamTargets>,
) {
    cmd.trigger(
        SetCameraTarget(targets.0[0], 0)
    );
}

// ---

fn switch(
    mut cmd: Commands,
    targets: Res<CamTargets>,
    keys: Res<ButtonInput<KeyCode>>,
    mut idx: Local<usize>
) {
    if keys.just_pressed(KeyCode::KeyN) {
        *idx = (*idx + 1) % targets.0.len();
        cmd.trigger(SetCameraTarget( targets.0[*idx], 0));
    }

}