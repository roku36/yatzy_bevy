use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use crate::asset_loader::SceneAssets;
use rand::Rng;

#[derive(Component)]
pub struct Dice {
    static_timer: u32,
    number: u32,
    locked: bool,
}

// dice face array
#[derive(Resource)]
pub struct DiceResult(pub [u32; 5]);

pub struct DicePlugin;

#[derive(Event)]
pub struct DiceRollEvent;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiceResult([0; 5]))
            .add_systems(Update, (keyboard_input, roll_dice, respawn_dice, lock_dice))
            .add_event::<DiceRollEvent>();
        // .add_systems(Update, roll_dices.in_set(InGameSet::EntityUpdates),);
    }
}

fn roll_dice(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut events: EventReader<DiceRollEvent>,
    dice: Query<Entity, With<Dice>>,
){
    for _event in events.read() {
        for entity in dice.iter() {
            // commands.entity(entity).despawn();
            commands.entity(entity).despawn_recursive();
        }
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
                    scene: scene_assets.dice0.clone(),
                    transform: Transform { translation, rotation, scale },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(2.0, 2.0, 2.0),
                Dice {
                    static_timer: 0,
                    number: 0,
                    locked: false,
                }
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

fn respawn_dice(
    mut dice: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity), With<Dice>>,
){
    for (mut transform, mut linear_velocity, mut angular_velocity) in dice.iter_mut() {
        if transform.translation.length() > 100.0 {
            transform.translation = Vec3::Y * 30.0;
            *linear_velocity = LinearVelocity(Vec3::ZERO);
            *angular_velocity = AngularVelocity(Vec3::ZERO);
        }
    }
}

fn lock_dice(
    mut dice: Query<(
    // &mut Handle<Scene>, 
    &mut Dice, 
    &mut RigidBody, 
    &Transform, 
    &LinearVelocity, 
    &AngularVelocity
), With<Dice>>,
    // dice_result: Res<DiceResult>,
    // scene_assets: Res<SceneAssets>,
){
    for (
        // mut handle_scene,
        mut dice,
        mut rigid_body,
        transform,
        linear_velocity,
        angular_velocity
    ) in dice.iter_mut() {
            if linear_velocity.0.length() < 0.1 && angular_velocity.0.length() < 0.1 && !dice.locked {
                // add time to dice
                dice.static_timer += 1;
            }
            if dice.static_timer > 20 {
                // *handle_scene = scene_assets.dice1.clone();
                *rigid_body = RigidBody::Static;
                dice.number = determine_dice_face(transform.rotation);
                info!("dice number: {}", dice.number);
                dice.static_timer = 0;
                dice.locked = true;
            }
        }
}

const CUBE_SIDES: [Vec3; 6] = [
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
];

fn determine_dice_face(quat: Quat) -> u32 {
    // let rotated_up = quat.mul_quat(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
    let mut max_height= -1.0;
    let mut best_match = 6;

    for (i, side) in CUBE_SIDES.iter().enumerate() {
        // let dot = rotated_up.dot(*side);
        let y = quat.mul_vec3(*side)[1];
        if y > max_height {
            max_height = y;
            best_match = i;
        }
    }

    best_match as u32 + 1
}
