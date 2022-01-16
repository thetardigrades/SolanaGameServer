use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use num_enum::TryFromPrimitive;
use solana_program::{
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
};

pub const MAX_VC: usize = 10; // Arbitrary
pub const MAX_DATA: usize = 256; // Arbitrary
pub const MAX_REQUIRED_CREDENTIALS: usize = 8; // Arbitrary

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleClaimCheck {
  subject_required_credentials: [Pubkey; MAX_REQUIRED_CREDENTIALS],
  issuer_required_credentials: [Pubkey; MAX_REQUIRED_CREDENTIALS],
}

// impl SimpleClaimCheck {}

impl Sealed for SimpleClaimCheck {}

impl Pack for SimpleClaimCheck {
  const LEN: usize = 2 * MAX_REQUIRED_CREDENTIALS * 32;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, SimpleClaimCheck::LEN];
    let (subject_required_credentials_flat, issuer_required_credentials_flat) = array_refs![
      src,
      MAX_REQUIRED_CREDENTIALS * 32,
      MAX_REQUIRED_CREDENTIALS * 32
    ];
    let mut result = SimpleClaimCheck {
      subject_required_credentials: [Pubkey::new_from_array([0u8; 32]); MAX_REQUIRED_CREDENTIALS],
      issuer_required_credentials: [Pubkey::new_from_array([0u8; 32]); MAX_REQUIRED_CREDENTIALS],
    };
    for (src, dst) in subject_required_credentials_flat
      .chunks(32)
      .zip(result.subject_required_credentials.iter_mut())
    {
      *dst = Pubkey::new(src);
    }
    for (src, dst) in issuer_required_credentials_flat
      .chunks(32)
      .zip(result.issuer_required_credentials.iter_mut())
    {
      *dst = Pubkey::new(src);
    }
    Ok(result)
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, SimpleClaimCheck::LEN];
    let (subject_required_credentials_dst, issuer_required_credentials_dst) = mut_array_refs![
      dst,
      MAX_REQUIRED_CREDENTIALS * 32,
      MAX_REQUIRED_CREDENTIALS * 32
    ];

    let SimpleClaimCheck {
      subject_required_credentials,
      issuer_required_credentials,
    } = self;

    for (i, src) in subject_required_credentials.iter().enumerate() {
      let dst_array = array_mut_ref![subject_required_credentials_dst, 32 * i, 32];
      dst_array.copy_from_slice(src.as_ref());
    }
    for (i, src) in issuer_required_credentials.iter().enumerate() {
      let dst_array = array_mut_ref![issuer_required_credentials_dst, 32 * i, 32];
      dst_array.copy_from_slice(src.as_ref());
    }
  }
}

// Note: Taken from AccountState:
// https://docs.rs/spl-token/3.1.0/src/spl_token/state.rs.html#177
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
pub enum CredentialType {
  HeartTokenMinter,
  DriversLicense, // TODO:
                  // is_uniquely_identifying
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VerifiedCredential {
  pub type_: CredentialType,
  pub verifier_pubkey: Pubkey,
  // pub timestamp_nanos: u64,
  // pub data: [u8; MAX_DATA]
  // TODO: Data location & hash.
}

impl VerifiedCredential {
  pub fn empty() -> VerifiedCredential {
    VerifiedCredential {
      type_: CredentialType::HeartTokenMinter,
      verifier_pubkey: Pubkey::new_from_array([0u8; 32]),
    }
  }
}

impl Sealed for VerifiedCredential {}

impl Pack for VerifiedCredential {
  const LEN: usize = 1 + 32;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, VerifiedCredential::LEN];
    let (type_, verifier_pubkey) = array_refs![src, 1, 32];
    Ok(VerifiedCredential {
      type_: CredentialType::try_from_primitive(type_[0])
        .or(Err(ProgramError::InvalidAccountData))?,
      verifier_pubkey: Pubkey::new_from_array(*verifier_pubkey),
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, VerifiedCredential::LEN];
    let (type_dst, verifier_pubkey_dst) = mut_array_refs![dst, 1, 32];

    let VerifiedCredential {
      type_,
      verifier_pubkey,
    } = self;

    type_dst[0] = *type_ as u8;
    verifier_pubkey_dst.copy_from_slice(verifier_pubkey.as_ref());
  }
}

pub struct HeartToken {
  // ID is account ID.
  pub is_initialized: bool,
  pub owner_pubkey: Pubkey,
  pub verified_credentials: [VerifiedCredential; MAX_VC],
}

impl Sealed for HeartToken {}

impl IsInitialized for HeartToken {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Pack for HeartToken {
  const LEN: usize = 1 + 32 + MAX_VC * VerifiedCredential::LEN;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, HeartToken::LEN];
    let (is_initialized, owner_pubkey, verified_credentials_flat) =
      array_refs![src, 1, 32, MAX_VC * VerifiedCredential::LEN];
    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };
    let mut result = HeartToken {
      is_initialized,
      owner_pubkey: Pubkey::new_from_array(*owner_pubkey),
      verified_credentials: [VerifiedCredential::empty(); MAX_VC],
    };
    for (src, dst) in verified_credentials_flat
      .chunks(VerifiedCredential::LEN)
      .zip(result.verified_credentials.iter_mut())
    {
      *dst = VerifiedCredential::unpack_from_slice(src).unwrap();
    }
    Ok(result)
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, HeartToken::LEN];
    let (is_initialized_dst, owner_pubkey_dst, verified_credentials_flat_dst) =
      mut_array_refs![dst, 1, 32, MAX_VC * VerifiedCredential::LEN];

    let HeartToken {
      is_initialized,
      owner_pubkey,
      verified_credentials,
    } = self;

    is_initialized_dst[0] = *is_initialized as u8;
    owner_pubkey_dst.copy_from_slice(owner_pubkey.as_ref());
    for (i, src) in verified_credentials.iter().enumerate() {
      let dst_array = array_mut_ref![
        verified_credentials_flat_dst,
        VerifiedCredential::LEN * i,
        VerifiedCredential::LEN
      ];
      src.pack_into_slice(dst_array);
    }
  }
}

pub struct Escrow {
  pub is_initialized: bool,
  pub initializer_pubkey: Pubkey,
  pub temp_token_account_pubkey: Pubkey,
  pub initializer_token_to_receive_account_pubkey: Pubkey,
  pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Pack for Escrow {
  const LEN: usize = 105;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Escrow::LEN];
    let (
      is_initialized,
      initializer_pubkey,
      temp_token_account_pubkey,
      initializer_token_to_receive_account_pubkey,
      expected_amount,
    ) = array_refs![src, 1, 32, 32, 32, 8];
    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };

    Ok(Escrow {
      is_initialized,
      initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
      temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
      initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
        *initializer_token_to_receive_account_pubkey,
      ),
      expected_amount: u64::from_le_bytes(*expected_amount),
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, Escrow::LEN];
    let (
      is_initialized_dst,
      initializer_pubkey_dst,
      temp_token_account_pubkey_dst,
      initializer_token_to_receive_account_pubkey_dst,
      expected_amount_dst,
    ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

    let Escrow {
      is_initialized,
      initializer_pubkey,
      temp_token_account_pubkey,
      initializer_token_to_receive_account_pubkey,
      expected_amount,
    } = self;

    is_initialized_dst[0] = *is_initialized as u8;
    initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
    temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
    initializer_token_to_receive_account_pubkey_dst
      .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
    *expected_amount_dst = expected_amount.to_le_bytes();
  }
}
