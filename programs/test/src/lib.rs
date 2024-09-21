use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: UserData) -> Result<()> {
        let user_data = &mut ctx.accounts.user_data;
        user_data.first_name = data.first_name;
        user_data.last_name = data.last_name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 40 + 40)]
    pub user_data: Account<'info, UserData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
}
