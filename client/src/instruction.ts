const BufferLayout = require("buffer-layout");
import { Buffer } from 'buffer';
const VariantLayout = BufferLayout.getVariantLayout;
import {
  Connection,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
  Signer,
  SystemProgram,
  Keypair,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
// import BN = require("bn.js");
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";

export const VAULT_PROGRAM_ID = new PublicKey(
  "9VxcdZKmmL6xwJWZorYnD29tZte5M29XAiKv3ZEW2AJd"
);

// Accounts expected:
// `[signer]` initializer of the lx token account
// `[writeable]` Vault storage account (vault ID)
// `[]` Vault's lX or X token account
// `[]` The llX mint account
// `[]` The strategy program
// `[]` The rent sysvar
// `[]` (Optional) Strategy instance data account
const InitializeVault = 0;
let vault_instruction_layout = {};
vault_instruction_layout["InitializeVault"] = [
  0,
  BufferLayout.struct([
    BufferLayout.u8("instruction_num"),
    // TODO(007): Governance address, strategist address, keeper address.
    // TODO(008): Withdrawal fee.
    // https://github.com/yearn/yearn-vaults/blob/master/contracts/BaseStrategy.sol#L781
    BufferLayout.u8("hodl"),
    BufferLayout.u8("strategy_program_deposit_instruction_id"),
    BufferLayout.u8("strategy_program_withdraw_instruction_id"),
    BufferLayout.u8("strategy_program_estimate_instruction_id"),
  ]),
];

// Deposits a given token into the vault.
//
// Note this API is an implementation of the StrategyInstruction#Deposit instruction.
//
// Accounts expected:
// 1. `[]` SPL Token program
// 2. `[signer]` The source wallet containing X tokens.
// 3. `[]` The target wallet for llX tokens.
// 4+ `[]` Source signers
// 5. `[]` The Vault storage account.
// 6. `[]` The strategy program.
// 7. `[]` (Optional) X SPL account owned by Vault if hodling.
// 8+. `[]` Strategy extra accoounts (see StrategyInstruction#Deposit)
// TODO(009):: Signer pubkeys for multisignature wallets - need signer_num param.
const Deposit = 1;
vault_instruction_layout["Deposit"] = [
  Deposit,
  BufferLayout.struct([
    BufferLayout.u8("instruction_num"),
    BufferLayout.nu64("amount"),
  ]),
];

// Withdraws a token from the vault.
//
// Note this API is an implementation of the StrategyInstruction#Withdraw instruction.
//
// Accounts expected:
// 1. `[]` SPL Token program
// 2. `[signer]` Source Wallet for derivative token (lX).
// 3. `[]` Target token (X) wallet target.
// 4+ `[]` Source signers
// 5. `[]` The Vault storage account.
// 6. `[]` The strategy program.
// 7. `[]` (Optional) X SPL account owned by Vault if hodling.
// 8+. `[]` Strategy extra accoounts (see StrategyInstruction#Withdraw)
// TODO(009):: Signer pubkeys for multisignature wallets - need signer_num param.
const Withdraw = 2;
vault_instruction_layout["Withdraw"] = [
  Withdraw,
  BufferLayout.struct([
    BufferLayout.u8("instruction_num"),
    BufferLayout.nu64("amount"), // # of derivative tokens.
  ]),
];

// Estimates the underlying value of the vault in its native asset.
//
// This instruction stores its results in a temporary account using the Shared Memory program.
// https://spl.solana.com/shared-memory
//
// Accounts expected:
// 1. `[]` Shared Memory program
// 1. `[]` Shared memory output
// 2. `[]` The Vault storage account.
// 3. `[]` (Optional) X SPL account owned by Vault if hodling.
// 4+ `[*]` Strategy extra accounts - any additional accounts required by strategy
const EstimateValue = 3;
vault_instruction_layout["EstimateValue"] = [
  EstimateValue,
  BufferLayout.struct([BufferLayout.u8("instruction_num")]),
];

// A helper utility which functions similarly to the (unlaunched) Shared Memory program.
//
// Data is read directly from the account memory.
const WriteData = 4;
vault_instruction_layout["WriteData"] = [
  WriteData,
  BufferLayout.struct([BufferLayout.u8("instruction_num")]),
];

export async function e2e(connection: Connection, payerAccount: Keypair) {
  await addLamports(connection, payerAccount);
  console.log("Setup payer account");
  const tokenA = await Token.createMint(
    connection,
    payerAccount,
    payerAccount.publicKey,
    null,
    6,
    TOKEN_PROGRAM_ID
  );

  // console.log("Created mints");
  // await addLamports(connection, payerAccount, 100000);
  const clientTokenAAccountKey = await tokenA.createAccount(
    payerAccount.publicKey
  );
  // const clientTokenlAAccountKey = await tokenlA.createAccount(payerAccount.publicKey);
  const vaultTokenAAccountKey = await tokenA.createAccount(
    payerAccount.publicKey
  );
  // const vaultTokenlAAccountKey = await tokenlA.createAccount(payerAccount.publicKey);
  await addLamports(connection, payerAccount, 100000);
  await tokenA.mintTo(clientTokenAAccountKey, payerAccount, [], 1000);
  console.log(`Created accounts and sent 1000 tokens to ${clientTokenAAccountKey}.`);
  let account_info = await tokenA.getAccountInfo(clientTokenAAccountKey);
  expect(account_info.amount.toString()).toEqual('1000');
  console.log(`Confirmed balance of 1000 tokens.`);

  const tokenlA = await Token.createMint(connection, payerAccount, payerAccount.publicKey, null, 6, TOKEN_PROGRAM_ID);
  // const vaultTokenlAAccountKey = await tokenlA.createAccount(payerAccount.publicKey);
  const clientTokenlAAccountKey = await tokenlA.createAccount(payerAccount.publicKey);
  // Setup the HODL vault for tokenA
  await addLamports(connection, payerAccount, 100000);
  await createHodlVault(connection, payerAccount, vaultTokenAAccountKey,
    false // debug_crash
  ).then(
    async (vaultStorageAccount: Keypair) => {
      console.log("Created hodl vault");
      await deposit(
        connection,
        payerAccount,
        VAULT_PROGRAM_ID,
        vaultStorageAccount.publicKey,
        clientTokenAAccountKey,
        clientTokenlAAccountKey,
        vaultTokenAAccountKey,
        10,
        false // debug_crash
      ).then(async (_) => {
        console.log("Deposited into vault account", vaultTokenAAccountKey.toBase58());
        await tokenA.getAccountInfo(vaultTokenAAccountKey).then(account_info => {
          console.log("vaultTokenAAccountKey ", vaultTokenAAccountKey.toBase58(), account_info);
          expect(account_info.amount.toString()).toEqual("10");
          console.log("Confirmed vault balance of 10 tokens.");
        });
        await withdraw(
          connection,
          payerAccount,
          VAULT_PROGRAM_ID,
          vaultStorageAccount.publicKey,
          clientTokenlAAccountKey,
          clientTokenAAccountKey,
          vaultTokenAAccountKey,
          10,
          false // debug_crash
        ).then(async (_) => {
          console.log("Withdrew {} from vault {}", 10, vaultTokenAAccountKey);
          console.log(`Created accounts and sent 1000 tokens to ${clientTokenAAccountKey}.`);
          let account_info = await tokenA.getAccountInfo(vaultTokenAAccountKey);
          expect(account_info.amount.toString()).toEqual('0');
          console.log(`Confirmed vault balance of 0 tokens.`);
        });
      });
    }
  );
}


export async function createHodlVault(
  connection: Connection,
  payer_account: Keypair,
  vault_x_token_account: PublicKey,
  debug_crash: boolean
): Promise<Keypair> {
  let vault_storage_account = new Keypair();
  const token_vault_derivative = await Token.createMint(
    connection,
    payer_account,
    payer_account.publicKey,
    null,
    6,
    TOKEN_PROGRAM_ID
  );
  let transaction = await createHodlVaultTransaction(connection, vault_storage_account.publicKey, payer_account.publicKey, vault_x_token_account, token_vault_derivative.publicKey, debug_crash);
  console.log("Sending instruction to create HODL vault");
  await lagunaSendAndConfirmTransaction(connection, transaction, [payer_account, vault_storage_account]);
  console.log(
    "vault_storage_account ",
    vault_storage_account.publicKey.toBase58(),
    connection.getAccountInfo(vault_storage_account.publicKey)
  );
  console.log(
    "vault_x_token_account ",
    vault_x_token_account.toBase58(),
    connection.getAccountInfo(vault_x_token_account)
  );
  console.log("Created hodl vault");
  let account_info = await connection.getAccountInfo(vault_storage_account.publicKey);
  console.log("data_account ", vault_storage_account.publicKey.toBase58(), account_info);
  return vault_storage_account;
}

export function createHodlDepositInstruciton(
  payer_account: PublicKey,
  strategy_program: PublicKey,
  vault_account: PublicKey,
  client_x_token_account: PublicKey,
  client_lx_token_account: PublicKey,
  vault_x_token_account: PublicKey,
  amount: number,
  debug_crash: boolean) : TransactionInstruction {
    return depositInstruction(
      VAULT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      client_x_token_account,
      client_lx_token_account,
      [
        { isWritable: false, pubkey: payer_account, isSigner: true },
        { isWritable: true, pubkey: vault_account, isSigner: false },
        { isWritable: false, pubkey: strategy_program, isSigner: false },
        { isWritable: true, pubkey: vault_x_token_account, isSigner: false },
      ],
      amount,
      debug_crash
    );
}

export async function deposit(
  connection: Connection,
  payer_account: Keypair,
  strategy_program: PublicKey,
  vault_account: PublicKey,
  client_x_token_account: PublicKey,
  client_lx_token_account: PublicKey,
  vault_x_token_account: PublicKey,
  amount: number,
  debug_crash: boolean
): Promise<void> {
  console.log("vault_account ", vault_account.toBase58());
  let transaction = new Transaction();
  console.log("payer_account {}", payer_account.publicKey.toBase58());
  transaction.add(
    createHodlDepositInstruciton(payer_account.publicKey ,strategy_program ,vault_account ,client_x_token_account ,client_lx_token_account ,vault_x_token_account ,amount, debug_crash)
  );
  console.log("vault_x_token_account ", vault_x_token_account.toBase58());
  await lagunaSendAndConfirmTransaction(connection, transaction, [payer_account]);
}


export async function createHodlWithdrawInstruction(
  connection: Connection,
  payer_account: Keypair,
  strategy_program: PublicKey,
  vault_account: PublicKey,
  client_lx_token_account: PublicKey,
  client_x_token_account: PublicKey,
  vault_x_token_account: PublicKey,
  amount: number,
  debug_crash: boolean
): Promise<void> {
  console.log("vault_account ", vault_account.toBase58());
  let transaction = new Transaction();
  console.log("payer_account {}", payer_account.publicKey.toBase58());
  console.log("client_x_token_account {}", client_x_token_account.toBase58());
  let _, bump_pda = await PublicKey.findProgramAddress([Buffer.from("vault", 'utf-8')], VAULT_PROGRAM_ID);
  let pda = bump_pda[0];
  console.log("pda", pda);

  console.log(`client_lx_token_account ${client_lx_token_account}`);
  transaction.add(
    withdrawInstruction(
      VAULT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      // Note the source is now the vault - not the client.
      client_lx_token_account,
      client_x_token_account,
      [
        { isWritable: false, pubkey: pda, isSigner: false },
        { isWritable: false, pubkey: vault_account, isSigner: false },
        { isWritable: false, pubkey: strategy_program, isSigner: false },
        { isWritable: true, pubkey: vault_x_token_account, isSigner: false },
      ],
      amount,
      debug_crash
    )
  );
  await lagunaSendAndConfirmTransaction(connection, transaction, [payer_account]);
}

export async function withdraw(
  connection: Connection,
  payer_account: Keypair,
  strategy_program: PublicKey,
  vault_account: PublicKey,
  client_lx_token_account: PublicKey,
  client_x_token_account: PublicKey,
  vault_x_token_account: PublicKey,
  amount: number,
  debug_crash: boolean
): Promise<void> {
  console.log("vault_account ", vault_account.toBase58());
  let transaction = new Transaction();
  console.log("payer_account {}", payer_account.publicKey.toBase58());
  console.log("client_x_token_account {}", client_x_token_account.toBase58());
  let _, bump_pda = await PublicKey.findProgramAddress([Buffer.from("vault", 'utf-8')], VAULT_PROGRAM_ID);
  let pda = bump_pda[0];
  console.log("pda", pda);

  console.log(`client_lx_token_account ${client_lx_token_account}`);
  transaction.add(
    withdrawInstruction(
      VAULT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      // Note the source is now the vault - not the client.
      client_lx_token_account,
      client_x_token_account,
      [
        { isWritable: false, pubkey: pda, isSigner: false },
        { isWritable: false, pubkey: vault_account, isSigner: false },
        { isWritable: false, pubkey: strategy_program, isSigner: false },
        { isWritable: true, pubkey: vault_x_token_account, isSigner: false },
      ],
      amount,
      debug_crash
    )
  );
  await lagunaSendAndConfirmTransaction(connection, transaction, [payer_account]);
}


export async function createHodlVaultTransaction(
  connection: Connection,
  vault_storage_account: PublicKey,
  payer_account_public_key: PublicKey,
  vault_x_token_account: PublicKey,
  token_vault_derivative: PublicKey,
  debug_crash: boolean
) {
  console.log("Creating HODL vault");


  // const vault_vault_token_account = await token_vault_derivative.createAccount(payer_account_public_key);
  let transaction = new Transaction();
  let instruction = await createAccountInstruction
    (
      connection,
      vault_storage_account,
      payer_account_public_key,
      1 + 1 + 32 + 32 + 8 + 32 + 1 + 1 + 1 + 36,
      VAULT_PROGRAM_ID
    );
  transaction.add(instruction);
  console.log("vault_storage_account ", vault_storage_account);
  // let vault_token_account = createAccount(connection, payer_account_public_key, numBytes, PROGRAM_ID);
  // let llx_token_mint_id = createAccount(connection, payer_account_public_key, numBytes, PROGRAM_ID);
  instruction = initializeVaultInstruction(
    payer_account_public_key, // TODO: Separate owner
    vault_storage_account,
    vault_x_token_account,
    token_vault_derivative,
    TOKEN_PROGRAM_ID,
    VAULT_PROGRAM_ID,
    1,
    2,
    3,
    true,
    debug_crash // debug_crash
  );
  transaction.add(instruction);

  return transaction;
}

// Note: These instructions mirror instruction.rs.
function initializeVaultInstruction(
  vault_token_account_owner: PublicKey,
  vault_storage_account: PublicKey,
  vault_token_account: PublicKey,
  llx_token_mint_id: PublicKey,
  token_program: PublicKey,
  strategy_program: PublicKey,
  strategy_program_deposit_instruction_id: number,
  strategy_program_withdraw_instruction_id: number,
  strategy_program_estimate_value_instruction_id: number,
  hodl: boolean,
  debug_crash: boolean
) {
  console.log(
    "vault_token_account_owner ",
    vault_token_account_owner.toBase58()
  );
  if (debug_crash) {
    console.log("Crashing initialize vault!")
  } else {
    console.log("Not crashing init");
  }
  console.log("vault_storage_account ", vault_storage_account.toBase58());
  console.log("vault_token_account ", vault_token_account.toBase58());
  console.log("llx_token_mint_id ", llx_token_mint_id.toBase58());
  console.log("token_program ", token_program.toBase58());
  console.log("strategy_program ", strategy_program.toBase58());
  const accounts = [
    { pubkey: vault_token_account_owner, isSigner: true, isWritable: false },
    { pubkey: vault_storage_account, isSigner: false, isWritable: true },
    { pubkey: vault_token_account, isSigner: false, isWritable: true },
    { pubkey: llx_token_mint_id, isSigner: false, isWritable: true },
    { pubkey: token_program, isSigner: false, isWritable: false },
    { pubkey: strategy_program, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];
  let data = {
    instruction_num: InitializeVault + (debug_crash ? 64 : 0),
    hodl: hodl,
    strategy_program_deposit_instruction_id:
      strategy_program_deposit_instruction_id,
    strategy_program_withdraw_instruction_id:
      strategy_program_withdraw_instruction_id,
    strategy_program_estimate_instruction_id:
      strategy_program_estimate_value_instruction_id,

  };
  console.log("instruction data ", data);
  console.log(
    "instruction layout size {}",
    vault_instruction_layout["InitializeVault"][1].getSpan(data)
  );
  return new TransactionInstruction({
    keys: accounts,
    data: encodeInstructionData(
      data,
      vault_instruction_layout["InitializeVault"][1]
    ),
    programId: VAULT_PROGRAM_ID,
  });
}


function depositInstruction(
  vault_program_id: PublicKey,
  token_program_id: PublicKey,
  client_x_token: PublicKey,
  vault_x_token_pubkey: PublicKey,
  additional_account_metas: {
    isWritable: boolean;
    pubkey: PublicKey;
    isSigner: boolean;
  }[],
  amount: number,
  debug_crash: boolean
) {
  console.log("vault_program_id {}", vault_program_id.toBase58());
  console.log("token_program_id {}", token_program_id.toBase58());
  console.log("client_x_token {}", client_x_token.toBase58());
  console.log("vault_x_token_pubkey {}", vault_x_token_pubkey.toBase58());
  if (debug_crash) {
    console.log("Crashing deposit!")
  } else {
    console.log("Not crashing");
  }
  let data = {
    instruction_num: Deposit + (debug_crash ? 64 : 0),
    amount,
  };

  let instructionData = encodeInstructionData(
    data,
    vault_instruction_layout["Deposit"][1]
  );
  return createTransferInstruction(
    instructionData,
    vault_program_id,
    token_program_id,
    client_x_token,
    vault_x_token_pubkey,
    additional_account_metas
  );
}

function withdrawInstruction(
  vault_program_id: PublicKey,
  token_program_id: PublicKey,
  source_pubkey: PublicKey,
  target_pubkey: PublicKey,
  additional_account_metas: {
    isWritable: boolean;
    pubkey: PublicKey;
    isSigner: boolean;
  }[],
  amount: number,
  debug_crash: boolean
) {
  console.log("vault_program_id {}", vault_program_id.toBase58());
  console.log("token_program_id {}", token_program_id.toBase58());
  console.log("source_pubkey {}", source_pubkey.toBase58());
  console.log("target_pubkey {}", target_pubkey.toBase58());
  let data = {
    instruction_num: Withdraw + (debug_crash ? 64 : 0),
    amount,
  };
  let instructionData = encodeInstructionData(
    data,
    vault_instruction_layout["Withdraw"][1]
  );
  return createTransferInstruction(
    instructionData,
    vault_program_id,
    token_program_id,
    source_pubkey,
    target_pubkey,
    additional_account_metas
  );
}
export function createTransferInstruction(
  data: Buffer,
  program_id: PublicKey,
  token_program_id: PublicKey,
  source_pubkey: PublicKey,
  target_pubkey: PublicKey,
  additional_account_metas: {
    isWritable: boolean;
    pubkey: PublicKey;
    isSigner: boolean;
  }[]
) {
  let accounts: {
    isWritable: boolean;
    pubkey: PublicKey;
    isSigner: boolean;
  }[] = [
    { isWritable: false, pubkey: token_program_id, isSigner: false },
    { isWritable: true, pubkey: source_pubkey, isSigner: false },
    { isWritable: true, pubkey: target_pubkey, isSigner: false },
  ].concat(additional_account_metas);
  // accounts = accounts;
  return new TransactionInstruction({
    keys: accounts,
    data,
    programId: VAULT_PROGRAM_ID,
  });
}

// Utility functions -------------------------------------------------------------------------------

export async function lagunaSendAndConfirmTransaction(connection: Connection, transaction: Transaction, signers: Signer[]): Promise<void> {
  await sendAndConfirmTransaction(connection, transaction, signers,
    {
      commitment: 'singleGossip',
      preflightCommitment: 'singleGossip',
    }).then(val =>
      console.log(`Successfully executed transaction: ${val}`)
    ).catch(err => {
      console.log(`Failed to create hodl vault ${err}`, err);
      throw err;
    });
}

function encodeInstructionData(
  instruction: Object,
  layout: typeof BufferLayout
) {
  console.log("Ensuring all data is provided in layout..");
  console.log("Layout {}", layout);
  for (let i = 0; i < layout.fields.length; i++) {
    let field = layout.fields[i];
    // console.log(`field[${i}] ${field} ${typeof (field)} ${field.property}`);
    if (!instruction.hasOwnProperty(field.property)) {
      console.log(`Missing layout field ${field.property}`);
      throw field;
    }
  }

  // const instructionMaxSpan = Math.max(...Object.values(layout.registry).map((r: typeof BufferLayout) => r.span));
  const b = Buffer.alloc(layout.getSpan(instruction));
  const span = layout.encode(instruction, b);
  return b.slice(0, span);
}

export async function createAccountInstruction(
  connection: Connection,
  dataAccount: PublicKey,
  payerAccountPublicKey: PublicKey,
  numBytes: number,
  programId: PublicKey
) : Promise<TransactionInstruction> {
  const rentExemption = await connection.getMinimumBalanceForRentExemption(
    numBytes
  );
  return SystemProgram.createAccount({
    fromPubkey: payerAccountPublicKey,
    newAccountPubkey: dataAccount,
    lamports: rentExemption,
    space: numBytes,
    programId: programId,
  });
}

export async function createAccount(
  connection: Connection,
  payerAccount: Keypair,
  numBytes: number,
  programId: PublicKey
) {
  const dataAccount = new Keypair();
  const rentExemption = await connection.getMinimumBalanceForRentExemption(
    numBytes
  );
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: rentExemption,
      space: numBytes,
      programId: programId,
    })
  );
  await sendAndConfirmTransaction(connection, transaction, [
    payerAccount,
    dataAccount,
  ]);
  let account_info = await connection.getAccountInfo(dataAccount.publicKey);
  console.log("data_account ", dataAccount.publicKey.toBase58(), account_info);
  return dataAccount.publicKey;
}


function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function addLamports(
  connection: Connection,
  account: Keypair,
  lamports = 100000
) {
  console.log(`Balance (lamports): ${(await connection.getBalance(account.publicKey))}`);
  if (lamports <= (await connection.getBalance(account.publicKey))) {
    const count = await connection.getBalance(account.publicKey);
    console.log(`${count} lamports held by payer`);
    return account;
  }

  for (let retry = 0; retry < 10; retry++) {
    try {
      await connection.requestAirdrop(account.publicKey, lamports);
      break;
    } catch (e) {
      console.log(`Airdrop failed: ${e}`);
    }
  }
  
  console.log(`Balance (lamports): ${(await connection.getBalance(account.publicKey))}`);
  for (let retry = 0; retry < 10; retry++) {
    await sleep(2000);
    if (lamports >= (await connection.getBalance(account.publicKey))) {
      const count = await connection.getBalance(account.publicKey);
      console.log(`${count} lamports held by payer`);
      return account;
    }
    console.log(`Airdrop retry ${retry}`);
  }
  throw new Error(`Airdrop of ${lamports} failed`);
}