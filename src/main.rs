use bevy::{prelude::*, sprite::MaterialMesh2dBundle, input::{keyboard::KeyboardInput, ButtonState}};
use rand::seq::SliceRandom;

mod consts;
use consts::{Board, TetriminoType, TopRightCorner, TETRIMINO_TYPES};

fn main() {
    App::new()
        // change background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(selected_tetrimino_movement_system)
        .run();
}

#[derive(Component)]
struct nothing {
    iamnothing: bool,
}

#[derive(Component)]
struct SelectedTetrimino {
    tetrimino_type: TetriminoType,
}

// Bundles are like "templates", to make it easy to create entities with a common set of components.
#[derive(Component)]
struct BoardBundle;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
    let starting_piece = TETRIMINO_TYPES.choose(&mut rand::thread_rng()).unwrap();

    commands
        .spawn()
        .insert(SelectedTetrimino {
            tetrimino_type: starting_piece.clone(),
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            // If you are overriding the camera transform / creating your own transform, you need to do this! The default transform (with Z=0.0) will place the camera so that your sprites (at positive +Z coordinates) would be behind the camera, and you wouldn't see them!
            // https://bevy-cheatbook.github.io/pitfalls/2d-camera-z.html
            transform: Transform {
                translation: Vec3 {
                    x: TopRightCorner::X - (Board::WIDTH / 2.0 * Board::TETRIOMINO_SIDE_LENGTH),
                    y: TopRightCorner::Y,
                    z: 2.0,
                },
                rotation: Quat::default(),
                scale: Vec3::splat(Board::TETRIOMINO_SIDE_LENGTH),
            },
            material: materials.add(ColorMaterial::from(starting_piece.get_color())),
            ..default()
        });

    // Spawn rectangles
    for i in 1..=Board::WIDTH as u8 {
        for j in 1..=Board::HEIGHT as u8 {
            // let tetrimino_type = TETRIMINO_TYPES;
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    // If you are overriding the camera transform / creating your own transform, you need to do this! The default transform (with Z=0.0) will place the camera so that your sprites (at positive +Z coordinates) would be behind the camera, and you wouldn't see them!
                    // https://bevy-cheatbook.github.io/pitfalls/2d-camera-z.html
                    transform: Transform {
                        translation: Vec3 {
                            x: (i as f32 * Board::TETRIOMINO_SIDE_LENGTH) - TopRightCorner::X,
                            y: (j as f32 * Board::TETRIOMINO_SIDE_LENGTH) - TopRightCorner::Y,
                            z: 1.0,
                        },
                        rotation: Quat::default(),
                        scale: Vec3::splat(Board::TETRIOMINO_SIDE_LENGTH),
                    },
                    material: materials.add(ColorMaterial::from(Color::GRAY)),
                    ..default()
                })
                .insert(nothing { iamnothing: true });
        }
    }
}

// https://bevy-cheatbook.github.io/input/keyboard.html
fn selected_tetrimino_movement_system(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(&SelectedTetrimino)>,
) {
    let mut selected_tetrimino_transform = query.single_mut();
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}
