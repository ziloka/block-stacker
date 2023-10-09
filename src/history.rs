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
        println!("there are current {} saved states, meaning {:.5} mb of memory is being used to save states", self.undo.len(), std::mem::size_of::<Board>() as f64 * self.undo.len() as f64 * f64::powi(10., -6));
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