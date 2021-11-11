use anchor_lang::prelude::*;

declare_id!("CqU8bUXfDFS5fn1W2uJ59Rnpmud5ck8CoWhiskzLJhou");

#[program]
pub mod course_2 {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
