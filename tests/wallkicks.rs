// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration

use tetris::tetris::{board::Board, consts::vec2};

const G: Option<(u8, u8, u8)> = Some((0, 0, 0));
const N: Option<(u8, u8, u8)> = None;

#[test]
fn tetromino_j_left_to_right_wallkicks() {
    // 0->R
    let negative_1x_0y = vec![vec![N,N,N,N,N],vec![N,N,N,N,N],vec![N,N,N,N,N],vec![G,G,N,N,N],vec![G,N,N,N,N],vec![G,N,N,N,N],vec![G,N,N,G,N]];
    let mut board = Board::import(negative_1x_0y, 0);
    board.active_piece.dots = vec![vec2(4., 3.), vec2(5., 1.), vec2(5., 2.), vec2(5., 3.)];
    board.rotate_tetromino_90(true, true);
    assert!(board.active_piece.dots.iter().all(|item| vec![vec2(4., 1.), vec2(6., 1.), vec2(6., 2.)].contains(item)));
    
    let negative_1x_negative_2y = vec![
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 2, 2, 0, 1, 0 ],
        vec![ 2, 1, 1, 1, 0 ],
        vec![ 2, 3, 2, 2, 2 ],
        vec![ 2, 3, 2, 2, 2 ],
        vec![ 2, 3, 3, 2, 2 ]
      ];

      // R-> 2
    let positive_1x_negative_1y = vec![
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 1, 0, 0, 0 ],
        vec![ 0, 1, 0, 2, 2 ],
        vec![ 2, 1, 1, 3, 2 ],
        vec![ 2, 3, 2, 2, 2 ]
    ];

    let positive_1x_0y = vec![
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 1, 0, 2, 2 ],
        vec![ 0, 1, 3, 3, 2 ],
        vec![ 2, 1, 1, 2, 2 ]
      ];

      // 2->L
      let positive_1x_negative_2y = vec![
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 0, 0, 0 ],
        vec![ 0, 0, 2, 2, 2 ],
        vec![ 0, 1, 1, 1, 2 ],
        vec![ 0, 1, 3, 3, 2 ],
        vec![ 2, 2, 2, 3, 2 ],
        vec![ 2, 2, 2, 3, 2 ]
      ];

      // L->0
      let negative_1x_negative_1y = [
        [ 0, 0, 0, 0, 0 ],
        [ 0, 0, 0, 0, 0 ],
        [ 0, 0, 0, 0, 0 ],
        [ 0, 0, 0, 0, 0 ],
        [ 0, 0, 1, 1, 0 ],
        [ 2, 2, 2, 1, 2 ],
        [ 2, 3, 3, 1, 2 ]
      ];

    
}

#[test]
fn tetromino_T_left_to_right_wallkicks() {}

#[test]
fn tetromino_S_left_to_right_wallkicks() {}



#[test]
fn tetromino_I_left_to_right_wallkicks() {}
