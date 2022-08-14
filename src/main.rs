use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::seq::SliceRandom;


mod consts;
use consts::{KEYS, BOARD, TETRIMINO_TYPES};

mod board;
use board::{Board, SelectedTetrimino};

mod piece;
use piece::{Position};

fn main() {
    App::new()
        // change background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(selected_tetrimino_movement_system)
        .add_system(tetrimino_gravity)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
    let starting_piece = TETRIMINO_TYPES.choose(&mut rand::thread_rng()).unwrap();
    let board = Board {
        active_piece: SelectedTetrimino {
            tetrimino_type: starting_piece.clone(),
            position: Position {
                X: BOARD::TOP_RIGHT_CORNER.X - (BOARD::WIDTH / 2.0 * BOARD::TETRIOMINO_SIDE_LENGTH),
                Y: BOARD::TOP_RIGHT_CORNER.Y,
            },
        },
    };

    // spawn the current piece available to move around
    commands
        .spawn()
        .insert(board)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: Vec3 {
                    x: board.active_piece.position.X,
                    y: board.active_piece.position.Y,
                    z: 2.0,
                },
                rotation: Quat::default(),
                scale: Vec3::splat(BOARD::TETRIOMINO_SIDE_LENGTH),
            },
            material: materials.add(ColorMaterial::from(starting_piece.get_color())),
            ..default()
        });

    // Spawn rectangles that are "empty"
    for i in 1..=BOARD::WIDTH as u8 {
        for j in 1..=BOARD::HEIGHT as u8 {
            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: Vec3 {
                        x: (i as f32 * BOARD::TETRIOMINO_SIDE_LENGTH) - BOARD::TOP_RIGHT_CORNER.X,
                        y: (j as f32 * BOARD::TETRIOMINO_SIDE_LENGTH) - BOARD::TOP_RIGHT_CORNER.Y,
                        z: 1.0,
                    },
                    rotation: Quat::default(),
                    scale: Vec3::splat(BOARD::TETRIOMINO_SIDE_LENGTH),
                },
                material: materials.add(ColorMaterial::from(Color::GRAY)),
                ..default()
            });
        }
    }
}

fn tetrimino_gravity(mut commands: Commands, query: Query<&Board>) {

}

// https://bevy-cheatbook.github.io/input/keyboard.html
fn selected_tetrimino_movement_system(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<Board>>,
) {
    // https://bevy-cheatbook.github.io/features/transforms.html?highlight=transform#transform-components
    let mut board = query.single_mut();
    for ev in key_evr
        .iter()
        .filter(|key| key.state == ButtonState::Pressed)
    {
        match ev.key_code {
            Some(KEYS::CLOCKWISE) => println!("clockwise key"),
            Some(KEYS::COUNTER_CLOCKWISE) => println!("counter clockwise key"),
            Some(KEYS::MOVE_LEFT) => board.translation.x -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH,
            Some(KEYS::MOVE_RIGHT) => board.translation.x += 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH,
            Some(KEYS::SOFTDROP) => board.translation.y -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH,
            Some(KEYS::HARDDROP) => println!("harddrop key"),
            Some(_) => println!("That key is not registered"),
            None => println!("Somehow nothing was pressed in the keyboard pressed event ??"),
        }
    }
}
