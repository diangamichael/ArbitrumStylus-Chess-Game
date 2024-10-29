// src/main.rs
mod board;
mod piece;
mod game;

use game::Game;
use stylus-sdk::contract;

#[contract]
pub struct ChessContract {
    game: Game,
}

#[contract_method]
impl ChessContract {
    pub fn new(player1: Address, player2: Address) -> Self {
        ChessContract {
            game: Game::new(player1, player2),
        }
    }

    pub fn play_turn(&mut self, from: (usize, usize), to: (usize, usize), caller: Address) -> Result<(), String> {
        self.game.play_turn(from, to, caller)
    }
}
