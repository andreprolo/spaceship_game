pub mod asset_loader;
pub mod asteroids;
pub mod camera;
pub mod collision_detection;
pub mod debug;
pub mod despawn;
pub mod movement;
pub mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy built-in.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.35)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User configured plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
