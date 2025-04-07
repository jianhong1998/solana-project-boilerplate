#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("EXMi6f7SEVuEZjVm4AK58QFBgisZVX8XcxJKEWwA3Hok");

#[program]
pub mod solbay {
    use super::*;

  pub fn close(_ctx: Context<CloseSolbay>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.solbay.count = ctx.accounts.solbay.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.solbay.count = ctx.accounts.solbay.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeSolbay>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.solbay.count = value.clone();
    Ok(())
  }
}

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
#[derive(Accounts)]
pub struct CloseSolbay<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub solbay: Account<'info, Solbay>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub solbay: Account<'info, Solbay>,
}

#[account]
#[derive(InitSpace)]
pub struct Solbay {
  count: u8,
}
