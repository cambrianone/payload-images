use std::env;
use std::str::FromStr;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u8)]
enum AccountRole {
    // Bitflag guide: is signer ⌄⌄ is writable
    WritableSigner = /* 3 */ 0b11,
    ReadonlySigner = /* 2 */ 0b10,
    Writable =        /* 1 */ 0b01,
    Readonly =        /* 0 */ 0b00,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    poa_name: String,
    proposal_storage_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountMeta {
    address: String,
    role: AccountRole,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProposalInstruction {
    program_address: String,
    accounts: Vec<AccountMeta>,
    data: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    proposal_instructions: Vec<ProposalInstruction>,
}

fn main() -> Result<()> {
    let input: Input = serde_json::from_str(&env::var("CAMB_INPUT").unwrap_or_default())
        .context("Invalid CAMB_INPUT JSON")?;

    let solana_program = Pubkey::from_str("FGgNUqGxdEYM1gVtQT5QcTbzNv4y1UPoVvXPRnooBdxo")?;
    let oracle_program = Pubkey::from_str("ECb6jyKXDTE8NjVjsKgNpjSjcv4h2E7JQ42yKqWihBQE")?;
    let sysvar_instructions = "Sysvar1nstructions1111111111111111111111111";

    let poa_state_key = input.poa_name.as_bytes();
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

    let instruction = ProposalInstruction {
        program_address: oracle_program.to_string(),
        accounts: vec![
            AccountMeta {
                address: proposal_storage_pda.to_string(),
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: poa_state_pda.to_string(),
                role: AccountRole::Readonly
            },
            AccountMeta {
                address: sysvar_instructions.to_string(),
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: oracle_program.to_string(),
                role: AccountRole::Readonly,
            },
        ],
        data: poa_state_key.to_vec(),
    };

    let response = Response {
        proposal_instructions: vec![instruction],
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
