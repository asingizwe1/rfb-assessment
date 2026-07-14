## STRUCTURE
src/
├── main.rs        ← entry point, just parses CLI and dispatches
├── cli.rs         ← defines all commands and arguments using clap
├── rpc.rs         ← reusable RPC client (the thing that talks to Bitcoin Core)
├── config.rs      ← reads credentials from env vars / config file / flags
├── error.rs       ← your custom error type
└── commands/
    ├── blockchain.rs  ← blockchain-info command logic
    ├── wallet.rs      ← wallet-info and balance command logic
    └── address.rs     ← new-address command logic

## OVERALL ARCHITECTURE
               main.rs
                   │
                   ▼
               cli.rs
                   │
                   ▼
            match command
      ┌────────┼──────────┐
      ▼        ▼          ▼
 blockchain  wallet    address
      │        │          │
      └────────┼──────────┘
               ▼
            rpc.rs
               │
               ▼
      Bitcoin Core JSON-RPC

## MAIN FLOW
Cargo.toml

↓

cli.rs

↓

config.rs

↓

rpc.rs

↓

commands/

↓

main.rs

## STRUCTURAL FLOW OF main.rs
Parse CLI arguments

↓

Load configuration

↓

Create RPC client

↓

Match the selected command

↓

Call the appropriate command module