use anchor_lang::prelude::*;

declare_id!("YMzmnDGoxtfXB68r2rdrFWhnCykFyYUWEj62NkvTsbE");

#[program]
pub mod stackframeanchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, space = 8 + CounterData::INIT_SPACE, payer = payer)]
    pub counter: Box<Account<'info, CounterData>>,
    pub system_program: Program<'info, System>,
}

// 6389 CU
#[account]
#[derive(InitSpace)]
pub struct CounterData {
    count: u64,
    test: Pubkey,
    test1: u64,
    test2: u64,
    big_struct: BigStructNoZeroCopy,
    bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BigStructNoZeroCopy {
    test: Pubkey,
    test1: u64,
    test2: u64,
    test3: Pubkey,
    test4: u64,
    test5: u64,
    test6: Pubkey,
    pubkey_array: [Pubkey; 55],
}
