// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration

use tetris::tetris::{
    board::Board,
    consts::{vec2, Tetromino, Vec2},
};

// make sure the first element in the initial_pos is the origin cell
fn generate(
    matrix: Vec<Vec<Option<(u8, u8, u8)>>>,
    tetromino_type: Tetromino,
    initial_pos: Vec<Vec2>,
    rotation_index: i8,
    dest_pos: Vec<Vec2>,
) {
    assert!(initial_pos.len() == 4 && initial_pos.len() == dest_pos.len(), "expected initial and dest vectors contain 4 cells, found: init: {}, dest: {}", initial_pos.len(), dest_pos.len());

    let mut board = Board::import(matrix, 0);
    board.active_piece.tetromino = tetromino_type;
    board.active_piece.dots = initial_pos;
    board.active_piece.rotation_index = rotation_index;
    board.rotate_tetromino_90(true, true);
    assert!(
        board
            .active_piece
            .dots
            .iter()
            .all(|item| dest_pos.contains(item)),
        "expected = {:?}\nfound = {:?}",
        dest_pos,
        board.active_piece.dots
    );
}

const G: Option<(u8, u8, u8)> = Some((0, 0, 0));
const N: Option<(u8, u8, u8)> = None;

#[test]
fn tetromino_j_clockwise_wallkicks() {
    // 0->R (-1, 0)
    generate(
        vec![vec![G,N,N,G,N],vec![G,N,N,N,N],vec![G,N,N,N,N],vec![G,G,N,N,N],vec![N,N,N,N,N],vec![N,N,N,N,N],vec![N,N,N,N,N]],
        tetris::tetris::consts::Tetromino::L,
        vec![ vec2(2., 1.), vec2(1., 1.),  vec2(3., 1.), vec2(3., 2.)],
        0,
        vec![ vec2(1., 2.), vec2(1., 1.), vec2(1., 0.), vec2(2., 0.)],
    );

    // 0->R (-1, -2)
    generate(
        vec![vec![G,N,N,G,G],vec![G,N,G,G,G],vec![G,N,G,G,G],vec![G,N,N,N,N],vec![G,G,N,N,N],vec![N,N,N,N,N],vec![N,N,N,N,N]],
        tetris::tetris::consts::Tetromino::L,
        vec![ vec2(2., 3.), vec2(1., 3.), vec2(3., 3.), vec2(3., 4.)],
        0,
        vec![ vec2(1., 2.), vec2(1., 1.), vec2(1., 0.), vec2(2., 0.)],
    );

    // R-> 2 (+1, -1)
    let positive_1x_0y = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 2, 2],
        vec![0, 1, 3, 3, 2],
        vec![2, 1, 1, 2, 2],
    ];

    // 2->L
    let positive_1x_negative_2y = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 2, 2, 2],
        vec![0, 1, 1, 1, 2],
        vec![0, 1, 3, 3, 2],
        vec![2, 2, 2, 3, 2],
        vec![2, 2, 2, 3, 2],
    ];

    // L->0
    let negative_1x_negative_1y = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 1, 0],
        [2, 2, 2, 1, 2],
        [2, 3, 3, 1, 2],
    ];
}

#[test]
fn tetromino_T_left_to_right_wallkicks() {}

#[test]
fn tetromino_S_left_to_right_wallkicks() {}

#[test]
fn tetromino_I_left_to_right_wallkicks() {}
