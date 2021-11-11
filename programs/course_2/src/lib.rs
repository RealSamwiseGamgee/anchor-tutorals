use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("CqU8bUXfDFS5fn1W2uJ59Rnpmud5ck8CoWhiskzLJhou");

#[program]
pub mod course_2 {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, new_text: Vec<u8>) -> ProgramResult {
        let announcement = &mut ctx.accounts.announcement;
        let text = from_utf8(&new_text).map_err(|err| {
            msg!("Invalid utf-8 encoding from : {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(text);
        announcement.text = new_text;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_text: Vec<u8>) -> ProgramResult {
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
    pub announcement: Account<'info, Announcement>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub announcement: Account<'info, Announcement>,
}

#[account]
pub struct Announcement {
    pub text: Vec<u8>,
}
