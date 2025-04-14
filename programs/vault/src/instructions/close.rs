use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct  CloseVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.vault_state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    // System Program is required here because because the CPI invokes the System Program for
    // transfer of sol
}

impl<'info> CloseVault<'info> {
    pub fn close_vault(&mut self) -> Result<()> {
        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new(
            program,
            accounts,
        ).with_signer(signer_seeds);
        transfer(cpi_ctx, self.vault.lamports())
    }
}