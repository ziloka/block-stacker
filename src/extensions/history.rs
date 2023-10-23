// use memento design pattern to implement undo/redo
// todo: use command design pattern instead for less memory usage

use std::collections::VecDeque;

use crate::core::board::Board;

#[derive(Default)]
pub struct History {
    undo: VecDeque<Box<Board>>,
    redo: VecDeque<Box<Board>>,
}

impl History {
    pub fn add_state(&mut self, board: Box<Board>) {
        self.undo.push_front(board);
    }

    pub fn undo(&mut self) -> Option<Box<Board>> {
        let board = self.undo.pop_front();
        self.redo.push_front(board.clone()?);
        board
    }

    pub fn redo(&mut self) -> Option<Box<Board>> {
        self.redo.pop_front()
    }
}
