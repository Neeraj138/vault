use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
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

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        /*
        The process of implementing a CPI is the same as that of creating a 
        instruction where you must specify: 
        The program ID of the program being called
        The accounts required by the instruction
        Any instruction data required as arguments
        */
        let program = self.system_program.to_account_info();
        let accounts_required_by_ins = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };
        let cpi_ctx =  CpiContext::new(program, accounts_required_by_ins);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(program, accounts).with_signer(signer_seeds);
        transfer(cpi_ctx, amount)
    }
}