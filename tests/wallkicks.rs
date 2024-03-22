// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration

use tetris::core::logic::{
    board::Board,
    consts::{vec2, Tetromino, Vec2},
};

// make sure the first element in the initial_pos is the origin cell
// if a test fails, to prevent head pain, make sure that the first piece of the initial_pos parameter is the tetromino origin point
// https://harddrop.com/wiki/File:SRS-true-rotations.png
fn generate(
    matrix: Vec<Vec<Option<(u8, u8, u8)>>>,
    tetromino_type: Tetromino,
    initial_pos: Vec<Vec2>,
    rotation_index: i8,
    dest_pos: Vec<Vec2>,
) {
    assert!(
        initial_pos.len() == 4 && initial_pos.len() == dest_pos.len(),
        "expected initial and dest vectors contain 4 cells, found: init: {}, dest: {}",
        initial_pos.len(),
        dest_pos.len()
    );

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
fn tetromino_j_90_clockwise_wallkicks() {
    // 0->R (-1, 0)
    generate(
        vec![
            vec![G, N, N, G, N],
            vec![G, N, N, N, N],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(2., 1.), vec2(3., 2.), vec2(1., 1.), vec2(3., 1.)],
        0,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(1., 0.), vec2(2., 0.)],
    );

    // 0->R (-1,-2)
    generate(
        vec![
            vec![G, N, N, G, G],
            vec![G, N, G, G, G],
            vec![G, N, G, G, G],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(2., 3.), vec2(3., 4.), vec2(1., 3.), vec2(3., 3.)],
        0,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(1., 0.), vec2(2., 0.)],
    );

    // R->2 (+1,-1)
    generate(
        vec![
            vec![G, N, G, G, G],
            vec![G, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(1., 2.), vec2(1., 3.), vec2(1., 1.), vec2(2., 1.)],
        1,
        vec![vec2(3., 1.), vec2(1., 0.), vec2(1., 1.), vec2(2., 1.)],
    );

    // R->2 (+1, 0)
    generate(
        vec![
            vec![G, N, N, G, G],
            vec![N, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(1., 1.), vec2(1., 2.), vec2(1., 0.), vec2(2., 0.)],
        1,
        vec![vec2(2., 1.), vec2(3., 1.), vec2(1., 1.), vec2(1., 0.)],
    );

    // 2->L (+1,-2)
    generate(
        vec![
            vec![G, G, G, N, G],
            vec![G, G, G, N, G],
            vec![N, N, N, N, G],
            vec![N, N, N, N, G],
            vec![N, N, G, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(2., 3.), vec2(1., 3.), vec2(3., 3.), vec2(1., 2.)],
        2,
        vec![vec2(2., 2.), vec2(3., 2.), vec2(3., 1.), vec2(3., 0.)],
    );

    // L->0 (-1,-1)
    generate(
        vec![
            vec![G, N, N, N, G],
            vec![G, G, G, N, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::J,
        vec![vec2(3., 1.), vec2(2., 2.), vec2(3., 2.), vec2(3., 0.)],
        3,
        vec![vec2(1., 0.), vec2(2., 0.), vec2(3., 0.), vec2(3., 1.)],
    );
}

#[test]
fn tetromino_l_90_clockwise_wallkicks() {
    // 0->1 (-1, 0)
    generate(
        vec![
            vec![G, N, G, N, G],
            vec![G, N, N, N, N],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(2., 1.), vec2(1., 2.), vec2(1., 1.), vec2(3., 1.)],
        0,
        vec![vec2(2., 2.), vec2(1., 0.), vec2(1., 1.), vec2(1., 2.)],
    );

    // 0->1 ( 0,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, G, N, G, G],
            vec![G, G, N, N, G],
            vec![N, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, G, G, G],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(2., 3.), vec2(1., 4.), vec2(1., 3.), vec2(3., 3.)],
        0,
        vec![vec2(2., 2.), vec2(3., 2.), vec2(2., 1.), vec2(2., 0.)],
    );

    // 1->2 (+1,-1)
    generate(
        vec![
            vec![N, G, G, N, G],
            vec![N, N, N, N, G],
            vec![N, N, G, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(1., 2.), vec2(1., 3.), vec2(2., 3.), vec2(1., 1.)],
        1,
        vec![vec2(2., 1.), vec2(3., 1.), vec2(3., 0.), vec2(1., 1.)],
    );

    // 1->2 (+1, 0)
    generate(
        vec![
            vec![N, N, G, N, G],
            vec![N, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(1., 1.), vec2(1., 2.), vec2(2., 2.), vec2(1., 0.)],
        1,
        vec![vec2(2., 1.), vec2(3., 1.), vec2(3., 0.), vec2(1., 1.)],
    );

    // 2->3 (+1,-2)
    generate(
        vec![
            vec![G, G, N, N, G],
            vec![G, G, G, N, G],
            vec![G, G, G, N, G],
            vec![G, N, N, N, G],
            vec![G, N, G, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(2., 3.), vec2(1., 3.), vec2(3., 3.), vec2(3., 2.)],
        2,
        vec![vec2(3., 1.), vec2(2., 0.), vec2(3., 0.), vec2(3., 2.)],
    );

    // 3->0 (-1,-1)
    generate(
        vec![
            vec![G, N, N, N, N],
            vec![G, N, G, N, N],
            vec![G, G, G, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(3., 1.), vec2(3., 2.), vec2(2., 0.), vec2(3., 0.)],
        3,
        vec![vec2(1., 1.), vec2(1., 0.), vec2(2., 0.), vec2(3., 0.)],
    );

    // 3->0 (-1, 0)
    generate(
        vec![
            vec![G, G, N, N, N],
            vec![G, N, N, N, N],
            vec![G, N, G, N, N],
            vec![G, G, G, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::L,
        vec![vec2(3., 1.), vec2(3., 2.), vec2(2., 0.), vec2(3., 0.)],
        3,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(2., 1.), vec2(3., 1.)],
    );
}

#[test]
fn tetromino_t_90_clockwise_wallkicks() {
    // 0->1 (-1,-2)
    generate(
        vec![
            vec![G, N, G, G, G],
            vec![G, N, N, G, G],
            vec![G, N, G, G, G],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(2., 3.), vec2(2., 4.), vec2(1., 3.), vec2(3., 3.)],
        0,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(2., 1.), vec2(1., 0.)],
    );

    // 0->1 (-1, 0)
    generate(
        vec![
            vec![G, N, G, G, G],
            vec![G, N, N, N, N],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(2., 1.), vec2(2., 2.), vec2(1., 1.), vec2(3., 1.)],
        0,
        vec![vec2(1., 2.), vec2(1., 0.), vec2(1., 1.), vec2(2., 1.)],
    );

    // 1->2 ( 0, 0)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, N, N, N, G],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(2., 1.), vec2(2., 2.), vec2(3., 1.), vec2(2., 0.)],
        1,
        vec![vec2(1., 1.), vec2(2., 1.), vec2(2., 0.), vec2(3., 1.)],
    );

    // 1->2 (+1,-1)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, N, N, N, G],
            vec![G, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(1., 2.), vec2(1., 3.), vec2(2., 2.), vec2(1., 1.)],
        1,
        vec![vec2(2., 1.), vec2(3., 1.), vec2(2., 0.), vec2(1., 1.)],
    );

    // 2->3 (+1,-2)
    generate(
        vec![
            vec![G, G, G, N, G],
            vec![G, G, N, N, G],
            vec![N, N, N, N, G],
            vec![N, N, N, N, G],
            vec![N, N, G, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(2., 3.), vec2(1., 3.), vec2(3., 3.), vec2(2., 2.)],
        2,
        vec![vec2(3., 2.), vec2(2., 1.), vec2(3., 1.), vec2(3., 0.)],
    );

    // 2->3 ( 0,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, N, N, G, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, G],
            vec![N, N, G, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(2., 3.), vec2(1., 3.), vec2(3., 3.), vec2(2., 2.)],
        2,
        vec![vec2(1., 1.), vec2(2., 1.), vec2(2., 0.), vec2(2., 2.)],
    );

    // 3->0 (-1,-1)
    generate(
        vec![
            vec![G, N, N, N, G],
            vec![G, G, N, N, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(3., 1.), vec2(3., 2.), vec2(2., 1.), vec2(3., 0.)],
        3,
        vec![vec2(1., 0.), vec2(2., 0.), vec2(2., 1.), vec2(3., 0.)],
    );

    // 3->0 (-1, 0)
    generate(
        vec![
            vec![G, G, G, N, G],
            vec![G, N, N, N, G],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::T,
        vec![vec2(3., 1.), vec2(3., 2.), vec2(2., 1.), vec2(3., 0.)],
        3,
        vec![vec2(2., 2.), vec2(1., 1.), vec2(3., 1.), vec2(2., 1.)],
    );
}

#[test]
fn tetromino_s_90_clockwise_wallkicks() {
    // 0->1 (-1,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, N, N, G, G],
            vec![G, N, G, G, G],
            vec![G, N, N, N, N],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(2., 3.), vec2(2., 4.), vec2(3., 4.), vec2(1., 3.)],
        0,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(2., 1.), vec2(2., 0.)],
    );

    // 0->1 (0,-2)
    generate(
        vec![
            vec![G, G, G, N, G],
            vec![G, G, N, N, G],
            vec![G, G, N, G, G],
            vec![G, N, N, N, N],
            vec![G, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(2., 3.), vec2(2., 4.), vec2(3., 4.), vec2(1., 3.)],
        0,
        vec![vec2(2., 2.), vec2(2., 1.), vec2(3., 1.), vec2(3., 0.)],
    );

    // 1->2 (+1,-1)
    generate(
        vec![
            vec![G, N, N, G, G],
            vec![G, G, N, N, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(1., 2.), vec2(1., 3.), vec2(2., 2.), vec2(2., 1.)],
        1,
        vec![vec2(3., 1.), vec2(1., 0.), vec2(2., 0.), vec2(2., 1.)],
    );

    // 1->2 (+1, 0)
    generate(
        vec![
            vec![G, N, N, G, G],
            vec![N, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(1., 1.), vec2(1., 2.), vec2(2., 1.), vec2(2., 0.)],
        1,
        vec![vec2(3., 1.), vec2(1., 0.), vec2(2., 1.), vec2(2., 0.)],
    );

    // 2->3 ( 0,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, N, N, G, G],
            vec![G, N, N, G, G],
            vec![G, G, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(2., 3.), vec2(3., 3.), vec2(1., 2.), vec2(2., 2.)],
        2,
        vec![vec2(1., 1.), vec2(2., 1.), vec2(2., 0.), vec2(1., 2.)],
    );

    // 3->0 (-1, 0)
    generate(
        vec![
            vec![G, G, G, N, G],
            vec![G, N, N, N, G],
            vec![G, G, N, N, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::S,
        vec![vec2(3., 1.), vec2(2., 2.), vec2(2., 1.), vec2(3., 0.)],
        3,
        vec![vec2(3., 2.), vec2(1., 1.), vec2(2., 2.), vec2(2., 1.)],
    );
}

#[test]
fn tetromino_z_90_clockwise_wallkicks() {
    // 0->1 ( 0,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, G, N, N, G],
            vec![G, G, G, N, G],
            vec![N, G, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(2., 3.), vec2(1., 4.), vec2(2., 4.), vec2(3., 3.)],
        0,
        vec![vec2(3., 2.), vec2(2., 1.), vec2(3., 1.), vec2(2., 0.)],
    );

    // 0->1 ( 0, 0)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![N, N, N, N, G],
            vec![N, N, N, N, G],
            vec![N, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(2., 1.), vec2(1., 2.), vec2(2., 2.), vec2(3., 1.)],
        0,
        vec![vec2(3., 2.), vec2(2., 0.), vec2(2., 1.), vec2(3., 1.)],
    );

    // 1->2 (+1,-1)
    generate(
        vec![
            vec![G, G, N, N, G],
            vec![G, N, N, G, G],
            vec![G, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(1., 2.), vec2(2., 3.), vec2(2., 2.), vec2(1., 1.)],
        1,
        vec![vec2(2., 1.), vec2(2., 0.), vec2(3., 0.), vec2(1., 1.)],
    );

    // 1->2 (+1, 0)
    generate(
        vec![
            vec![G, N, N, N, G],
            vec![G, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(1., 1.), vec2(2., 2.), vec2(2., 1.), vec2(1., 0.)],
        1,
        vec![vec2(2., 0.), vec2(3., 0.), vec2(1., 1.), vec2(2., 1.)],
    );

    // 2->3 (+1,-2)
    generate(
        vec![
            vec![G, G, N, G, G],
            vec![G, G, N, N, G],
            vec![G, G, N, N, G],
            vec![G, N, N, G, G],
            vec![G, N, N, G, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(2., 3.), vec2(1., 3.), vec2(2., 2.), vec2(3., 2.)],
        2,
        vec![vec2(2., 1.), vec2(3., 1.), vec2(2., 0.), vec2(3., 2.)],
    );

    // 3->0 ( 0, 0)
    generate(
        vec![
            vec![G, N, G, G, G],
            vec![G, N, N, N, G],
            vec![G, N, N, G, G],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
        ],
        Tetromino::Z,
        vec![vec2(2., 1.), vec2(2., 2.), vec2(1., 1.), vec2(1., 0.)],
        3,
        vec![vec2(1., 2.), vec2(3., 1.), vec2(2., 2.), vec2(2., 1.)],
    );
}

#[test]
fn tetromino_i_90_clockwise_wallkicks() {}
