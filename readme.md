# txdecode

A simple Rust CLI that decodes Solana transactions into a human-readable format.

## Features

* Fetches a transaction using its signature
* Displays transaction metadata

  * Slot
  * Status
  * Fee
* Lists all account keys
* Resolves program names
* Decodes:

  * System Program (Transfer, Advance Nonce)
  * Compute Budget instructions
  * SPL Token TransferChecked
* Shows SOL balance changes
* Shows SPL Token balance changes
* Resolves common token names (USDC, BONK, JUP, etc.)

## Tech Stack

* Rust
* Tokio
* Reqwest
* Serde JSON
* Solana JSON-RPC

## Usage

```bash
cargo run -- <TRANSACTION_SIGNATURE>
```

Example:

```bash
cargo run -- 5yN8...
```

## Example Output

```text
Transaction
-------------------------
Slot: 357786364
Status: Success
Fee: 64457 lamports

Account Keys
-------------------------
0: ...
1: ...

Instructions
-------------------------
Instruction 0
Program      : Compute Budget

Instruction 1
Program      : SPL Token
Instruction  : TransferChecked
Token        : USDC
Amount       : 251

Balance Changes
-------------------------
-0.000064457 SOL

Token Balance Changes
-------------------------
USDC
-251
+251
```

## Supported Programs

* System Program
* Compute Budget Program
* SPL Token Program
