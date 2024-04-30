use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("7t4yFy9tzfAM3uWeuRC1Fh6QGRyKToyQGZdj7uAZ9XeB");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        instructions::initialize_game(ctx)
    }
    pub fn join_game(ctx: Context<JoinGame>, initiator: Pubkey) -> Result<()> {
        // Initiator is used as a seed for PDA derivation and validation, hence not directly referenced.
        let _ = initiator;
        instructions::join_game(ctx)
    }

    pub fn make_move(ctx: Context<MakeMove>, initiator: Pubkey, column: u8, row: u8) -> Result<()> {
        // Initiator is used as a seed for PDA derivation and validation, hence not directly referenced.
        let _ = initiator;
        instructions::make_move(ctx, column, row)
    }
}
