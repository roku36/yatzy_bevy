#![allow(clippy::unnecessary_cast)]

use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
mod examples_common_3d;
use crate::examples_common_3d::XpbdExamplePlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Gravity(Vec3::NEG_Y * 80.0))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, apply_limit)
        .run();
}

/// The acceleration used for movement.
#[derive(Component)]
struct MovementAcceleration(Scalar);

#[derive(Component)]
struct LimitRadius(Scalar);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cylinder_mesh = meshes.add(Mesh::from(shape::Cylinder { radius: 10.0, height: 1.0, resolution: 256, segments: 1 }));

    // cylinder for dice dish
    commands.spawn((
        PbrBundle {
            mesh: cylinder_mesh.clone(),
            material: materials.add(Color::rgb(0.0, 0.2, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cylinder(1.0, 10.0),
    ));
    // commands.spawn((
    //     PbrBundle {
    //         mesh: circle_mesh.clone(),
    //         material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
    //         transform: Transform::from_xyz(0.0, 1.0, 0.0),
    //         ..default()
    //     },
    //     RigidBody::Dynamic,
    //     Collider::cylinder(0.5, 1.0),
    // ));


    let cube_size = 2.0;
    let cube_gap = 0.5;

    let mut rng = rand::thread_rng();

    // spawn five cubes
    let cube_height = 15.0;
    // let cube_pos: [Vec3; 5] = [
    //     Vec3::new(0.0, 5.0, 0.0),
    //     Vec3::new(1.0, 5.0, 1.0),
    //     Vec3::new(1.0, 5.0, -1.0),
    //     Vec3::new(-1.0, 5.0, 1.0),
    //     Vec3::new(-1.0, 5.0, -1.0),
    // ];
    let cube_pos = [
        Vec3::new(0.0, cube_height, 0.0),
        Vec3::new(1.0, cube_height, 1.0),
        Vec3::new(1.0, cube_height, -1.0),
        Vec3::new(-1.0, cube_height, 1.0),
        Vec3::new(-1.0, cube_height, -1.0),
    ];
    for pos in cube_pos {
        let translation= pos * (cube_size + cube_gap);
        let axis = Vector3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
        let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
        let rotation = Quaternion::from_axis_angle(axis, angle as f32);
        let scale = Vec3::splat(cube_size as f32);
        commands.spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
                transform: Transform { translation, rotation, scale },
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            MovementAcceleration(10.0),
            LimitRadius(7.0),
        ));
    }

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 60.0, 20.0))
            .looking_at(Vec3::Y * 2.0, Vec3::Y),
        ..default()
    });
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&MovementAcceleration, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for (movement_acceleration, mut linear_velocity) in &mut query {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let horizontal = right as i8 - left as i8;
        let vertical = down as i8 - up as i8;
        let direction =
            Vector::new(horizontal as Scalar, 0.0, vertical as Scalar).normalize_or_zero();

        // Move in input direction
        if direction != Vector::ZERO {
            linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
            linear_velocity.z += direction.z * movement_acceleration.0 * delta_time;
        }
    }
}

fn apply_limit(mut query: Query<(&LimitRadius, &Position, &mut LinearVelocity)>) {
    for (limit_radius, position, mut linear_velocity) in &mut query {
        // if euclidean_distance(position.xy, Vec3::ZERO) > limit_radius.0 {
        if position.xz().length() > limit_radius.0 {
            let normalized = position.xz().normalize();
            let dot_velocity = normalized.dot(linear_velocity.xz());
            if dot_velocity > 0.0 {
                linear_velocity.x -= dot_velocity * normalized.x;
                linear_velocity.z -= dot_velocity * normalized.y;
            }
        }
    }
}
