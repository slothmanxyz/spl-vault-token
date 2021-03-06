use anchor_lang::prelude::*;
use anchor_spl::token::{
  TokenAccount,
  Token,
};

declare_id!("MqUazenmqvR1S2T9aNh9eeKY1yWMy3KG8fQRWTXRb6L");

#[program]
pub mod vault_relayer {
  use super::*;

  pub fn relay_deposit<'info>(ctx: Context<'_, '_, '_, 'info, RelayDeposit<'info>>, amount: u64) -> Result<()> {
    //let owner = &ctx.accounts.owner;
    let remaining_accounts = ctx.remaining_accounts;

    spl_vault_token::cpi::deposit(
      CpiContext::new(
        remaining_accounts[3].to_account_info(), 
        spl_vault_token::cpi::accounts::PoolInteraction {
          owner: ctx.accounts.owner.to_account_info(),
          token_account: ctx.accounts.token_account.to_account_info(),
          vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
          vault_token_mint: remaining_accounts[0].to_account_info(),
          vault_info: remaining_accounts[1].to_account_info(),
          pool: remaining_accounts[2].to_account_info(),
          token_program: ctx.accounts.token_program.to_account_info(), // remaining_accounts[3].to_account_info(),
        }
      ), amount)?;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct RelayDeposit<'info>{
  pub owner: Signer<'info>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub vault_token_account: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
}
