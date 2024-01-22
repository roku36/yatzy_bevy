#![allow(clippy::unnecessary_cast)]

use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
mod examples_common_3d;
mod asset_loader;
use crate::{
    examples_common_3d::XpbdExamplePlugin,
    asset_loader::{SceneAssets, AssetLoaderPlugin},
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .add_plugins(AssetLoaderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Gravity(Vec3::NEG_Y * 80.0))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        // .add_systems(Update, apply_limit)
        .run();
}

/// The acceleration used for movement.
#[derive(Component)]
struct MovementAcceleration(Scalar);

// #[derive(Component)]
// struct LimitRadius(Scalar);

fn setup(
    mut commands: Commands,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    scene_assets: Res<SceneAssets>,
) {
    let tray_scene = scene_assets.tray.clone();

    // spawn dice tray
    commands.spawn((
        SceneBundle {
            scene: tray_scene,
            ..default()
        },
        RigidBody::Static,
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh))
    ));

    let cube_size = 1.0;
    let cube_gap = 2.0;

    let mut rng = rand::thread_rng();

    // spawn five cubes
    let cube_height = 15.0;
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
            SceneBundle {
                scene: scene_assets.dice.clone(),
                transform: Transform { translation, rotation, scale },
                ..default()
            },
            RigidBody::Dynamic,
            // Collider::cuboid(cube_size, cube_size, cube_size),
            AsyncSceneCollider::new(Some(ComputedCollider::ConvexHull)),
            MovementAcceleration(10.0),
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
