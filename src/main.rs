#![allow(clippy::unnecessary_cast)]

use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
mod examples_common_3d;
mod asset_loader;
mod dice;
use crate::{
    examples_common_3d::XpbdExamplePlugin,
    asset_loader::{SceneAssets, AssetLoaderPlugin},
    dice::DicePlugin,
};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            XpbdExamplePlugin,
            AssetLoaderPlugin,
            DicePlugin,
        ))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Gravity(Vec3::NEG_Y * 80.0))
        .insert_resource(CameraSpeed(Vec2::ZERO))
        .run();
}

#[derive(Resource)]
struct CameraSpeed(Vec2);

/// The acceleration used for movement.
#[derive(Component)]
struct MovementAcceleration(Scalar);

fn setup(
    mut commands: Commands,
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
        transform: Transform::from_translation(Vec3::new(0.0, 30.0, 10.0))
            .looking_at(Vec3::Y * 2.0, Vec3::Y),
        ..default()
    });
}

fn move_camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut camera_speed: ResMut<CameraSpeed>,
) {
    let mut transform = query.single_mut();

    if keys.pressed(KeyCode::Right) {
        camera_speed.0.x += 0.1;
    }
    if keys.pressed(KeyCode::Left) {
        camera_speed.0.x -= 0.1;
    }
    if keys.pressed(KeyCode::Up) {
        camera_speed.0.y += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        camera_speed.0.y -= 1.0;
    }

    camera_speed.0 *= 0.95;

    transform.translation.y += camera_speed.0.y * time.delta_seconds();
    transform.translation.y = transform.translation.y.clamp(5.0, 60.0);
    transform.look_at(Vec3::Y * 0.0, Vec3::Y);

    // rotate around the center
    let rotation = Quat::from_rotation_y(time.delta_seconds() * camera_speed.0.x);
    transform.rotate_around(Vec3::ZERO, rotation);
}
