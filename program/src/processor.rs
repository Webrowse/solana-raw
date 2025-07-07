

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instruction::VaultInstruction;

pub fn process_instruction( 
    _program_id: &Pubkey,
    _account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::unpack(instruction_data)?;

    match instruction {
        VaultInstruction::Initialize => {
            msg!("Instruction: Initialise");
        },
        VaultInstruction::Ping => {
            msg!("Instruction: Ping");
        }
    }
    Ok(())
}