use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetAnnouncement;
use puppet::program::Puppet;
use puppet::{self, Announcement};

declare_id!("DXk9DpBdz6uEfxXPg8TNh8k4URsASsHktYDk5aJfJoah");

#[program]
pub mod puppet_master {
    use super::*;
    pub fn pull_string(ctx: Context<PullStrings>, new_text: Vec<u8>) -> ProgramResult {
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts = SetAnnouncement {
            announcement: ctx.accounts.puppet.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_announcement(cpi_ctx, new_text)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Announcement>,
    pub puppet_program: Program<'info, Puppet>,
}
