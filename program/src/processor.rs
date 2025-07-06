

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub fn process_instruction( 
    _program_id: &Pubkey,
    _account: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!(r#"Solana Raw Hello "{}""#);
    Ok(())
}