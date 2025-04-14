use anchor_lang::prelude::*;
use crate::state::VaultState;

/*
what are the accounts that we would be needing for this initialize ins?
- we need the user to sign, so users account as a signer
- next, we need some account to store the vault state such as bumps for the PDA or
in case of multiple vaults support for a single user, we can store the state such as
vault_type etc. this account can be a pda derived from users address and a seed.
- we need a vault account to store the sol that we are transferring and depositing. this
can be a system account derived from vault_state account and another seed.
- system_program account required because the init constraint invokes 
the System Program to create the account.

something to think of => why can't we just derive the vault account from user's key
and store both the state such as bump and also hold the lamports of the user in the same
vault account, this would eliminate the need of extra vault state account?
this would fail during the withdrawal because we aren't allowed to transfer sol
directly from pda that contains data too. The reason behind this is that - when we 
are creating the pda account to hold the data, the ownership is transferred from
system program to our program id. But, in withdraw, for transferring sol, we call the transfer
instruction of system program on the pda which would obviously fail as we know that only the program
that owns the account can reduce sol in it. The system program is no longer the owner, so it cant reduce
the sol/transfer sol from that account. That's why we need two seperate accounts, one to hold state data
and the other to hold amount lamports.

*/
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault_state", user.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault_state.set_inner(VaultState { 
            vault_state_bump: bumps.vault_state,
            vault_bump: bumps.vault
        });
        Ok(())
    }
}