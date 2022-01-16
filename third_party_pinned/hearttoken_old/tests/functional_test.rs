#![cfg(feature = "test-bpf")]

use {
  assert_matches::*,
  hearttoken::{
    entrypoint::process_instruction,
    instruction::{EscrowInstruction, HeartTokenInstruction},
  },
  solana_program::{
    borsh::get_packed_len,
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    msg,
    rent::Rent,
    system_instruction,
    sysvar::{self},
  },
  solana_program_test::{processor, ProgramTest},
  solana_sdk::signature::Keypair,
  solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
  spl_token::{processor::Processor, state::AccountState},
  // c::state::Mint,
  std::str::FromStr,
};
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

// #[tokio::test]
// async fn test_sysvar() {
//   let program_id = Pubkey::from_str(&"Sysvar1111111111111111111111111111111111111").unwrap();
//   let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
//     "spl_example_sysvar",
//     program_id,
//     processor!(process_instruction),
//   )
//   .start()
//   .await;

//   let mut transaction = Transaction::new_with_payer(
//     &[Instruction::new_with_bincode(
//       program_id,
//       &(),
//       vec![
//         AccountMeta::new(sysvar::clock::id(), false),
//         AccountMeta::new(sysvar::rent::id(), false),
//       ],
//     )],
//     Some(&payer.pubkey()),
//   );
//   transaction.sign(&[&payer], recent_blockhash);
//   banks_client.process_transaction(transaction).await.unwrap();
// }

trait AddPacked {
  fn add_packable_account<T: Pack>(
    &mut self,
    pubkey: Pubkey,
    amount: u64,
    data: &T,
    owner: &Pubkey,
  );
}

impl AddPacked for ProgramTest {
  fn add_packable_account<T: Pack>(
    &mut self,
    pubkey: Pubkey,
    amount: u64,
    data: &T,
    owner: &Pubkey,
  ) {
    let mut account = Account::new(amount, T::get_packed_len(), owner);
    data.pack_into_slice(&mut account.data);
    self.add_account(pubkey, account);
  }
}

// fn program_test() -> ProgramTest {
//   ProgramTest::new("spl_record", id(), processor!(process_instruction))
// }

// #[tokio::test]
// async fn test_create_heart_token() {
//   let account_alice = Keypair::new();
//   let account_heart_token = Keypair::new();
//   let mut program_test = ProgramTest::new(
//     "heart_token_test",
//     hearttoken::id(),
//     processor!(hearttoken::processor::Processor::process_heart_token),
//   );

//   // Start the test client
//   let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//   // Create HeartToken.
//   let account_space = spl_token::state::Mint::LEN;
//   let mut transaction = Transaction::new_with_payer(
//     &[
//       system_instruction::create_account(
//         &payer.pubkey(),
//         &account_heart_token.pubkey(),
//         1.max(Rent::default().minimum_balance(hearttoken::state::HeartToken::LEN)),
//         hearttoken::state::HeartToken::LEN as u64,
//         &hearttoken::id(),
//       ),
//       HeartTokenInstruction::create_heart_token(
//         &hearttoken::id(),
//         &account_alice.pubkey(),
//         &account_heart_token.pubkey()
//       )
//       .unwrap(),
//     ],
//     Some(&payer.pubkey()),
//   );
//   transaction.sign(&[&payer, &account_alice, &account_heart_token], recent_blockhash);
//   // Create mint:
//   assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
// }

#[tokio::test]
async fn test_create_heart_minter() {
  let account_alice = Keypair::new();
  let account_heart_token = Keypair::new();
  // let heart_token_minter = Keypair::new();
  let keypair: [u8;64] = [
    107, 254, 121, 199, 233, 104, 91, 98, 219, 230, 11, 238, 73, 88, 242, 134, 198, 227, 13, 235,
    0, 64, 96, 208, 124, 152, 133, 96, 65, 88, 149, 96, 68, 150, 109, 75, 78, 72, 134, 74, 26, 54,
    152, 10, 233, 15, 48, 202, 174, 83, 206, 230, 45, 171, 29, 138, 3, 221, 137, 56, 228, 100, 153,
    203,
  ];
  let heart_token_minter = Keypair::from_bytes(&keypair).unwrap();
  
  let mut program_test = ProgramTest::new(
    "heart_token_test",
    hearttoken::id(),
    processor!(hearttoken::processor::Processor::process_heart_token),
  );

  // Start the test client
  let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
  // Create HeartToken.
  let account_space = spl_token::state::Mint::LEN;
  let mut transaction = Transaction::new_with_payer(
    &[
      system_instruction::create_account(
        &payer.pubkey(),
        &account_heart_token.pubkey(),
        1.max(Rent::default().minimum_balance(hearttoken::state::HeartToken::LEN)),
        hearttoken::state::HeartToken::LEN as u64,
        &hearttoken::id(),
      ),
      HeartTokenInstruction::create_heart_token(
        &hearttoken::id(),
        &account_alice.pubkey(),
        &account_heart_token.pubkey(),
        &heart_token_minter.pubkey()
      )
      .unwrap(),
    ],
    Some(&payer.pubkey()),
  );
  transaction.sign(
    &[&payer, &account_alice, &account_heart_token, &heart_token_minter],
    recent_blockhash,
  );
  // Create mint:
  assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}

// Based on Record functional test: https://github.com/solana-labs/solana-program-library/blob/2b3f71ead5b81f4ea4a2fd3e4fe9583a6e39b6a4/record/program/tests/functional.rs
// Unisqap example test https://github.com/dzmitry-lahoda/solana-uniswap-example/blob/a8f108adefe8fa61a947d408a5ce0064b1d8c2df/tests/tests.rs
// DO NOT COMMIT XXX: #[tokio::test]
async fn test_token() {
  // Create a SPL token
  // Create a main token account for Alice
  // Create temporary token account for Alice
  // let hearttoken::id() = Pubkey::new_unique();
  // TODO: Make authority derived from program?
  let authority = Keypair::new();
  let seed = "token";
  let mint_a = Keypair::new();
  let account_alice = Keypair::new();
  let account_alice_temp = Keypair::new();
  let account_escrow_state = Keypair::new();

  // let account = Pubkey::create_with_seed(&authority.pubkey(), seed, &spl_token::id()).unwrap();
  let mut program_test = ProgramTest::new(
    "token_test",
    spl_token::id(),
    processor!(Processor::process),
  );
  program_test.add_program(
    "escrow_test",
    hearttoken::id(),
    processor!(hearttoken::processor::Processor::process),
  );

  // Start the test client
  let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

  // CREATE Mint A.
  let account_space = spl_token::state::Mint::LEN;
  let mut transaction = Transaction::new_with_payer(
    &[
      system_instruction::create_account(
        &payer.pubkey(),
        &mint_a.pubkey(),
        1.max(Rent::default().minimum_balance(account_space)),
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
      ),
      spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_a.pubkey(),
        &payer.pubkey(),
        None, // Freeze authority
        6,
      )
      .unwrap(),
    ],
    Some(&payer.pubkey()),
  );
  transaction.sign(&[&payer, &mint_a], recent_blockhash);
  // Create mint:
  assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));

  // Create accounts for holding coins.
  let mut transaction = Transaction::new_with_payer(
    &[
      // Create Alice's account & transfer 1000 $A.
      system_instruction::create_account(
        &payer.pubkey(),
        &account_alice.pubkey(),
        1.max(Rent::default().minimum_balance(spl_token::state::Account::LEN)),
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
      ),
      spl_token::instruction::initialize_account(
        &spl_token::id(),
        &account_alice.pubkey(),
        &mint_a.pubkey(),
        &authority.pubkey(),
      )
      .unwrap(),
      spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_a.pubkey(),
        &account_alice.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        1000,
      )
      .unwrap(),
      // Create Alice's temp account.
      system_instruction::create_account(
        &payer.pubkey(),
        &account_alice_temp.pubkey(),
        1.max(Rent::default().minimum_balance(spl_token::state::Account::LEN)),
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
      ),
      spl_token::instruction::initialize_account(
        &spl_token::id(),
        &account_alice_temp.pubkey(),
        &mint_a.pubkey(),
        &account_alice.pubkey(),
      )
      .unwrap(),
    ],
    Some(&payer.pubkey()),
  );

  transaction.sign(
    &[&payer, &account_alice, &account_alice_temp],
    recent_blockhash,
  );
  // Create Alice's account with 1000 $A & temp-account for escrow.
  assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));

  // Transfer 100 from Alice's account to her temp.
  let mut transaction = Transaction::new_with_payer(
    &[spl_token::instruction::transfer(
      &spl_token::id(),
      &account_alice.pubkey(),
      &account_alice_temp.pubkey(),
      &authority.pubkey(),
      &[&&authority.pubkey()],
      100,
    )
    .unwrap()],
    Some(&payer.pubkey()),
  );
  transaction.sign(&[&payer, &authority], recent_blockhash);
  assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));

  // Verify some data on Alice's temp account for sanity checking & fun.
  let alice_account_temp_account = banks_client
    .get_account(account_alice_temp.pubkey())
    .await
    .unwrap()
    .expect("Account unretrievable");
  assert_eq!(alice_account_temp_account.owner, spl_token::id());
  let internal_account =
    spl_token::state::Account::unpack(&alice_account_temp_account.data).unwrap();
  assert_eq!(internal_account.owner, account_alice.pubkey());
  assert_matches!(
    internal_account.state,
    spl_token::state::AccountState::Initialized
  );

  // // Create Escrow account
  let mut transaction = Transaction::new_with_payer(
    &[
      // Create Alice's account & transfer 1000 $A.
      system_instruction::create_account(
        &payer.pubkey(),
        &account_escrow_state.pubkey(),
        1.max(Rent::default().minimum_balance(hearttoken::state::Escrow::LEN)),
        hearttoken::state::Escrow::LEN as u64,
        &hearttoken::id(),
      ),
      EscrowInstruction::initialize_escrow(
        &hearttoken::id(),
        &account_alice.pubkey(),
        &account_alice_temp.pubkey(),
        &account_alice.pubkey(), // Using Alice in lieu of Bob.
        &account_escrow_state.pubkey(),
        &spl_token::id(),
        100, // amount
      )
      .unwrap(),
    ],
    Some(&payer.pubkey()),
  );
  transaction.sign(
    &[
      &payer,
      &account_escrow_state,
      &account_alice,
      // &account_alice_temp,
    ],
    recent_blockhash,
  );
  // Create Alice's account with 1000 $A & temp-account for escrow.
  assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
  // Verify some data on Alice's temp account for sanity checking & fun.
  let alice_account_temp_account = banks_client
    .get_account(account_alice_temp.pubkey())
    .await
    .unwrap()
    .expect("Account unretrievable");
  assert_eq!(alice_account_temp_account.owner, spl_token::id());
  let internal_account =
    spl_token::state::Account::unpack(&alice_account_temp_account.data).unwrap();
  let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], &hearttoken::id());

  // Ensure that the escrow account's ownership
  assert_eq!(internal_account.owner, pda);
}
