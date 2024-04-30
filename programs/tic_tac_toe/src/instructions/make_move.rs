use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn make_move(ctx: Context<MakeMove>, column: u8, row: u8) -> Result<()> {
    ctx.accounts
        .game
        .make_move(row as usize, column as usize, ctx.accounts.player.key())
}

#[derive(Accounts)]
#[instruction(initiator: Pubkey)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [b"game", initiator.key().as_ref()], bump, constraint = game.initiator == initiator.key())]
    pub game: Account<'info, Game>,
}
