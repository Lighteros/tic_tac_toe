use crate::errors::TicTacToeError;
use anchor_lang::{prelude::*, solana_program::hash::hash};

#[account]
pub struct Game {
    pub board: [[Sign; 3]; 3],   // 9
    pub status: GameState,       // 1
    pub player_o: Pubkey,        // 32
    pub player_x: Pubkey,        // 32
    pub current_turn: Sign,      // 1
    pub initiator: Pubkey,       // 32
    pub game_number: u64,        // 8
    pub game_result: GameResult, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Open,
    InProgress,
    Completed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    InProgress,
    PlayerX,
    PlayerO,
    Draw,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    X,
    O,
    Empty,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = 9 + 1 + 32 + 32 + 1 + 32 + 8 + 1;

    pub fn initialize_game(&mut self, initiator: Pubkey, game_number: u64) -> Result<()> {
        self.board = [[Sign::Empty; 3]; 3];
        self.status = GameState::Open;
        self.player_o = Pubkey::default();
        self.player_x = Pubkey::default();
        self.current_turn = Sign::Empty;
        self.initiator = initiator;
        self.game_number = game_number;
        self.game_result = GameResult::InProgress;
        Ok(())
    }

    pub fn join_game(&mut self, player: Pubkey) -> Result<()> {
        require!(
            self.status == GameState::Open,
            TicTacToeError::GameAlreadyStarted
        );

        let players = [self.initiator.to_bytes(), player.to_bytes()].concat();
        let players_hash = hash(&players);

        self.status = GameState::InProgress;
        self.current_turn = Sign::X;

        if players_hash.to_bytes()[0] % 2 == 0 {
            self.player_x = player;
            self.player_o = self.initiator;
        } else {
            self.player_x = self.initiator;
            self.player_o = player;
        }

        Ok(())
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: Pubkey) -> Result<()> {
        require!(
            self.status == GameState::InProgress,
            TicTacToeError::GameNotInProgress
        );
        let current_player = if self.player_x == player {
            require!(self.current_turn == Sign::X, TicTacToeError::NotPlayersTurn);
            Sign::X
        } else if self.player_o == player {
            require!(self.current_turn == Sign::O, TicTacToeError::NotPlayersTurn);
            Sign::O
        } else {
            return Err(TicTacToeError::InvalidPlayer.into());
        };

        require!(
            self.board[row][col] == Sign::Empty,
            TicTacToeError::CellAlreadyTaken
        );

        self.board[row][col] = current_player;

        self.update_game_result();

        if self.game_result != GameResult::InProgress {
            self.status = GameState::Completed;
        } else {
            self.current_turn = if self.current_turn == Sign::X {
                Sign::O
            } else {
                Sign::X
            };
        };

        Ok(())
    }

    fn is_winning_trio(&self, trio: [Sign; 3]) -> bool {
        trio[0] != Sign::Empty && trio[0] == trio[1] && trio[0] == trio[2]
    }

    fn update_game_result(&mut self) {
        for i in 0..2 {
            if self.is_winning_trio([self.board[0][i], self.board[1][i], self.board[2][i]]) {
                self.game_result = if self.board[0][i] == Sign::X {
                    GameResult::PlayerX
                } else {
                    GameResult::PlayerO
                };
                return;
            }
            if self.is_winning_trio([self.board[i][0], self.board[i][1], self.board[i][2]]) {
                self.game_result = if self.board[i][0] == Sign::X {
                    GameResult::PlayerX
                } else {
                    GameResult::PlayerO
                };
                return;
            }
        }

        if self.is_winning_trio([self.board[0][0], self.board[1][1], self.board[2][2]])
            || self.is_winning_trio([self.board[2][0], self.board[1][1], self.board[0][2]])
        {
            self.game_result = if self.board[1][1] == Sign::X {
                GameResult::PlayerX
            } else {
                GameResult::PlayerO
            };
            return;
        }

        // Game is not won, game has more empty seats
        if self.board.iter().flatten().any(|sign| *sign == Sign::Empty) {
            self.game_result = GameResult::InProgress;
            return;
        }

        // Game is drawn
        self.game_result = GameResult::Draw;
    }
}
