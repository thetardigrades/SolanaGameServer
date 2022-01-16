use solana_program;

pub mod entrypoint;
pub mod processor;
pub mod state;
pub mod instruction;
pub mod error;

// Random based on Token ID's ID. Defines hearttoken::id().
solana_program::declare_id!("EscrowegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
