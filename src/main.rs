//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use array2d::Array2D;

fn main() {
    let game_state = GameState {
        weight: 32,
        height: 32,
        board: create_map(32, 32),
        car: Car {
            position: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(10.0, 10.0),
        },
    };

    App::new()
        .insert_resource(game_state)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[derive(Component)]
struct Car {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Resource)]
struct GameState {
    board: Array2D<i32>,
    weight: i32,
    height: i32,
    car: Car,
}

/// This system prints 'A' key state
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        game_state.car.position.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        game_state.car.position.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        game_state.car.position.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        game_state.car.position.x += 1.0;
    }
    let translation = Transform::default()
        .with_translation(Vec3::new(
            game_state.car.position.x as f32 * 32.0,
            game_state.car.position.y as f32 * 32.0,
            0.0,
        ))
        .with_scale(Vec3::splat(32.));
}

fn create_map(height: usize, weight: usize) -> Array2D<i32> {
    let mut array = Array2D::filled_with(0, weight, height);

    for i in 10..20 {
        array[(i, 5)] = 1;
        array[(5, i)] = 1;
        array[(i, 25)] = 1;
        array[(25, i)] = 1;
    }

    for i in 0..5 {
        for j in 0..2 {
            array[(10 - i, 5 + j + i)] = 1;
            array[(10 - i, 25 - j - i)] = 1;

            array[(20 + i, 5 + j + i)] = 1;
            array[(20 + i, 25 - j - i)] = 1;
        }
    }

    return array;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: ResMut<GameState>,
) {
    commands.spawn(Camera2dBundle::default());

    let translation = Transform::default()
        .with_translation(Vec3::new(
            game_state.car.position.x as f32 * 32.0,
            game_state.car.position.y as f32 * 32.0,
            0.0,
        ))
        .with_scale(Vec3::splat(32.));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: translation,
            material: materials.add(Color::srgb(0.5, 0.0, 0.0)),
            ..default()
        },
        Car {
            position: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(0.0, 0.0),
        },
    ));

    for i in 0..game_state.weight {
        for j in 0..game_state.height {
            let translation = Transform::default()
                .with_translation(Vec3::new(i as f32 * 32.0, j as f32 * 32.0, 0.0))
                .with_scale(Vec3::splat(32.));

            let mut block_color = Color::WHITE;
            if game_state.board[(i as usize, j as usize)] == 1 {
                block_color = Color::BLACK;
            }

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: translation,
                material: materials.add(block_color),
                ..default()
            });
        }
    }
}
