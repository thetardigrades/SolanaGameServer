// use crate::{error::TokenError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

// TODO: Do I need this?
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}


/// Processes an [Instruction](enum.Instruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
  let instruction = TokenInstruction::unpack(input)?;

  match instruction {
      TokenInstruction::InitializeMint {
          decimals,
          mint_authority,
          freeze_authority,
      } => {
          msg!("Instruction: InitializeMint");
          Self::process_initialize_mint(accounts, decimals, mint_authority, freeze_authority)
      }
  }
}

// Creates a new HeartToken with the specified account.
fn create_heart_token(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  // TODO: Fees - portion to verifier.
  // TODO: Uniqueness check - verifier incentitivzed to ensure uniqueness?
  // TODO: Incentivize user to make their unencrypted data public?

  // Input: User wallet/account signature, verifier signature, verified credential proving identity.
  // Side-effects:
  //  1) Account checked if it has any other HeartTokens associated with it (Bloom filter?)
  //  2) If account already has HeartToken, return Error & point to claim_heart_token
  //  3) Else, create HeartToken

  // Check if account has existing HeartToken - return Error if so.
  // HeartToken 
  solana_program::declare_id!("So11111111111111111111111111111111111111112");
}

HeartToken data:
HeartToken ID, current account (pub/priv key)
Public data (e.g. profile)
Public, verify-only data (e.g. phone number encrypted with public key)
Public claims/credentials - e.g. Google indicating employment


// HeartToken provides auth & access log?

CRUD(auth, heart_token_target, data_key, data_filter)
  // CRUD profile data; only 

// HeartToken.com functionality
Query(SQL over-data?)
  

get_data(hearttoken, type) {
  // Input: HeartToken ID (or username)
  

}

fn sign_contract(addr*, contract, addr_ht_requirements) {
  for addr in addrs:

}

fn attest_identity(self, target_account, {name, heart_token_id}, verify_cost) {

  transfer(target_account, self, verify_cost)
}

fn get_data(heart_token, schema, access_token) {

}

fn get_heart_token(self, addr) {

}


// heart_token_id is account_id. If account_id changes, you can transfer the name
// to it and it would disable the old account's heart_token access

