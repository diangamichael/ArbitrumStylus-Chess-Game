// src/game.rs
use crate::board::Board;
use crate::piece::{Color, Piece};
use stylus-sdk::Address;

pub struct Game {
    board: Board,
    turn: Color,
    player1: Address,
    player2: Address,
    moves: Vec<(usize, usize, usize, usize)>, // Store moves as (from_x, from_y, to_x, to_y)
    game_over: bool,
    winner: Option<Color>,
}

impl Game {
    pub fn new(player1: Address, player2: Address) -> Self {
        Game {
            board: Board::new(),
            turn: Color::White,
            player1,
            player2,
            moves: vec![],
            game_over: false,
            winner: None,
        }
    }

    pub fn play_turn(&mut self, from: (usize, usize), to: (usize, usize), caller: Address) -> Result<(), String> {
        if self.game_over {
            return Err("The game is over.".into());
        }

        if caller != self.current_player() {
            return Err("It's not your turn.".into());
        }

        // Logic to validate and record move
    }

    fn current_player(&self) -> Address {
        match self.turn {
            Color::White => self.player1,
            Color::Black => self.player2,
        }
    }
}
