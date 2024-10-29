// src/board.rs
use crate::piece::{Piece, Color};

#[derive(Debug, Clone)]
pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        // Initialize board with starting positions
        // Code to place pieces in their starting positions here...
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Piece> {
        self.grid[x][y].as_ref()
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        // Logic to move piece from one square to another
    }
}
