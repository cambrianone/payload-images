//! Cambrian check-oracle payload - Rust implementation
//! 
//! Validates oracle data and generates instructions for Cambrian AVS

use std::env;
use std::str::FromStr;

use anyhow::{Context, Result};
use cambrian_rust_sdk::{Input, Response};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;

fn main() -> Result<()> {
    let input: Input = serde_json::from_str(&env::var("CAMB_INPUT").unwrap_or_default())
        .context("Invalid CAMB_INPUT JSON")?;

    let solana_program = Pubkey::from_str("FGgNUqGxdEYM1gVtQT5QcTbzNv4y1UPoVvXPRnooBdxo")?;
    let oracle_program = Pubkey::from_str("ECb6jyKXDTE8NjVjsKgNpjSjcv4h2E7JQ42yKqWihBQE")?;
    let sysvar_instructions = Pubkey::from_str("Sysvar1nstructions1111111111111111111111111")?;

    let poa_state_key = input.poa_name.as_bytes();
    // Storage space for Cambrian proposal (3 instructions × 25 bytes)
    let storage_space: u32 = 3 * 25;
    let (proposal_storage_pda, _) = Pubkey::find_program_address(
        &[
            b"STORAGE",
            input.poa_name.as_bytes(),
            input.proposal_storage_key.as_bytes(),
            &storage_space.to_le_bytes(),
        ],
        &solana_program,
    );

    let (poa_state_pda, _) =
        Pubkey::find_program_address(&[b"STATE", poa_state_key], &solana_program);

    let instruction = Instruction {
        program_id: oracle_program,
        accounts: vec![
            AccountMeta {
                pubkey: proposal_storage_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: poa_state_pda,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: sysvar_instructions,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: oracle_program,
                is_signer: false,
                is_writable: false,
            },
        ],
        data: poa_state_key.to_vec(),
    };

    println!("{}", Response::from(instruction).to_output_ix()?);
    Ok(())
}
