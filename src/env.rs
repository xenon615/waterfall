use bevy::{
    prelude::*, scene::SceneInstanceReady
};
use avian3d::prelude::*;

use crate::NotReady;
pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_observer(collider_added)
        ;
    }
}

// ---

#[derive(Component, Debug)]
pub struct LanternLight;

#[derive(Component, Debug)]
pub struct TransporterStand;

#[derive(Component, Debug)]
pub struct RiverSourceMarker;

#[derive(Component)]
pub struct  HillNR;

#[derive(Component)]
pub struct  LanternNR;

// ---

fn startup (
    mut cmd: Commands,
    assets: ResMut<AssetServer>,
    mut al: ResMut<AmbientLight> 
) {
    al.brightness = 40.;
    cmd.spawn((HillNR, NotReady));
    cmd.spawn((LanternNR, NotReady));
    cmd.spawn((
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("models/scene.glb"))),
        ColliderConstructorHierarchy::new(None)
        .with_constructor_for_name("hill", ColliderConstructor::TrimeshFromMesh)
        ,
        RigidBody::Static,
        
    ))
    .observe(setup)
    ;

    cmd.spawn((
        DirectionalLight {
            illuminance: 200.,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-90_f32.to_radians()))
    ));
}

// ---

fn collider_added(
    trg: Trigger<OnAdd, Collider>,
    q: Query<&Name>,
    mut  cmd: Commands,
    ready_q: Single<Entity, With<HillNR>>
) {
    if let Ok(name) = q.get(trg.entity()) {
        if name.contains("hill") {
            cmd.entity(ready_q.into_inner()).despawn();
        }
    }
}

// ---

fn setup(
    tr: Trigger<SceneInstanceReady>,
    l_q : Query<(&Parent, &Transform, &GltfExtras)>,
    children: Query<&Children>,
    mut cmd: Commands,
    ready_q: Single<Entity, With<LanternNR>>
) {
    for c in children.iter_descendants_depth_first(tr.entity()) {
        let Ok((p, t, se)) = l_q.get(c) else {
            continue;
        };
        if se.value.contains("LanternLight") {
            let mut trans = t.with_rotation(Quat::from_rotation_x(-90_f32.to_radians()));
            trans.translation.y = -0.6;
            let plb = cmd.spawn((
                SpotLight {
                    color: Color::srgb(1., 0.64, 0.),
                    intensity: 0.,
                    outer_angle: 2.8,
                    inner_angle: 4.5,
                    shadows_enabled: false,
                    ..default()
                },
                trans,
                LanternLight
            )).id();
            cmd.entity(**p).add_child(plb);
        } else if se.value.contains("TransporterStand") {
            cmd.entity(c).insert(TransporterStand);
        } else if se.value.contains("RiverSource") {
            cmd.entity(c).insert(RiverSourceMarker);
        }
    }
    cmd.entity(ready_q.into_inner()).despawn();

}
