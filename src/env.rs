use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{GameState, NotReady};
pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<TransporterStand>()
        .register_type::<Lantern>()
        .register_type::<LanternLight>()
        .add_systems(Startup, startup)
        .add_systems(Update, set_lanterns.run_if(in_state(GameState::Loading)))
        .observe(collider_added)
        ;
    }
}

// ---

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct TransporterStand;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Lantern;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LanternLight;


#[derive(Component)]
pub struct  HillNR;

#[derive(Component)]
pub struct  LanternNR;


// ---


fn startup (
    mut cmd: Commands,
    assets: ResMut<AssetServer>
) {
    cmd.spawn((HillNR, NotReady));
    cmd.spawn((LanternNR, NotReady));
    cmd.spawn((
        SceneBundle {
            scene: assets.load(GltfAssetLabel::Scene(0).from_asset("models/scene.glb")),
            ..default()
        },
        ColliderConstructorHierarchy::new(None)
        .with_constructor_for_name("hill", ColliderConstructor::TrimeshFromMesh)
        ,
        RigidBody::Static,
    ));

    cmd.spawn(
        DirectionalLightBundle{
            directional_light: DirectionalLight {
                illuminance: 100.,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(-90_f32.to_radians())),
            ..default()
        }
    );

}

// ---

fn collider_added(
    trg: Trigger<OnAdd, Collider>,
    q: Query<&Name>,
    mut  cmd: Commands,
    ready_q: Query<Entity, With<HillNR>>
) {
    if let Ok(name) = q.get(trg.entity()) {
        if name.contains("hill") {
            if let Ok(e) = ready_q.get_single() {
                // println!("hill ready");
                cmd.entity(e).despawn();
            } 
        }
    }
}

// ---

fn set_lanterns(
    l_q : Query<(Entity, &Parent, &Transform), (With<LanternLight>, Without<SpotLight>)>,
    mut commands: Commands,
    ready_q : Query<Entity, (With<LanternNR>, With<NotReady>)>,
    mut spawned: Local<bool>
) {

    if l_q.is_empty() {
        if *spawned {
            if let Ok(re) = ready_q.get_single() {
                commands.entity(re).despawn();
                // println!("lantern ready");
            }
        }
    } else {
        *spawned = true;
    }

    for (e, p, t) in l_q.iter() {
        let mut trans = t.with_rotation(Quat::from_rotation_x(-90_f32.to_radians()));
        trans.translation.y = -0.6;

        let plb = commands.spawn((
            SpotLightBundle {
                spot_light: SpotLight {
                    color: Color::srgb(1., 0.64, 0.),
                    intensity: 5_000_000.,
                    outer_angle: 2.8,
                    inner_angle: 4.5,
                    shadows_enabled: false,
                    ..default()
                },
                // visibility: Visibility::Hidden,
                transform: trans,  
                ..default()
            },
            LanternLight
        )).id();

        commands.entity(**p).add_child(plb);
        commands.entity(e).despawn_recursive();

    }
}
