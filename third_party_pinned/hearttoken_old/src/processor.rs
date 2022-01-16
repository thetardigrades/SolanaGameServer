use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program::invoke,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack},
  pubkey::Pubkey,
  sysvar::{rent::Rent, Sysvar},
};

use std::str::FromStr;

use crate::{
  error::{EscrowError, HeartTokenError},
  instruction::{EscrowInstruction, HeartTokenInstruction},
  state::{CredentialType, Escrow, HeartToken},
};

pub const GOD_PUBKEY_STR: &str = "5cjmBetNkWYa2ZKZTsTMreNZQNSpwyhDrTVsynJVKZ9C";// "5gREDw2KxWKTceSTCbtQSG32aSnHrxPUUNo1PERZBMTq";

pub struct Processor;
impl Processor {
  // HeartToken Process
  pub fn process_heart_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = HeartTokenInstruction::unpack(instruction_data)?;

    match instruction {
      HeartTokenInstruction::CreateHeartToken {} => {
        msg!("Instruction: CreateHeartToken");
        Self::process_create_heart_token(accounts, program_id)
      }
      HeartTokenInstruction::CreateClaimType { claim_check_program_id, check_program_instruction_id } => {
        msg!("Instruction: CreateClaimType");
        return Err(HeartTokenError::NotImplemented.into());
      }
      HeartTokenInstruction::IssueClaim { claim_type_id, subject } => {
        msg!("Instruction: IssueClaim");
        return Err(HeartTokenError::NotImplemented.into());
      }
      HeartTokenInstruction::CreateSimpleClaimCheck {subject_required_credentials, issuer_required_credentials} => {
        msg!("Instruction: CreateSimpleClaimCheck");
        return Err(HeartTokenError::NotImplemented.into());
      }
      HeartTokenInstruction::ExecuteSimpleClaimCheck {claim_type_id} => {
        msg!("Instruction: ExecuteSimpleClaimCheck");
        return Err(HeartTokenError::NotImplemented.into());
      }
    }
  }

  // fn process_create_simple_claim_check(accounts: &[AccountInfo],
  //   subject_required_credentials: &[Pubkey],
  //   issuer_required_credentials:  &[Pubkey],
  //   program_id: &Pubkey) {
  //   let account_info_iter = &mut accounts.iter();
  //   let owner = next_account_info(account_info_iter)?;
  //   if account.owner != program_id {
  //     msg!("Invalid owner!");
  //     return Err(HeartTokenError::InvalidMinter.into());
  //   }
  // }

  fn check_minter(account: &AccountInfo, program_id: &Pubkey) -> ProgramResult {
    msg!("Account: {}", account.key.to_string());
    if account.key.to_string() == GOD_PUBKEY_STR {
      return Ok(());
    }
    // Ensure minter account is owned by HeartToken program.
    if account.owner != program_id {
      msg!("Invalid owner!");
      return Err(HeartTokenError::InvalidMinter.into());
    }
    // Check that minter has Minter credential.
    let account_info = HeartToken::unpack_unchecked(&account.data.borrow())?;
    if !account_info
      .verified_credentials
      .iter()
      .any(|&vc| vc.type_ == CredentialType::HeartTokenMinter)
    {
      msg!("No Minter VC!");
      return Err(HeartTokenError::InvalidMinter.into());
    }
    Ok(())
  }

  fn process_create_heart_token(
    accounts: &[AccountInfo],
    // heart_token_owner: Pubkey,
    program_id: &Pubkey,
  ) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let owner = next_account_info(account_info_iter)?;
    if !owner.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    let heart_token_account = next_account_info(account_info_iter)?;
    let heart_token_minter = next_account_info(account_info_iter)?;
    msg!("pub key {}", *heart_token_minter.key);

    Processor::check_minter(heart_token_minter, program_id)?;

    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    if !rent.is_exempt(
      heart_token_account.lamports(),
      heart_token_account.data_len(),
    ) {
      return Err(HeartTokenError::NotRentExempt.into());
    }

    let mut heart_token_info = HeartToken::unpack_unchecked(&heart_token_account.data.borrow())?;
    if heart_token_info.is_initialized() {
      return Err(ProgramError::AccountAlreadyInitialized);
    }
    heart_token_info.is_initialized = true;
    heart_token_info.owner_pubkey = *owner.key;

    // Write the heart_token info to the actual account.
    HeartToken::pack(heart_token_info, &mut heart_token_account.data.borrow_mut())?;

    Ok(())
  }

  // Escrow Process
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {
      EscrowInstruction::InitEscrow { amount } => {
        msg!("Instruction: InitEscrow");
        Self::process_init_escrow(accounts, amount, program_id)
      }
    }
  }

  fn process_init_escrow(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
  ) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    if !initializer.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    // No need to check owner bc function will fail.
    let temp_token_account = next_account_info(account_info_iter)?;

    // Need to check owner to insure not entering invalid state.
    let token_to_receive_account = next_account_info(account_info_iter)?;
    if *token_to_receive_account.owner != spl_token::id() {
      return Err(ProgramError::IncorrectProgramId);
    }

    let escrow_account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
      return Err(EscrowError::NotRentExempt.into());
    }

    let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
    if escrow_info.is_initialized() {
      return Err(ProgramError::AccountAlreadyInitialized);
    }

    escrow_info.is_initialized = true;
    escrow_info.initializer_pubkey = *initializer.key;
    escrow_info.temp_token_account_pubkey = *temp_token_account.key;
    escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
    escrow_info.expected_amount = amount;

    // Write the escrow info to the actual account.
    Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;

    // Transfer ownership of the temp account to this program via a derived address.
    let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    let token_program = next_account_info(account_info_iter)?;
    let owner_change_ix = spl_token::instruction::set_authority(
      token_program.key,
      temp_token_account.key,
      Some(&pda),
      spl_token::instruction::AuthorityType::AccountOwner,
      initializer.key,
      &[&initializer.key],
    )?;

    msg!("Calling the token program to transfer token account ownership...");
    msg!(
      "Token program: {}. Transferring ownership {} -> {}",
      token_program.key,
      initializer.key,
      pda
    );
    invoke(
      &owner_change_ix,
      &[
        temp_token_account.clone(),
        initializer.clone(),
        token_program.clone(),
      ],
    )?;
    msg!("Called");
    Ok(())
  }
}
