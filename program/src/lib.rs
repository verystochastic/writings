use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod instruction;
pub mod processor;
pub mod state;

use crate::processor::Processor;

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

solana_program::declare_id!("BLo9qGtPapnVMx8LqgBH4QSqr3RpxTwLbJfQwHNZR1cS"); 