# solana-raw (On-Chain Program)

This is a low-level Solana smart contract written in pure Rust using the `solana_program` crate (no Anchor).

## Features

- Vault program storing SOL or tokens
- Uses raw instruction handling and account serialization
- Borsh-based account state

## Usage

Deploy using Solana CLI:
```bash
solana program deploy ./path/to/your/program.so
