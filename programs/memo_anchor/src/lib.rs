use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod memo_anchor {
    use super::*;
    pub fn build_memo(ctx: Context<Initialize>, input:u8) -> Result<()> {

        msg!("Initializing");
        // let account = &mut ctx.accounts.my_account;
        // let account_info_iter = account.iter();
        
        let account_info_iter = [1,2,3,4];

        let mut missing_required_signature = false;
        for account_info in account_info_iter.iter() {
            if let Some(address) = account_info.address() {
                msg!("Signed by: {:?}", address);
            }
            else{
                missing_required_signature = true;
            }
        }
        if missing_required_signature{
            return Err(Error::MissingRequiredSignature);
        }

        let memo = input.map_err(|err| {
            msg!("Invalid UTF-8, from bytes: {:?}", err);
        })?;

        msg!("Memo (len {}): {:?}", memo.len(), memo);


        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyMemo>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyMemo {
    pub input: u8,
}
