use bevy::prelude::*;
use bevy::core_pipeline::Skybox;
use bevy::render::camera::{Exposure, PhysicalCameraParameters};

// ---

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn) 
        ;
    }
} 

// ---

#[derive(Component)]
pub struct Cam;

// ---

fn spawn (
    mut commands : Commands,
    assets: ResMut<AssetServer>
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-100., 20., 35.).with_rotation(Quat::from_rotation_y(20_f32.to_radians())),
        // .looking_to(-Vec3::Z, Vec3::Y),
        Camera {
            hdr: true,
            ..default()
        },
        Exposure::from_physical_camera(PhysicalCameraParameters {
            sensitivity_iso: 80.,
            ..default()
        }),
        Skybox {
            image: assets.load("skyboxes/space_green.ktx2"),
            brightness: 300.,
            ..default()
        },
        Cam,
    ));
}

// ---
