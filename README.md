# solana-raw

This is a minimal Solana on-chain program written in pure Rust using the `solana_program` crate (without Anchor). It demonstrates a basic vault-style smart contract with custom instruction unpacking, manual account validation, and state mutation.


## Folder Structure

```
solana-raw/
├── program/
│   ├── Cargo.toml
│   └── src/
│       ├── entrypoint.rs
│       ├── instruction.rs
│       ├── processor.rs
│       ├── state.rs
│       └── lib.rs
├── .gitignore
└── README.md
```

## Features


- Pure Rust Solana smart contract without Anchor.
- Handles two instructions: `Initialize` and `Increment`.
- Uses Borsh for serialization of account state.
- Stores and updates state in a PDA-based vault account.


## Program Logic

### Entrypoint

The `entrypoint.rs` file defines the Solana program's entrypoint, similar to `main()` in a regular Rust binary. It delegates execution to the `processor`.

```rust
entrypoint!(process_instruction);
```


### Instruction

The `instruction.rs` file defines the supported instructions your program can handle. In this case, there are two instructions:

- `Initialize`: Used to set up the vault account.
- `Increment`: Increases a counter or amount stored in the vault.

```rust

pub enum VaultInstruction {
    Initialize,
    Increment,
}
```

### State

The `state.rs` file defines the on-chain state structure that holds the total amount deposited.

```rust
pub struct VaultState {
    pub initialized: bool,
    pub counter: u64,
}
```

### Processor

The `processor.rs` handles logic based on incoming instructions and updates the state accordingly. For example, it:

- Deserializes `instruction_data`.
- Validates and mutates accounts.
- Writes new state to the vault account.

## Build and Deploy

### Build Program

```sh
cd program
cargo build-bpf
```

### Deploy to Devnet

```sh
solana program deploy /path/to/solana-raw/program/target/deploy/solana_raw.so
```

### Set Environment

```sh
solana config set --url devnet
solana address -k deployer-keypair.json
```

## Security Note

- Do not upload `.json` keypairs to GitHub.
- Vault account and program ID are safe if derived through CLI (`solana-keygen` and `dirs`).



```
