use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::seq::SliceRandom;

mod consts;
use consts::{
    TetriminoType, BOARD_HEIGHT, BOARD_WIDTH, BOTTOM_LEFT_CORNER, TETRIOMINO_SIDE_LENGTH,
};

fn main() {
    App::new()
        // change background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(selected_tetrimino)
        .run();
}

#[derive(Bundle)]
struct TetriminoBundle {
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
    let types = [
        TetriminoType::I,
        TetriminoType::O,
        TetriminoType::T,
        TetriminoType::S,
        TetriminoType::Z,
        TetriminoType::S,
        TetriminoType::J,
    ];

    println!("bottom left corner: x = {}, y = {}", BOTTOM_LEFT_CORNER.X, BOTTOM_LEFT_CORNER.Y);
    // Spawn rectangles
    for i in 1..=BOARD_WIDTH as u8 {
        for j in 1..=BOARD_HEIGHT as u8 {
            if (i as f32 == BOARD_WIDTH && j as f32 == BOARD_HEIGHT) || (i == 0 && j == 0) {
                println!("x = {}, y = {}", (i as f32 * TETRIOMINO_SIDE_LENGTH) - BOTTOM_LEFT_CORNER.X, (j as f32 * TETRIOMINO_SIDE_LENGTH) - BOTTOM_LEFT_CORNER.Y)
            }
            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: Vec3 {
                        x: (i as f32 * TETRIOMINO_SIDE_LENGTH) - BOTTOM_LEFT_CORNER.X,
                        y: (j as f32 * TETRIOMINO_SIDE_LENGTH) - BOTTOM_LEFT_CORNER.Y,
                        z: 0.0,
                    },
                    rotation: Quat::default(),
                    scale: Vec3::splat(40.0),
                },
                // material: materials.add(ColorMaterial::from(Color::GRAY)),
                material: materials.add(ColorMaterial::from(types.choose(&mut rand::thread_rng()).unwrap().get_color())),
                ..default()
            });
        }
    }
}

#[derive(Bundle)]
struct SelectedTetriminoBundle {
    tetriminoType: TetriminoType,
}

fn selected_tetrimino(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // println!("x = {}, y = {}", 0.0, (BOTTOM_LEFT_CORNER.Y * BOARD_HEIGHT) * TETRIOMINO_SIDE_LENGTH);
    // right now spawn a black dot in the middle of the "board"
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3 {
                x: BOTTOM_LEFT_CORNER.Y,
                y: BOTTOM_LEFT_CORNER.X,
                // y: (BOTTOM_LEFT_CORNER.Y * BOARD_HEIGHT) * TETRIOMINO_SIDE_LENGTH,
                z: 1.0,
            },
            rotation: Quat::default(),
            scale: Vec3::splat(40.0),
        },
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}
