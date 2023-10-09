// use memento design pattern to implement undo/redo
// todo: use command design pattern instead

use std::collections::VecDeque;

use crate::tetris::board::Board;

#[derive(Default)]
pub struct History<'a> {
    undo: VecDeque<Box<Board<'a>>>,
    redo: VecDeque<Box<Board<'a>>>
}

impl<'a> History<'a> {
    pub fn add_state(&mut self, board: Box<Board<'a>>) {
        self.undo.push_front(board);
    }

    pub fn undo(&mut self) -> Option<Box<Board<'a>>> {
        let board = self.undo.pop_front();
        self.redo.push_front(board.clone()?);
        board
    }

    pub fn redo(&mut self) -> Option<Box<Board<'a>>> {
        self.redo.pop_front()
    }
}