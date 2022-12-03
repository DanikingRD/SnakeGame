mod components;
mod constants;
mod direction;

use constants::{ARENA_WIDTH, ARENA_HEIGHT, BACKGROUND_COLOR, SNAKE_SEGMENT_COLOR, SNAKE_HEAD_COLOR, FOOD_COLOR};
use components::{Position, Size, SnakeHead, Food};
use direction::Direction;

use bevy::{
    prelude::{
        App, Camera2dBundle, ClearColor, Commands, CoreStage, Entity, Input, KeyCode,
        Plugin, PluginGroup, Query, Res, SystemSet, Transform, Vec2, Vec3, With, IntoSystemDescriptor,
    },
    sprite::{Sprite, SpriteBundle},
    time::FixedTimestep,
    window::{WindowDescriptor, WindowPlugin, Windows},
    DefaultPlugins,
};

use rand::random;

fn main() {
    let window_plugin = WindowPlugin {
        window: WindowDescriptor {
            title: "SnakeGame by Daniking".to_string(),
            width: 500.,
            height: 500.,
            ..Default::default()
        },
        ..Default::default()
    };
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugin(SnakeGamePlugin)
        .run();
}

struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(setup_camera)
            .add_startup_system(add_snake_head)
            .add_system(update_snake_dir.before(update_snake))
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(setup_translation)
                    .with_system(setup_scaling),
            )
            .add_system_set(
                SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(update_snake),
            ).add_system_set(
                SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn add_snake_head(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead {
            dir: Direction::Right,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn setup_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            size.width / ARENA_WIDTH as f32 * window.width() as f32,
            size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        ))
    }
}
fn setup_translation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            1.0,
        );
    }
}

fn update_snake_dir(mut query: Query<&mut SnakeHead, With<SnakeHead>>, keyboard: Res<Input<KeyCode>>) {
    for mut head in query.iter_mut() {
        if keyboard.pressed(KeyCode::Left) {
            head.dir = Direction::Left;
        }
        if keyboard.pressed(KeyCode::Up) {
            head.dir = Direction::Up;
        }
        if keyboard.pressed(KeyCode::Right) {
            head.dir = Direction::Right;
        }
        if keyboard.pressed(KeyCode::Down) {
            head.dir = Direction::Down;
        }
    }
}
fn update_snake(mut query: Query<(&mut Position, &mut SnakeHead), With<SnakeHead>>) {
    for (mut pos,  head) in query.iter_mut() {

        match head.dir {
            Direction::Left => {
                pos.x -= 1;
            },
            Direction::Up => {
                pos.y += 1;
            },
            Direction::Right => {
                pos.x += 1;
            },
            Direction::Down => {
                pos.y -= 1;
            },
        }
 
    }
}


fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn food_spawner(mut commands: Commands) {
    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        }
    ).insert(Food)
    .insert(
        Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        }
    ).insert(Size::square(0.8));
}

