use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("b8zyWpt7G28SELnjvQEKkotpDjUEPNafhWd16tGAr1J");

#[program]
pub mod course_3 {
    use super::*;
    pub fn create(ctx: Context<Create>, authority: Pubkey, new_text: Vec<u8>) -> ProgramResult {
        let announcement = &mut ctx.accounts.announcement;
        let text = from_utf8(&new_text).map_err(|err| {
            msg!("Invalid utf-8 encoding from : {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(text);
        announcement.authority = authority;
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
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 32 + 256)]
    pub announcement: Account<'info, Announcement>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority)]
    pub announcement: Account<'info, Announcement>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Announcement {
    pub authority: Pubkey,
    pub text: Vec<u8>,
}
