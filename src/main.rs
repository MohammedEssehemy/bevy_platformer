use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

use bevy_platformer::{
    config::{COLOR_BACKGROUND, WINDOW_HEIGHT, WINDOW_WIDTH},
    AnimationPlugin, CharacterPlugin, FloorPlugin, PlatformsPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugin(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_plugin(PlatformsPlugin)
        .add_plugin(FloorPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(CharacterPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());
}
