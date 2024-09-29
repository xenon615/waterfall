use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::{env::TransporterStand,  GameState, ShowGismos};

pub struct  TransporterPlugin;
impl Plugin for TransporterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn)
        .add_systems(OnEnter(GameState::Game), setup)
        ;
    }
}

// ---

#[derive(Component)]
pub struct Track;

#[derive(Component, Debug)]
pub struct Roll(pub u8);

#[derive(Component)]
pub struct Transporter;

#[derive(Component)]
pub struct Bearing;

const TRANS_LEN: f32 = 15.0;
const ROLLS_COUNT: u8 = 2;
const ROLL_WIDTH: f32 = 10.0;

// ---

fn spawn(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    cmd.spawn((
        TransformBundle::from_transform(
            Transform::from_xyz(0., 10., 0.)
            // .with_rotation(Quat::from_rotation_y(-PI / 4.))
        )
        
        ,
        VisibilityBundle::default(),
        Transporter,
        Name::new("Transporter")
    )).with_children(|parent| {
        let mut roll_pos_z = -TRANS_LEN / 2. ;
        // ---
        let track_y = 0.2;
        let track_x = 9.0;
    
        let z_coef = 1.5;
        let track_line_count = 10;
        let track_arc_count = 6;
        let step_z = TRANS_LEN / (track_line_count - 1) as f32;
   
        let roll_radius =  track_arc_count as f32  * step_z / PI;
        let mut track_pos = Vec3::new(0., roll_radius + track_y, roll_pos_z);
       
        let track_z = step_z /z_coef;
       
        let track_step = Vec3::new(0., 0., step_z);
        let track_count = (track_arc_count + track_line_count) *  2;
        let angle_step =  PI / track_arc_count  as f32;
        let mut next_angle = 0.0;
    
        let track_mesh = meshes.add(Cuboid::from_size(Vec3::new(track_x, track_y, track_z)));
        let mut tracks: Vec<Entity> = Vec::new();
        let track_mat = materials.add(Color::BLACK);
    
        for  i in 0..track_count {
            if ((track_count / 2) - track_arc_count ..track_count / 2 ).contains(&i) || (track_count - track_arc_count..track_count).contains(&i) {
                next_angle += angle_step;    
            }
    
            let track_id = parent.spawn((
                PbrBundle {
                    transform: Transform::from_translation(track_pos).with_rotation(Quat::from_rotation_x(next_angle)),
                    mesh: track_mesh.clone(),
                    material: track_mat.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                GravityScale(0.5),
                Collider::cuboid(track_x, track_y, track_z),
                Restitution::new(0.1).with_combine_rule(CoefficientCombine::Min),
                Friction::new(1.0).with_combine_rule(CoefficientCombine::Multiply),
                ColliderDensity(0.2),
                Track
            )).id();
            tracks.push(track_id);   
            track_pos += if next_angle != 0. { Quat::from_rotation_x(next_angle).mul_vec3(track_step) } else { track_step };
        }
    
        for i in 0 .. track_count  {
            let anchor =  Vec3::new(0., 0., track_z * 0.7);
            parent.spawn(
                RevoluteJoint::new(tracks[i], tracks[(i + 1) % track_count])
                .with_local_anchor_1(anchor)
                .with_local_anchor_2(-anchor)
                .with_aligned_axis(Vec3::X)
                .with_angle_limits(-1., 1.)
                .with_compliance(0.0001)
                .with_angular_velocity_damping(0.5)
                .with_linear_velocity_damping(5.)
            );
        }
    
        // ---
    
        let roll_step = TRANS_LEN / (ROLLS_COUNT - 1) as f32;
      
        let roll_mesh = meshes.add(Cylinder::new(roll_radius, ROLL_WIDTH));
        let roll_mat = materials.add(Color::WHITE);
    
        for idx in 0 .. ROLLS_COUNT {
            let roll_id = parent.spawn((
                MaterialMeshBundle {
                    transform: Transform::from_xyz(0., 0., roll_pos_z).with_rotation(Quat::from_rotation_z(PI / 2.)),
                    mesh: roll_mesh.clone(),
                    material: roll_mat.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cylinder(roll_radius, ROLL_WIDTH),
                Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
                Restitution::new(0.1),
                Roll(idx),
                ShowGismos
            ))

            .with_children(|r| {
                let disk_mesh = meshes.add(Cylinder::new(roll_radius * 1.2 , 0.2));
                for i in 0..2 {
                    let disk_sign = if i == 0 {-1.} else {1.};
                    r.spawn((
                        MaterialMeshBundle {
                            transform: Transform::from_xyz(0., disk_sign * ROLL_WIDTH / 2. + 0.1, 0.),
                            mesh: disk_mesh.clone(),
                            material: roll_mat.clone(),
                            ..default()
                        },
                        Collider::cylinder(roll_radius * 1.2, 0.2),
                        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
                        Restitution::new(1.),
                        AngularDamping(0.5)
                    ));
                }
            }) 
            .id()
            ;

            let bearing_side = if idx == 0 {-1.} else {1.};
            let bearing_offset = 8.0;
            let bearing_id = parent.spawn((
                MaterialMeshBundle {
                    transform: Transform::from_xyz(bearing_side * bearing_offset , 0.0, roll_pos_z)
                    .with_rotation(Quat::from_rotation_z(PI / 2.))
                    ,
                    material: track_mat.clone(),
                    mesh: meshes.add(Cuboid::from_length(0.5)),
                    ..default()
                },
                RigidBody::Static,
                GravityScale(0.),
                Dominance(1),
                Collider::cuboid(0.5, 0.5, 0.5),
                Bearing,
                ShowGismos
            )).id();
    
            parent.spawn(
                RevoluteJoint::new(roll_id, bearing_id)
                .with_aligned_axis(Vec3::Y)
                .with_local_anchor_1(-bearing_side * Vec3::Y * bearing_offset)
                .with_angular_velocity_damping(0.)
            );

            roll_pos_z += roll_step;
        }
    });

}

// ---

fn setup(
    mut transporter_q: Query<&mut Transform, With<Transporter>>,
    stand_q: Query<&Transform, (With<TransporterStand>, Without<Transporter>)>,
) {
    if let Ok(mut t) = transporter_q.get_single_mut() {
        if let Ok(s) = stand_q.get_single() {
            t.translation = s.translation;   
        }
    }
}
