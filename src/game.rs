use crate::{
    core::logic::{board::Board, consts::Tetromino},
    input::Input,
};
// use macroquad::miniquad::date::now;
use std::{boxed::Box, sync::mpsc::{Receiver, Sender}};

pub struct Game {
    pub players: Vec<Box<dyn Player>>,
    pub input: Input,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            // board: Box::new(Board::new(now() as usize)),
            players: Vec::new(),
            input: Input::default(),
        }
    }
}

pub trait Player {
    fn on_next_piece(&mut self, Tetromino: Tetromino);

    fn do_work(&self) {}
}

pub struct Human {
    board: Box<Board>,
    recv: Receiver<>,
    sender: Sender<>
    pub input: Input,
}

impl Default for Human {
    fn default() -> Self {
        Self {
            board: Box::new(Board::new(0)),
            input: Input::default(),
        }
    }
}

impl Player for Human {
    fn on_next_piece(&mut self, tetromino: Tetromino) {}
}

pub struct Bot {
    board: Box<Board>,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            board: Box::new(Board::new(0)),
        }
    }
}

impl Player for Bot {
    fn on_next_piece(&mut self, tetromino: Tetromino) {}
}

pub struct Room {
    players: Vec<Box<dyn Player>>,
    id: u64,
    name: String,
    // password: String,
    max_players: u8,
    current_players: u8,
    // is_public: bool,
    is_active: bool,
    is_started: bool,
    is_finished: bool,
    is_paused: bool,
    is_spectator: bool,
    is_private: bool,
    is_practice: bool,
    // is_tournament: bool
}

impl Room {
    pub fn new(
        id: u64,
        name: String,
        max_players: u8,
        is_private: bool,
        is_practice: bool,
    ) -> Self {
        Self {
            players: Vec::new(),
            id,
            name,
            // password,
            max_players,
            current_players: 0,
            // is_public,
            is_active: false,
            is_started: false,
            is_finished: false,
            is_paused: false,
            is_spectator: false,
            // is_ranked,
            is_private,
            is_practice,
            // is_tournament
        }
    }

    pub fn add_player(&mut self, player: Box<dyn Player>) {
        self.players.push(player);
        self.current_players += 1;
    }

    pub fn remove_player(&mut self, index: usize) {
        self.players.remove(index);
        self.current_players -= 1;
    }

    pub fn start(&mut self) {
        self.is_started = true;
        self.is_active = true;

        while !self.is_finished {
            for player in &mut self.players {
                player.do_work();
            }
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn finish(&mut self) {
        self.is_finished = false;
        self.is_active = false;
    }

    pub fn is_full(&self) -> bool {
        self.current_players == self.max_players
    }

    pub fn is_empty(&self) -> bool {
        self.current_players == 0
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn is_spectator(&self) -> bool {
        self.is_spectator
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }

    pub fn is_practice(&self) -> bool {
        self.is_practice
    }
}
