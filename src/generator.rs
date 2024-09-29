
use bevy::{
    prelude::*,
    gltf::GltfMesh
};
use avian3d::{prelude::*, math::PI};
use crate::env::LanternLight;
use crate::{GameState, NotReady};
use crate::transporter::Roll;

pub struct GeneratorPlugin;
impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(Update, loaded.run_if(in_state(GameState::Loading)))
        .add_systems(Update, work.run_if(in_state(GameState::Game)))
        ;
    }
}

// ---

#[derive(Component)]
pub struct Part(usize);

#[derive(Component)]
pub struct GeneratorNR;

#[derive(Resource)]
pub struct GeneratorGLTF(Handle<Gltf>);

#[derive(Component)]
pub struct Rotor;


// ---

fn startup (
    mut cmd: Commands,
    assets: ResMut<AssetServer>,
) {
    cmd.spawn((NotReady, GeneratorNR));
    cmd.insert_resource(GeneratorGLTF(assets.load("models/generator.glb")));
    
}

// ---

fn loaded (
    gltfs: Res<Assets<Gltf>>,
    handle: Res<GeneratorGLTF> ,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut loaded: Local<bool>,
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if *loaded {
        return;
    }

    if let Some(gltf)  = gltfs.get(&handle.0) {
        *loaded = true;
        let mat = materials.add(Color::BLACK);
        for (idx, key) in ["vane", "rotor"].iter().enumerate() {

            let Some(mh) = gltf.named_meshes.get(*key) else {
                continue;
            };
            let Some(g_mesh) = assets_gltfmesh.get(mh) else {
                continue;
            };
            let mesh = g_mesh.primitives[0].mesh.clone();

            let part_sign = if idx == 0 {-1.} else {1.};
            let part_id = cmd.spawn((
                PbrBundle {
                    transform: Transform::from_translation( part_sign * Vec3::Y * 12.5)
                    .with_rotation(Quat::from_rotation_x( -part_sign * PI / 2.)),
                    mesh,
                    visibility: Visibility::Hidden,
                    material: mat.clone(),
                    ..default()
                },
                ColliderConstructor::TrimeshFromMesh,
                Part(idx)
            )).id();   
            if idx == 1 {
                cmd.entity(part_id).insert(Rotor);
            }
        }
    }
}

// ---

fn setup(
    roll_q: Query<(Entity, &Roll)>,
    mut parts_q: Query<(Entity, &mut Visibility, &Part), Without<Roll>>,
    ready_q: Query<Entity, With<GeneratorNR>>,
    mut cmd: Commands
) {
    if roll_q.is_empty()  || parts_q.is_empty() {
        return;
    }

    let Ok(not_ready_e) = ready_q.get_single() else {
        return;
    };

    let mut rolls: Vec<(Entity, &Roll)> = roll_q.iter().collect();

    rolls.sort_by(|a, b| {
        a.1.0.cmp(&b.1.0)
    });

    cmd.entity(rolls[1].0).insert(Rotor);

    for (pe, mut pv, pp) in  parts_q.iter_mut() {
        *pv = Visibility::Visible;
        cmd.entity(rolls[pp.0].0).add_child(pe);
    }
    cmd.entity(not_ready_e).despawn();
}

// ---

fn work(
    r_q: Query<&AngularVelocity, With<Rotor>>,
    mut l_q: Query<&mut SpotLight, With<LanternLight>>
) {
    let intensity =  if let Ok(av) = r_q.get_single() {
        av.length_squared() *  10_000_000.
    } else {0.};

    for mut sl in l_q.iter_mut() {
        sl.intensity = intensity;     
    }
}