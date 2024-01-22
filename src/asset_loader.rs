use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub dice0: Handle<Scene>,
    pub dice1: Handle<Scene>,
    pub tray: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        dice0: asset_server.load("Dice.glb#Scene0"),
        dice1: asset_server.load("Dice.glb#Scene1"),
        tray: asset_server.load("Tray.glb#Scene0"),
    }
}
