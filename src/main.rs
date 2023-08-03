#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// https://github.com/bevyengine/bevy/blob/latest/examples/window/screenshot.rs

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowResolution},
    utils::tracing::Level,
    log::LogPlugin
};
use rand::seq::SliceRandom;

mod consts;
use consts::{ActivePiece, BOARD, KEYS, TETRIMINO_TYPES, TetriminoType};

mod piece;

fn main() {
    App::new()
        // change background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "tetris".to_string(),
                        resolution: WindowResolution::new(
                            BOARD::WIDTH * BOARD::TETRIOMINO_SIDE_LENGTH * 1.25,
                            BOARD::HEIGHT * BOARD::TETRIOMINO_SIDE_LENGTH * 1.25,
                        ),
                        present_mode: PresentMode::AutoVsync,
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                }),
        )
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, selected_tetrimino_movement_system)
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
    
    let board = spawn_board(&mut commands, &mut meshes, &mut materials, starting_piece);

    // spawn the current piece available to move around
    commands
        .spawn((
            board.active_piece,
            ActivePiece,
            // https://www.reddit.com/r/bevy/comments/xs792h/help_with_understanding_child_parent_relationships/
            SpatialBundle {
                transform: Transform::from_xyz(
                    0.0,
                    BOARD::TOP_RIGHT_CORNER.y - (1.0 * BOARD::TETRIOMINO_SIDE_LENGTH),
                    1.0,
                ),
                visibility: Visibility::Visible,
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
                    visibility: Visibility::Visible,
                    ..default()
                });
            }
        });

    
}

fn spawn_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    starting_piece: &TetriminoType
) -> BOARD {
    let board = BOARD {
        active_piece: starting_piece.clone(),
    };

    commands
        .spawn((
            board,
            // https://www.reddit.com/r/bevy/comments/xs792h/help_with_understanding_child_parent_relationships/
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                visibility: Visibility::Visible,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Spawn boxes that represent the board
            for i in 1..=BOARD::WIDTH as u8 {
                // 1 to 10
                for j in 1..=BOARD::HEIGHT as u8 {
                    // 1 to 20
                    parent.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform {
                            translation: Vec3 {
                                x: BOARD::BOTTOM_LEFT_CORNER.x
                                    + (i as f32 * BOARD::TETRIOMINO_SIDE_LENGTH),
                                y: BOARD::BOTTOM_LEFT_CORNER.y
                                    + (j as f32 * BOARD::TETRIOMINO_SIDE_LENGTH),
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
        });

        board
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
                if selected_tetrimino.translation.x - 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
                    >= BOARD::BOTTOM_LEFT_CORNER.x + BOARD::TETRIOMINO_SIDE_LENGTH
                {
                    selected_tetrimino.translation.x -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH;
                }
            }
            Some(KEYS::MOVE_RIGHT) => {
                // println!("{}, top right corner: {:?}", selected_tetrimino.translation, BOARD::TOP_RIGHT_CORNER);
                if selected_tetrimino.translation.x + 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
                    <= BOARD::TOP_RIGHT_CORNER.x + BOARD::TETRIOMINO_SIDE_LENGTH
                {
                    selected_tetrimino.translation.x += 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH;
                }
            }
            Some(KEYS::SOFTDROP) => {
                if selected_tetrimino.translation.y - 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH
                    >= BOARD::BOTTOM_LEFT_CORNER.y + BOARD::TETRIOMINO_SIDE_LENGTH
                {
                    selected_tetrimino.translation.y -= 1.0 * BOARD::TETRIOMINO_SIDE_LENGTH;
                }
            }
            Some(KEYS::HARDDROP) => println!("harddrop key"),
            Some(_) => println!("That key is not registered"),
            None => println!("Somehow nothing was pressed in the keyboard pressed event ??"),
        }
        // println!("{}", selected_tetrimino.translation);
    }
}
