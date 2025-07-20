use crate::instruction::VaultInstruction;
use crate::state::VaultState;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk_ids::system_program;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::unpack(instruction_data)?;

    match instruction {
        VaultInstruction::Initialize => {
            msg!("Instruction: Initialise");
            let account_info_iter = &mut accounts.iter();
            let state_account = next_account_info(account_info_iter)?;

            let _payer = next_account_info(account_info_iter)?;
            let _system_program_account = next_account_info(account_info_iter)?;

            if *program_id != *state_account.owner && system_program::id() != *state_account.owner {
                msg!("Program do not own the account");
                return Err(ProgramError::IncorrectProgramId);
            }

            let state = VaultState {
                initialized: true,
                counter: 0,
            };

            let mut data = state_account.data.borrow_mut();
            state.serialize(&mut &mut data[..])?;

            msg!(r#"Vault::initialted "success""#);
        }
        VaultInstruction::Increment => {
            msg!("Instruction: Increment");

            let account_info_iter = &mut accounts.iter();
            let state_account = next_account_info(account_info_iter)?;

            if *program_id != *state_account.owner {
                msg!("Program doesn't own the account");
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut state = VaultState::try_from_slice(&state_account.data.borrow())?;
            if !state.initialized {
                msg!("Vault was not initialized");
                return Err(ProgramError::UninitializedAccount);
            }

            state.counter += 1;
            msg!("current counter: {}", state.counter);

            state.serialize(&mut &mut state_account.data.borrow_mut()[..])?;
        }
    }
    Ok(())
}
