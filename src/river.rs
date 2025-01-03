use avian3d::prelude::{Collider, ColliderDensity, Dominance,  LinearVelocity, Restitution, RigidBody};
use bevy::{
    prelude::*,
    pbr:: {NotShadowCaster, NotShadowReceiver}
};
use crate::{env::RiverSourceMarker, shared::{fibonacci_sphere, random_pos}, GameState, NotReady};

pub struct  RiverPlugin;
impl Plugin for RiverPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RiverSource>()
        .add_systems(Startup, startup)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(OnEnter(GameState::Game), enter_game)
        .add_systems(Update, recycle.run_if(in_state(GameState::Game)))

        ;
    }
}

// ---

#[derive(Component)]
pub struct Drop;

const DROP_RADIUS: f32 = 0.9;
const DROPS_COUNT: usize = 512;

#[derive(Resource, Default)]
pub struct RiverSource(Vec3);

#[derive(Component)]
pub struct RiverNR;

// ---

fn startup(
    mut cmd: Commands,
) {
    cmd.spawn((NotReady, RiverNR));
}

// ---

fn setup(
    mut rs: ResMut<RiverSource>,
    marker_q: Single<(Entity, &Transform), With<RiverSourceMarker>>,
    mut cmd: Commands,
    ready_q: Single<Entity, With<RiverNR>>
) {
    let (e, mt) = marker_q.into_inner();
    rs.0 = mt.translation;
    cmd.entity(e).despawn_recursive();
    cmd.entity(ready_q.into_inner()).despawn();
}

// ---

fn enter_game(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    rs: Res<RiverSource>,
    mut cmd: Commands
) {

    let mesh = meshes.add(Sphere::new(DROP_RADIUS));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0., 0., 1.),
        ..default()
    });

    for p in fibonacci_sphere(DROPS_COUNT)  {
        cmd.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(p * 20. + rs.0.with_y(200.)),
            RigidBody::Dynamic,
            Collider::sphere(DROP_RADIUS),
            ColliderDensity(100.0),
            Restitution::new(0.1).with_combine_rule(avian3d::prelude::CoefficientCombine::Min),
            Dominance(1),
            Drop,
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }

}

// ---

fn recycle (
    mut ball_q: Query<(&mut Transform, &mut LinearVelocity), With<Drop>>,
    rs: Res<RiverSource> 
) {
    for (mut t, mut v) in ball_q.iter_mut() {
        
        if t.translation.y < -5. {
            t.translation = random_pos(rs.0, 10.);
            t.look_to(-Vec3::Y, Vec3::Y);
            v.0 = Vec3::ZERO;
        }
    }
}