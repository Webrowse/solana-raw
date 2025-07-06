

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

use crate::processor::process_instruction;

entrypoint!(process_instruction_entry);

fn process_instruction_entry(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    process_instruction(program_id, account, instruction_data)
}