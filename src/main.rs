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
        .run();
}

/// The acceleration used for movement.
#[derive(Component)]
struct MovementAcceleration(Scalar);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Ground
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            transform: Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    let cube_size = 2.0;
    let cube_gap = 2.0;

    let mut rng = rand::thread_rng();

    // Spawn cube stacks
    for x in -2..2 {
        for z in -2..2 {
            let translation= Vec3::new(x as f32, 5.0, z as f32) * (cube_size + cube_gap);
            let axis = Vector3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
            let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
            let rotation = Quaternion::from_axis_angle(axis, angle as f32);
            let scale = Vec3::splat(cube_size as f32);
            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::rgb(0.2, 0.7, 0.9).into()),
                    // transform: Transform::from_translation(position)
                    //     .with_scale(Vec3::splat(cube_size as f32)),
                    // ..default()

                    // instead of only translation, apply random rotation.
                    transform: Transform { translation, rotation, scale },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
                MovementAcceleration(10.0),
            ));
        }
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
