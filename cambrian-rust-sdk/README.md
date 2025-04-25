# Cambrian Rust SDK

A Rust library for building payload containers that integrate with the Cambrian platform's Actively Validated Services (AVS) ecosystem.

## Overview

This SDK provides the core types and utilities needed to create Rust-based Cambrian payload containers. It simplifies the process of:

- Parsing the `CAMB_INPUT` environment variable
- Converting between Solana instructions and Cambrian payload formats
- Handling account roles with proper bitflag encoding
- Serializing output in the required JSON format

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
cambrian-rust-sdk = "0.1.0"
```

## Quick Example

```rust
use std::env;
use std::str::FromStr;

use anyhow::{Context, Result};
use cambrian_rust_sdk::{Input, Response};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;

fn main() -> Result<()> {
    // Parse input from environment variable
    let input: Input = serde_json::from_str(&env::var("CAMB_INPUT").unwrap_or_default())
        .context("Invalid CAMB_INPUT JSON")?;

    // Create a Solana instruction (simplified example)
    let program_id = Pubkey::from_str("Your_Program_ID_Here")?;
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(Pubkey::from_str("Account_Address_Here")?, false),
            AccountMeta::new_readonly(Pubkey::from_str("ReadOnly_Account_Here")?, true),
        ],
        data: vec![0, 1, 2, 3], // Instruction data
    };

    // Convert to Cambrian format and output JSON
    println!("{}", Response::from(instruction).to_output_ix()?);
    Ok(())
}
```

## Key Features

### Type Definitions

The SDK provides Rust types that match the Cambrian payload input/output formats:

- `Input`: Parses the JSON structure from `CAMB_INPUT` environment variable
- `Response`: The complete output format with proposal instructions
- `ProposalInstruction`: Represents a single instruction in the output
- `AccountMeta`: Account metadata with address and role information
- `AccountRole`: Enum representing the account role bitflags

### Conversion Utilities

The SDK includes conversions between Solana types and Cambrian payload formats:

- `From<Instruction>` for `ProposalInstruction`
- `From<&Instruction>` for `ProposalInstruction`
- `From<Instruction>` for `Response`
- `From<&Instruction>` for `Response`
- `From<Vec<Instruction>>` for `Response`
- `From<&[Instruction]>` for `Response`

### Account Role Handling

Account roles are represented using a bitflag system:

```
0b00 = READONLY     (not a signer, not writable)
0b01 = WRITABLE     (not a signer, writable)
0b10 = READONLY_SIGNER (signer, not writable)
0b11 = WRITABLE_SIGNER (signer, writable)
```

The SDK handles this conversion for you with `From<(bool, bool)>` for `AccountRole`.

## Complete Usage Example

For a complete example of using this SDK, see the [`check-oracle-rust`](https://github.com/cambrianone/payload-images/tree/master/check-oracle-rust) payload in the Cambrian payload images repository.

## Related Resources

- [Cambrian Platform Documentation](https://cambrianone.github.io/docs/camb-mvp/)
- [Payload Images Repository](https://github.com/cambrianone/payload-images)

## License

This project is licensed under the MIT License - see the LICENSE file for details.