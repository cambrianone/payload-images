# Cambrian Payload Images

**Reference implementations for Cambrian payload containers**

This repository contains ready-to-use examples of payload containers that integrate with the Cambrian platform's Autonomously Verifying Services (AVS) ecosystem.

## 📋 Prerequisites

```bash
# Install the Cambrian CLI
npm i --global @cambrianone/camb-client@latest

# Requirements
- Node.js ≥ 22.0.0
- Docker ≥ 20.0.0
```

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/cambrianone/payload-images
cd payload-images

# Build a payload container (choose one)
docker build -t payload-check-oracle ./check-oracle
# OR
docker build -t payload-check-oracle-rust ./check-oracle-rust

# Run the payload against your AVS
camb payload run-container -a <AVS public key | AVS URL> payload-check-oracle
```

> **Need the full setup?** Follow the [complete workflow guide](#complete-workflow) below.

## ✨ Available Examples

| Example | Language | Description | Code |
|---------|----------|-------------|------|
| [`check-oracle`](./check-oracle/src/index.ts) | TypeScript | Validates oracle data and generates instructions | [Source](./check-oracle/src/index.ts) |
| [`check-oracle-rust`](./check-oracle-rust/src/main.rs) | Rust | Same functionality as check-oracle, implemented in Rust | [Source](./check-oracle-rust/src/main.rs) |
| [`demo-transfer`](./demo-transfer/src/index.ts) | TypeScript | Demonstrates Solana token transfer instruction creation | [Source](./demo-transfer/src/index.ts) |
| [`cambrian-rust-sdk`](./cambrian-rust-sdk/src/lib.rs) | Rust | Shared library for building Rust-based payloads | [Source](./cambrian-rust-sdk/src/lib.rs) |

## 📂 Code Overview

### [`check-oracle/src/index.ts`](./check-oracle/src/index.ts)
This TypeScript implementation creates an oracle validation instruction. It:
- Processes input from the `CAMB_INPUT` environment variable
- Derives program addresses using seeds for the proposal storage and PoA state
- Constructs an instruction to check oracle data with proper account metadata
- Returns the formatted instruction data as JSON to stdout

### [`check-oracle-rust/src/main.rs`](./check-oracle-rust/src/main.rs)
The Rust version of the oracle checker that:
- Parses the `CAMB_INPUT` environment variable using the Cambrian Rust SDK
- Finds program-derived addresses for proposal storage and PoA state
- Creates a Solana instruction with the appropriate accounts and data
- Formats and outputs the instruction as JSON using the SDK's helper methods

### [`demo-transfer/src/index.ts`](./demo-transfer/src/index.ts)
A simple demonstration payload that:
- Creates a SOL transfer instruction from the executor PDA to a hardcoded address
- Uses the Solana system program's transfer instruction
- Formats the instruction into the required output format with proper encoding

### [`cambrian-rust-sdk/src/lib.rs`](./cambrian-rust-sdk/src/lib.rs)
A Rust library that provides:
- Type definitions for payload input/output formats
- Conversion utilities between Solana instructions and Cambrian payload formats
- Serialization helpers for JSON output
- Account role handling with proper bitflag encoding

## 🔌 Integration Guide

### Input Format

Your payload container receives this JSON input via the `CAMB_INPUT` environment variable:

```typescript
{
  "executorPDA": "string",    // Executor program-derived address
  "apiUrl": "string",         // API endpoint for interaction
  "extraSigners": ["string"], // Optional array of private keys
  "poaName": "string",        // Name of the Proof of Authority
  "proposalStorageKey": "string" // Storage key for the proposal
}
```

### Output Format

Your container must write a JSON-stringified object to stdout:

```typescript
{
  "proposalInstructions": [
    {
      "accounts": [
        {
          "address": "string", // Account address
          "role": 0|1|2|3      // Account role bitflag
        }
      ],
      "data": "string",        // Base58-serialized data
      "programAddress": "string" // Target program address
    }
  ]
}
```

Where `role` follows this enum:
```typescript
enum AccountRole {
    READONLY = 0,         // 0b00
    WRITABLE = 1,         // 0b01
    READONLY_SIGNER = 2,  // 0b10
    WRITABLE_SIGNER = 3   // 0b11
}
```

## 💡 Creating Your Own Payload

### Step-by-Step Guide

1. **Choose a starting point**: Select an existing example that's closest to your needs:
   - For TypeScript: Use [`check-oracle`](./check-oracle/) or [`demo-transfer`](./demo-transfer/)
   - For Rust: Use [`check-oracle-rust`](./check-oracle-rust/) and [`cambrian-rust-sdk`](./cambrian-rust-sdk/)

2. **Understand the payload lifecycle**:
   ```
   Input (CAMB_INPUT env var) → Process data → Generate instruction(s) → Output JSON to stdout
   ```

3. **Implement your logic**:
   - Modify the code to generate your specific Solana instructions
   - Ensure you follow the proper account metadata format (address and role)
   - Encode instruction data correctly (typically as Base58)

4. **Create a Dockerfile**: All payload containers require:
   - An entrypoint that runs your code
   - Access to the `CAMB_INPUT` environment variable
   - JSON output to stdout in the expected format

5. **Build and test locally**:
   ```bash
   # Build your container
   docker build -t my-custom-payload ./my-custom-payload
   
   # Test with mock input
   docker run -e CAMB_INPUT='{"executorPDA":"your-executor-pda","apiUrl":"https://api.example.com","poaName":"my-poa","proposalStorageKey":"my-key"}' my-custom-payload
   ```

6. **Deploy using the Cambrian CLI**:
   ```bash
   camb payload run-container -a <AVS public key | AVS URL> my-custom-payload
   ```

### Example: Creating a Custom Token Transfer Payload

Here's a minimal example of creating a custom SPL token transfer payload in TypeScript:

```typescript
// src/index.ts
import { AccountRole, getBase58Codec, address } from '@solana/web3.js';
import { getTokenTransferInstruction } from '@solana-program/token';

const run = async (input) => {
  try {
    // Create token transfer instruction
    const instruction = getTokenTransferInstruction({
      source: address('SourceTokenAccountAddress'),
      destination: address('DestinationTokenAccountAddress'),
      authority: address(input.executorPDA),
      amount: 1000000n, // 1 token with 6 decimals
    });
    
    // Format for Cambrian output
    const response = {
      proposalInstructions: [{
        programAddress: instruction.programId.toString(),
        accounts: instruction.keys.map(meta => ({
          address: meta.pubkey.toString(),
          role: (meta.isSigner ? 2 : 0) | (meta.isWritable ? 1 : 0)
        })),
        data: getBase58Codec().decode(instruction.data),
      }]
    };
    
    console.log(JSON.stringify(response));
  } catch (e) {
    console.error('Error:', e);
    throw e;
  }
};

const input = JSON.parse(process.env.CAMB_INPUT || '{}');
run(input).catch(console.error);
```

### Debugging and Troubleshooting

Common issues and solutions:

1. **Invalid output format**: Ensure your JSON output matches the expected format exactly
2. **Missing accounts**: All required accounts must be included with correct roles
3. **Incorrect data encoding**: Instruction data must be properly encoded
4. **Environment variable access**: Verify your container can access the `CAMB_INPUT` environment variable

To debug locally:
- Run with verbose logging: `docker run -e CAMB_INPUT='...' -e DEBUG=true my-payload`
- Inspect container logs: `docker logs <container-id>`
- Test different input parameters to validate behavior

## 🛠️ Complete Workflow

Follow these steps for a full setup:

1. **Scaffold an AVS**
   ```bash
   camb proposal init -t AVS <output directory>
   ```

2. **Run the AVS**
   ```bash
   camb avs run -u -v <AVS public key>
   ```

3. **Build an oracle update container**
   ```bash
   git clone https://github.com/cambrianone/oracle-update-examples
   docker build -t oracle-update-current-date ./oracle-update-examples/current-date/container-stream
   ```

4. **Build the payload image**
   ```bash
   docker build -t payload-check-oracle ./check-oracle
   ```

5. **Scaffold operator(s)**
   ```bash
   camb proposal init -t operator <output directory>
   ```
   Choose `container-stream` as oracle update method and set `oracle-update-current-date` as oracle update container image.
   
   > Admin private key is in `~/.cambrian/config.toml`

6. **Run the operator(s)**
   ```bash
   camb operator run -a <AVS public key> -v <voter public key>
   ```

7. **Run the payload**
   ```bash
   camb payload run-container -a <AVS public key | AVS URL> payload-check-oracle
   ```

## 📚 Resources

- [Cambrian Platform Documentation](https://cambrianone.github.io/docs/camb-mvp/)
- [CLI Flow Guide](https://cambrianone.github.io/docs/camb-mvp/docs/cli-flow/)
- [Oracle Update Examples](https://github.com/cambrianone/oracle-update-examples)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
