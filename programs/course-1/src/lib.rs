use anchor_lang::prelude::*;

declare_id!("9eyezNhnXYyEDYeUevEV1KEczoVCYgtVp1EVeas3CBYp");

#[program]
pub mod course_1 {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
