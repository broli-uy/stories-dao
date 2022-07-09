use solana_program::msg;
use solana_program::account_info::next_account_info;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_pack::Pack;
use spl_token::state::Mint as TokenMint;
// use solana_program::program_error::ProgramError;

pub fn example(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  Ok(())
}
