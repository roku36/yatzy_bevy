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
        .run();
}

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

