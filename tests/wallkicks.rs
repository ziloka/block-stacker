// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration

use tetris::tetris::{board::Board, consts::vec2};

macro_rules! print_matrix {
    ($matrix: expr) => {
        let mut board_string = String::new();
        for row in $matrix.iter() {
            for element in row.iter().rev() {
                if let Some(_) = element {
                    board_string.insert_str(0, "x");
                } else {
                    board_string.insert_str(0, " ");
                }
            }
            board_string.insert_str(0, "\n");
        }
        // write!(f, "{}", board_string);
        println!("{}", board_string);
    };
}

macro_rules! generate {
    ($matrix: expr, $tetromino_type: expr, $initial_pos: expr, $rotation_index: literal, $dest_pos: expr) => {
        // $matrix.reverse(); // this line does not work
        print_matrix!($matrix);
        let mut board = Board::import($matrix, 0);
        board.active_piece.tetromino = $tetromino_type;
        board.active_piece.dots = $initial_pos;
        board.active_piece.rotation_index = $rotation_index;
        board.rotate_tetromino_90(true, true);
        // dbg!(&board.active_piece.dots);
        dbg!(&board);
        assert!(
            board
                .active_piece
                .dots
                .iter()
                .all(|item| $dest_pos.contains(item)),
            "expected = {:?}\nfound = {:?}",
            $dest_pos,
            board.active_piece.dots
        );
    };
}

const G: Option<(u8, u8, u8)> = Some((0, 0, 0));
const N: Option<(u8, u8, u8)> = None;

#[test]
fn tetromino_j_left_to_right_wallkicks() {
    // 0->R
    generate!(
        vec![
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![N, N, N, N, N],
            vec![G, G, N, N, N],
            vec![G, N, N, N, N],
            vec![G, N, N, N, N],
            vec![G, N, N, G, N],
        ],
        tetris::tetris::consts::Tetromino::L,
        vec![vec2(2., 1.), vec2(3., 2.), vec2(1., 1.), vec2(3., 1.)],
        0,
        vec![vec2(1., 2.), vec2(1., 1.), vec2(1., 0.), vec2(2., 0.)]
    );

    let negative_1x_negative_2y = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![2, 2, 0, 1, 0],
        vec![2, 1, 1, 1, 0],
        vec![2, 3, 2, 2, 2],
        vec![2, 3, 2, 2, 2],
        vec![2, 3, 3, 2, 2],
    ];

    // R-> 2
    let positive_1x_negative_1y = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 2, 2],
        vec![2, 1, 1, 3, 2],
        vec![2, 3, 2, 2, 2],
    ];

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
