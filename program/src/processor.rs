use solana_program::{
    account_info::{self, next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey
};

use crate::instruction::VaultInstruction;
use crate::state::VaultState;
use borsh::{BorshDeserialize, BorshSerialize};

pub fn process_instruction(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::unpack(instruction_data)?;

    match instruction {
        VaultInstruction::Initialize => {
            msg!("Instruction: Initialise");
        }
        VaultInstruction::Ping => {
            msg!("Instruction: Ping");

            let account_info_iter = &mut account.iter();
            let state_account = next_account_info(account_info_iter)?;

            if program_id != state_account.owner {
                msg!("Program doesn't own the account");
                return Err(ProgramError::IncorrectProgramId);
            } 

            let mut state = VaultState::try_from_slice(&state_account.data.borrow())?;
            msg!("Current counter: {}", state.counter);

            state.counter += 1;
            msg!("Increased counter: {}", state.counter);

            state.serialize(&mut &mut state_account.data.borrow_mut()[..])?;
        }
    }
    Ok(())
}
