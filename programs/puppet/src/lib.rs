use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("HCAzLEY2FbjzY6D2QW3fh9pWaMecmAGWM63SzYfaRpVM");

#[program]
pub mod puppet {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn set_announcement(ctx: Context<SetAnnouncement>, new_text: Vec<u8>) -> ProgramResult {
        let announcement = &mut ctx.accounts.announcement;
        let text = from_utf8(&new_text).map_err(|err| {
            msg!("Invalid utf-8 encoding from : {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(text);
        announcement.text = new_text;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 256)]
    pub puppet: Account<'info, Announcement>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAnnouncement<'info> {
    #[account(mut)]
    pub announcement: Account<'info, Announcement>,
}

#[account]
pub struct Announcement {
    pub text: Vec<u8>,
}
