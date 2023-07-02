use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::{FLOOR_THICKNESS, WINDOW_BOTTOM_Y, WINDOW_WIDTH};

const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

#[derive(Bundle)]
struct FloorBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl FloorBundle {
    fn new() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_FLOOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                    scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(FloorBundle::new());
}
