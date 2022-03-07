use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod memo_anchor {
    use super::*;
    use std::str;
    pub fn build_memo(ctx: Context<BuildMemo>, input:Vec<u8>) -> Result<()> {

        msg!("Initializing");

        let accounts = ctx.remaining_accounts;

        let mut missing_required_signature = false;
        for account_info in accounts.iter() {
            if let Some(address) = account_info.signer_key() {
                msg!("Signed by: {:?}", address);
            }
            else{
                missing_required_signature = true;
            }
        }
        if missing_required_signature{
            return err!(MyError::MissingRequiredSignature);
        }


        let unwrapped_input = str::from_utf8(&input).map_err(|err| {
            msg!("Invalid UTF-8, from bytes: {:?}", err);
        });

    //     let memo = str::from_utf8(input).map_err(|err| {
    //     msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
    //     ProgramError::InvalidInstructionData
    // })?;
    msg!("Memo (len {}): {:?}", unwrapped_input.unwrap().len() , unwrapped_input);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct BuildMemo<> {

}

#[error_code]
pub enum MyError {
    #[msg("Missing required signature")]
    MissingRequiredSignature,
    #[msg("Invalid Instruction Data")]
    InvalidInstructionData
}


