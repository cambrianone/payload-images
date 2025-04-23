# Cambrian payload images examples

## Prerequisites

- Installed [Cambrian CLI utility](https://cambrianone.github.io/docs/camb-mvp/)
- docker >= 20.0.0

## Running payload

- Scaffold AVS: `camb proposal init -t AVS <output directory>`
- Run AVS: `camb avs run -u -v <AVS public key>`
- Build container for updating oracle data. You can use [one of the reference images](https://github.com/cambrianone/oracle-update-examples): `git clone https://github.com/cambrianone/oracle-update-examples && docker build -t oracle-update-current-date ./oracle-update-examples/current-date/container-stream`
- Build payload image: `git clone https://github.com/cambrianone/payload-images && docker build -t payload-check-oracle ./payload-images/check-oracle`
- Scaffold operator(s) and choose `container-stream` as oracle update method and set `oracle-update-current-date` as oracle update container image: `camb proposal init -t operator <output directory>`, you can find an admin private key in `~/.cambrian/config.toml`
- Run operator(s): `camb operator run -a <AVS public key> -v <voter public key>`
- Run payload: `camb payload run-container -a <AVS public key | AVS URL> payload-check-oracle`

See more details [here](https://cambrianone.github.io/docs/camb-mvp/#flow) and [here](https://cambrianone.github.io/docs/camb-mvp/docs/cli-flow/)

Payload container receives a parameter (in `CAMB_MVP` environment variable) serialized as JSON-object.
It's type is:

```typescript
type TPayloadInput = {
  executorPDA: string;
  apiUrl: string;
  extraSigners: Array<string>;
  poaName: string;
  proposalStorageKey: string;
}

```

`extraSigners` represents an optional array of serialized private keys used for signing transaction.

Container should write a JSON-stringified object.
It's type is:

```typescript
type TPayloadOutput = {
  proposalInstructions: Array<{
    accounts: Array<{
      address: string;
      role: 0 | 1 | 2 | 3
    }>,
    data: string;
    programAddress: string;
  }>;
}
```
where `data` is base58-serialized data buffer and `role` is the following enum:

```typescript
enum AccountRole {
    // Bitflag guide: is signer ⌄⌄ is writable
    WRITABLE_SIGNER = /* 3 */ 0b11,
    READONLY_SIGNER = /* 2 */ 0b10,
    WRITABLE =        /* 1 */ 0b01,
    READONLY =        /* 0 */ 0b00,
}
```

This type could be represented as JSON-schema:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TPayloadOutput",
  "type": "object",
  "properties": {
    "proposalInstructions": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "accounts": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "address": {
                  "type": "string",
                  "description": "Account address"
                },
                "role": {
                  "type": "integer",
                  "enum": [0, 1, 2, 3],
                  "description": "Account role as bitflag (is signer | is writable)\n\n0 (0b00): READONLY\n1 (0b01): WRITABLE\n2 (0b10): READONLY_SIGNER\n3 (0b11): WRITABLE_SIGNER"
                }
              },
              "required": ["address", "role"],
              "additionalProperties": false
            },
            "description": "Array of accounts involved in the instruction"
          },
          "data": {
            "type": "string",
            "description": "Instruction data as base58 encoded string"
          },
          "programAddress": {
            "type": "string",
            "description": "Program address for the instruction"
          }
        },
        "required": ["accounts", "data", "programAddress"],
        "additionalProperties": false
      },
      "description": "Array of proposal instructions"
    }
  },
  "required": ["proposalInstructions"],
  "additionalProperties": false
}
```      
