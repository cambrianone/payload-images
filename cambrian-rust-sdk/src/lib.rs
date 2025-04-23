use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use solana_program::instruction::Instruction;

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum AccountRole {
    // Bitflag guide: is signer ⌄⌄ is writable
    WritableSigner = 0b11,
    ReadonlySigner = 0b10,
    Writable = 0b01,
    Readonly = 0b00,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub poa_name: String,
    pub proposal_storage_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMeta {
    pub address: String,
    pub role: AccountRole,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalInstruction {
    pub program_address: String,
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub proposal_instructions: Vec<ProposalInstruction>,
}

impl Response {
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
