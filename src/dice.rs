use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use crate::asset_loader::SceneAssets;
use rand::Rng;

#[derive(Component, Debug)]
pub struct Dice;

pub struct DicePlugin;

#[derive(Event)]
pub struct DiceRollEvent;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, roll_dices)
            .add_systems(Update, (keyboard_input, roll_dices))
            .add_event::<DiceRollEvent>();
        // .add_systems(Update, roll_dices.in_set(InGameSet::EntityUpdates),);
    }
}

fn roll_dices(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut events: EventReader<DiceRollEvent>,
){
    for _event in events.read() {
        let mut rng = rand::thread_rng();

        let cube_size = 1.0;
        let cube_gap = 2.0;

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
                Collider::cuboid(2.0, 2.0, 2.0),
                Dice,
            ));
        }
    }
}

fn keyboard_input(
    mut ev_dice_roll: EventWriter<DiceRollEvent>,
    keys: Res<Input<KeyCode>>,
){
    if keys.just_pressed(KeyCode::Space) {
        ev_dice_roll.send(DiceRollEvent);
    }
}

