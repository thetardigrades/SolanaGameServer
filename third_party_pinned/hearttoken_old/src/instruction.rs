use crate::error::{EscrowError::InvalidInstruction, HeartTokenError};
use crate::state::{CredentialType, HeartToken, VerifiedCredential, MAX_REQUIRED_CREDENTIALS};
use solana_program::program_error::ProgramError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use std::convert::TryInto;
use std::mem::size_of;

use std::iter::Chain;
use std::slice::Iter;


pub enum HeartTokenInstruction {
    /// Creates a HeartToken.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the owner of the HeartToken.
    /// 1. `[writable]` The HeartToken account, it will hold all necessary info about the HT.
    /// 2. `[signer]` The HeartToken minter
    /// 3. `[]` The rent sysvar
    CreateHeartToken {
        // TODO: Make HeartTokenOwner a credential?
        // heart_token_owner: Pubkey, // Wallet of holder.
    // verified_credentials: Vec<VerifiedCredential>
    },

    /// Creates a new Claim type.
    ///
    ///
    /// Accounts expected:
    ///
    /// 3. `[]` The rent sysvar
    CreateClaimType {
        claim_check_program_id: Pubkey,
        check_program_instruction_id: u8
        // data_hash: [u8; DATA_HASH_SIZE]
        // data aggregation? E.g. on_success_program_id
        //
    },
    IssueClaim {
        claim_type_id: Pubkey,
        subject: Pubkey,
        // data_hash
    },
    // RevokeClaim {
    //     claim_type_id: Pubkey,
    //     subject: Pubkey,
    // },

    /// Creates a new Claim type.
    ///
    ///
    /// Accounts expected:
    ///
    /// 3. `[]` The rent sysvar
    CreateSimpleClaimCheck {
        subject_required_credentials: [Pubkey; MAX_REQUIRED_CREDENTIALS],
        issuer_required_credentials:  [Pubkey; MAX_REQUIRED_CREDENTIALS]
    },
    ExecuteSimpleClaimCheck {
        claim_type_id: Pubkey
    }
    // CreateHeartTokenContract {
    // }
    // SignWithHeartTokens {
    // }
    // RecoverHeartToken {
    //     heart_token_account: Pubkey, // The account/ID of the HeartToken
    //     heart_token_owner: Pubkey,  // Wallet of holder.
    //     verified_credentials: Vec<VerifiedCredential>
    // },
    // AddCredentialsToHeartToken {
    //     heart_token_owner: Pubkey,
    //     verified_credentials: Vec<VerifiedCredential>
    // },
    // SignWithHeartTokens {
    //     instructions: Vec<Instruction>,
    //     // constraints: Vec<Constraint>
    // },
}

impl HeartTokenInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => {
                // let (pubkey, _rest) = Self::unpack_pubkey(rest)?;
                Self::CreateHeartToken {
                    // heart_token_owner: pubkey,
                }
            }
            1 => {
                let (claim_check_program_id, rest) = Self::unpack_pubkey(rest)?;
                let check_program_instruction_id = *rest.get(0).unwrap();
                Self::CreateClaimType {
                    claim_check_program_id,
                    check_program_instruction_id
                }
            }
            2 => {
                let (claim_type_id, rest) = Self::unpack_pubkey(rest)?;
                let (subject, _rest) = Self::unpack_pubkey(rest)?;
                Self::IssueClaim {
                    claim_type_id,
                    subject, // heart_token_owner: pubkey,
                }
            }
            3 => {
                if rest.len() == 2 * 32 * MAX_REQUIRED_CREDENTIALS  {
                    let subject_required_credentials = [Pubkey::new_from_array([0u8; 32]); MAX_REQUIRED_CREDENTIALS];
                    let issuer_required_credentials = [Pubkey::new_from_array([0u8; 32]); MAX_REQUIRED_CREDENTIALS];
                    Self::CreateSimpleClaimCheck {
                        subject_required_credentials,
                        issuer_required_credentials
                    }
                } else {
                    return Err(HeartTokenError::InvalidInstruction.into());
                }
            }
            4 => {
                let (pubkey, _rest) = Self::unpack_pubkey(rest)?;
                Self::ExecuteSimpleClaimCheck {
                    claim_type_id: pubkey,
                }
            }
        
            _ => return Err(HeartTokenError::InvalidInstruction.into()),
        })
    }
    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(HeartTokenError::InvalidInstruction.into())
        }
    }

    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::CreateHeartToken {} => {
                buf.push(0);
                // buf.extend_from_slice(heart_token_owner.as_ref());
            }
            &Self::CreateClaimType {
                ref claim_check_program_id,
                check_program_instruction_id
            } => {
                buf.push(1);
                buf.extend_from_slice(claim_check_program_id.as_ref());
                buf.push(check_program_instruction_id);
            }
            &Self::IssueClaim {
                ref claim_type_id,
                ref subject,
            } => {
                buf.push(2);
                buf.extend_from_slice(claim_type_id.as_ref());
                buf.extend_from_slice(subject.as_ref());
            }
            &Self::CreateSimpleClaimCheck {
                ref subject_required_credentials,
                ref issuer_required_credentials,
            } => {
                buf.push(3);
                for key in subject_required_credentials.iter().chain(issuer_required_credentials.iter()) {
                    buf.extend_from_slice(key.as_ref());
                }
            }
            &Self::ExecuteSimpleClaimCheck {
                ref claim_type_id,
            } => {
                buf.push(4);
                buf.extend_from_slice(claim_type_id.as_ref());
            }
        }
        buf
    }

    pub fn create_claim_type(
        heart_token_program_id: &Pubkey,
        claim_check_program_id: &Pubkey,
        check_program_instruction_id: u8
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            // AccountMeta::new_readonly(*claim_check_program_id, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];
        let data = HeartTokenInstruction::CreateClaimType {
            claim_check_program_id: *claim_check_program_id,
            check_program_instruction_id
        }
        .pack();
        Ok(Instruction {
            program_id: *heart_token_program_id,
            accounts,
            data,
        })
    }

    pub fn create_issue_claim(
        heart_token_program_id: &Pubkey,
        claim_type_id: &Pubkey,
        subject: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            // AccountMeta::new_readonly(*claim_check_program_id, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];
        let data = HeartTokenInstruction::IssueClaim {
            claim_type_id: *claim_type_id,
            subject: *subject,
        }
        .pack();
        Ok(Instruction {
            program_id: *heart_token_program_id,
            accounts,
            data,
        })
    }

    pub fn create_heart_token(
        heart_token_program_id: &Pubkey,
        heart_token_owner: &Pubkey,
        heart_token_account: &Pubkey,
        heart_token_minter: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new_readonly(*heart_token_owner, true),
            AccountMeta::new(*heart_token_account, false),
            AccountMeta::new_readonly(*heart_token_minter, true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];
        let data = HeartTokenInstruction::CreateHeartToken {}.pack();
        Ok(Instruction {
            program_id: *heart_token_program_id,
            accounts,
            data,
        })
    }
}

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The SPL token program for transfer authority.
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64,
    },
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitEscrow { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        }
        buf
    }

    pub fn initialize_escrow(
        escrow_program_id: &Pubkey,
        account: &Pubkey,
        temp_escrow_account: &Pubkey,
        receiver_account: &Pubkey,
        escrow_account: &Pubkey,
        token_program_id: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new_readonly(*account, true),
            AccountMeta::new(*temp_escrow_account, false),
            AccountMeta::new_readonly(*receiver_account, false),
            AccountMeta::new(*escrow_account, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(*token_program_id, false),
        ];
        let data = EscrowInstruction::InitEscrow { amount: amount }.pack();
        Ok(Instruction {
            program_id: *escrow_program_id,
            accounts,
            data,
        })
    }
}
