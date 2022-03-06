use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod memo_anchor {
    use super::*;
    use std::str;
    pub fn build_memo(ctx: Context<Initialize>, input:&[u8]) -> Result<()> {

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
            return err!(MyError::MissingRequiredSignature);
        }

        // let memo = input.map_err(|err| {
        //     msg!("Invalid UTF-8, from bytes: {:?}", err);
        // })?;

        let memo = str::from_utf8(&input).map_err(|err| {
            msg!("Invalid UTF-8, from bytes: {:?}", err);
        });

        msg!("Memo: {:?}", memo);

        // msg!("Memo (len {}): {:?}", memo.len(), memo);
        


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

#[error_code]
pub enum MyError {
    #[msg("Missing required signature")]
    MissingRequiredSignature
}

#[account]
pub struct MyMemo {
    pub input: u8,
}
