use thiserror::Error;

use solana_program::program_error::ProgramError;


#[derive(Error, Debug, Copy, Clone)]
pub enum HeartTokenError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Invalid minter provided")]
    InvalidMinter,
    #[error("Not Implemented")]
    NotImplemented,
}

impl From<HeartTokenError> for ProgramError {
  fn from(e: HeartTokenError) -> Self {
      ProgramError::Custom(e as u32)
  }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
      ProgramError::Custom(e as u32)
  }
}
