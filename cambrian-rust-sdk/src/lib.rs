//! Cambrian Rust SDK - Type definitions and utilities for Cambrian AVS (Actively Validated Services) payloads

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use solana_program::instruction::Instruction;

/// Account roles for Cambrian payload instructions with bitflag representation
/// 
/// Bitflag guide: is signer ⌄⌄ is writable
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum AccountRole {
    WritableSigner = 0b11,
    ReadonlySigner = 0b10,
    Writable = 0b01,
    Readonly = 0b00,
}

/// Input structure from CAMB_INPUT environment variable
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// Name of the Proof of Authority
    pub poa_name: String,
    pub proposal_storage_key: String,
}

/// Account metadata for a Cambrian payload instruction
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMeta {
    pub address: String,
    pub role: AccountRole,
}

/// Cambrian proposal instruction format
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalInstruction {
    pub program_address: String,
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}

/// Complete response output for a Cambrian payload
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// List of instructions to be included in the proposal
    pub proposal_instructions: Vec<ProposalInstruction>,
}

impl Response {
    /// Serializes the response to a JSON string for output
    /// 
    /// Returns a properly formatted JSON string or an error
    pub fn to_output_ix(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}

impl From<Instruction> for ProposalInstruction {
    fn from(value: Instruction) -> Self {
        ProposalInstruction {
            program_address: value.program_id.to_string(),
            accounts: value
                .accounts
                .into_iter()
                .map(|meta| AccountMeta {
                    address: meta.pubkey.to_string(),
                    role: (meta.is_writable, meta.is_signer).into(),
                })
                .collect(),
            data: value.data,
        }
    }
}

impl From<&Instruction> for ProposalInstruction {
    fn from(value: &Instruction) -> Self {
        ProposalInstruction {
            program_address: value.program_id.to_string(),
            accounts: value
                .accounts
                .iter()
                .map(|meta| AccountMeta {
                    address: meta.pubkey.to_string(),
                    role: (meta.is_writable, meta.is_signer).into(),
                })
                .collect(),
            data: value.data.clone(),
        }
    }
}

impl From<&[Instruction]> for Response {
    fn from(value: &[Instruction]) -> Self {
        Response {
            proposal_instructions: value.iter().map(Into::into).collect(),
        }
    }
}

impl From<Instruction> for Response {
    fn from(value: Instruction) -> Self {
        Response {
            proposal_instructions: vec![value.into()],
        }
    }
}

impl From<&Instruction> for Response {
    fn from(value: &Instruction) -> Self {
        Response {
            proposal_instructions: vec![value.into()],
        }
    }
}

impl From<Vec<Instruction>> for Response {
    fn from(value: Vec<Instruction>) -> Self {
        Response {
            proposal_instructions: value.into_iter().map(Into::into).collect(),
        }
    }
}


impl From<(bool, bool)> for AccountRole {
    fn from((is_writable, is_signer): (bool, bool)) -> Self {
        match (is_writable, is_signer) {
            (false, false) => AccountRole::Readonly,
            (false, true) => AccountRole::ReadonlySigner,
            (true, false) => AccountRole::Writable,
            (true, true) => AccountRole::WritableSigner,
        }
    }
}
