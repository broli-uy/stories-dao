use solana_program::entrypoint;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;

// use crate::functions::...;

fn process_instruction(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  match instruction_data[0] {
    // b'1' => { ...(&_program_id, &accounts, &instruction_data) }
    _ => { Err(ProgramError::InvalidInstructionData) }
  }
}

entrypoint!(process_instruction);
