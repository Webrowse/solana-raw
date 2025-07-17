
use solana_program::program_error::ProgramError;

pub enum VaultInstruction {
    Initialize,
    Increment,
}

impl VaultInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, _rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => Ok(VaultInstruction::Initialize),
            1 => Ok(VaultInstruction::Increment),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}