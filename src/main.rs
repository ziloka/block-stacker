use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::seq::SliceRandom;

mod consts;
use consts::{ActivePiece, BOARD, KEYS, TETRIMINO_TYPES};

mod board;
use board::Board;

mod piece;

fn main() {
    App::new()
        // change background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(selected_tetrimino_movement_system)
        // .add_system(tetrimino_gravity)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
    let starting_piece = TETRIMINO_TYPES.choose(&mut rand::thread_rng()).unwrap();
    let board = Board {
        active_piece: starting_piece.clone(),
    };

    // spawn the current piece available to move around
    commands
        .spawn((
            board.active_piece,
            ActivePiece,
            // https://www.reddit.com/r/bevy/comments/xs792h/help_with_understanding_child_parent_relationships/
            SpatialBundle {
                transform: Transform::from_xyz(
                    BOARD::TOP_RIGHT_CORNER.x,
                    BOARD::TOP_RIGHT_CORNER.y,
                    1.0,
                ),
                visibility: Visibility { is_visible: true },
                ..default()
            },
        ))
        .with_children(|parent| {
            for delta in starting_piece.get_structure().iter() {
                // position is relative to the parent
                let x = BOARD::TETRIOMINO_SIDE_LENGTH * delta.x;
                let y = BOARD::TETRIOMINO_SIDE_LENGTH * delta.y;
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform {
                        translation: Vec3 { x: x, y: y, z: 1.0 },
                        rotation: Quat::default(),
                        scale: Vec3::splat(BOARD::TETRIOMINO_SIDE_LENGTH),
                    },
                    material: materials.add(ColorMaterial::from(board.active_piece.get_color())),
                    visibility: Visibility { is_visible: true },
                    ..default()
                });
            }
        });

    // Spawn boxes that represent the board
    for i in 1..=BOARD::WIDTH as u8 {
        // 1 to 10
        for j in 1..=BOARD::HEIGHT as u8 {
            // 1 to 20
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: Vec3 {
                        x: BOARD::BOTTOM_LEFT_CORNER.x + (i as f32 * BOARD::TETRIOMINO_SIDE_LENGTH),
                        y: BOARD::BOTTOM_LEFT_CORNER.y + (j as f32 * BOARD::TETRIOMINO_SIDE_LENGTH),
                        z: 0.0,
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

// fn tetrimino_gravity(mut commands: Commands, query: Query<&ActivePiece>) {}

// https://bevy-cheatbook.github.io/input/keyboard.html
fn selected_tetrimino_movement_system(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<ActivePiece>>,
) {
    // https://bevy-cheatbook.github.io/features/transforms.html?highlight=transform#transform-components
    let mut selected_tetrimino = query.single_mut();
    for ev in key_evr
        .iter()
        .filter(|key| key.state == ButtonState::Pressed)
    {
        match ev.key_code {
            Some(KEYS::CLOCKWISE) => {
                selected_tetrimino.rotation *= Quat::from_rotation_z(-90.0_f32.to_radians())
            }
            Some(KEYS::COUNTER_CLOCKWISE) => {
                selected_tetrimino.rotation *= Quat::from_rotation_z(90.0_f32.to_radians())
            }
            Some(KEYS::MOVE_LEFT) => {
                selected_tetrimino.translation.x -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
            }
            Some(KEYS::MOVE_RIGHT) => {
                selected_tetrimino.translation.x += 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
            }
            Some(KEYS::SOFTDROP) => {
                selected_tetrimino.translation.y -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
            }
            Some(KEYS::HARDDROP) => println!("harddrop key"),
            Some(_) => println!("That key is not registered"),
            None => println!("Somehow nothing was pressed in the keyboard pressed event ??"),
        }
        println!("{}", selected_tetrimino.translation);
    }
}
