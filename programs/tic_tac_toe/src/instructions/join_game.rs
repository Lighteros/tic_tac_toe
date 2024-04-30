use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
    ctx.accounts.game.join_game(ctx.accounts.player.key())
}

#[derive(Accounts)]
#[instruction(initiator: Pubkey)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [b"game", initiator.key().as_ref()], bump, constraint = game.initiator == initiator.key())]
    pub game: Account<'info, Game>,
}
