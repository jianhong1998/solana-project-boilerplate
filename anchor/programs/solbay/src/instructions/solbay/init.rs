use crate::states::Solbay;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeSolbay<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Solbay::INIT_SPACE,
  payer = payer
  )]
  pub solbay: Account<'info, Solbay>,
  pub system_program: Program<'info, System>,
}
