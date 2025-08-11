# pump69-program

Solana Anchor program with LeagueFactory and League contracts for pump69 platform

## Overview

This repository contains the Solana Anchor program for the pump69 platform, featuring smart contracts for managing leagues and league factories on the Solana blockchain.

## Contracts

### LeagueFactory
The LeagueFactory contract handles the creation and management of league instances.

### League
The League contract manages individual league functionality, including participant management, scoring, and rewards distribution.

## Setup Instructions

### Prerequisites
- [Rust](https://rustup.rs/) installed
- [Solana CLI tools](https://docs.solana.com/cli/install-solana-cli-tools) installed
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) installed
- [Node.js](https://nodejs.org/) and [Yarn](https://yarnpkg.com/) installed

### Installation

1. Clone the repository:
```bash
git clone https://github.com/notoxpengz/pump69-program.git
cd pump69-program
```

2. Initialize the Anchor project:
```bash
anchor init . --force
```

3. Set Solana cluster to mainnet-beta:
```bash
solana config set --url https://api.mainnet-beta.solana.com
```

4. Install dependencies:
```bash
yarn install
```

5. Build the program:
```bash
anchor build
```

## Project Structure

```
pump69-program/
├── programs/
│   └── pump69-program/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── league_factory.rs
│       │   └── league.rs
│       └── Cargo.toml
├── tests/
├── migrations/
├── app/
├── Anchor.toml
└── package.json
```

## Configuration

The project is configured to use the Solana mainnet-beta cluster. Update `Anchor.toml` as needed:

```toml
[features]
seeds = false
skip-lint = false
[programs.mainnet-beta]
pump69_program = "YOUR_PROGRAM_ID_HERE"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "mainnet-beta"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

## Testing

Run tests with:
```bash
anchor test
```

## Deployment

1. Ensure you have SOL in your wallet for deployment fees
2. Deploy to mainnet-beta:
```bash
anchor deploy --provider.cluster mainnet-beta
```

## Usage

After deployment, interact with the contracts using the generated TypeScript client in the `app/` directory or through direct RPC calls.

## Contributing

Please read our contributing guidelines before submitting pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
