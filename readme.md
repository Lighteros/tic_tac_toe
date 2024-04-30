# Solana Tic Tac Toe

Solana Tic Tac Toe is a decentralized application (dApp) running on the Solana blockchain. It leverages the power of smart contracts to create a trustless, secure, and fun Tic Tac Toe game where users can play against each other directly from their wallets. Built with the Anchor framework.

## Features

- **Decentralized Game Logic**: All game logic is executed on-chain, ensuring fairness and transparency.
- **Program Derived Accounts (PDAs)**: Uses PDAs to securely manage game sessions between two players.
- **Open Game Sessions**: Allows any user to initiate or join an existing game session.
- **Player Role Assignment**: Dynamically assigns player roles ("X" or "O") based on wallet addresses.

## Getting Started

### Prerequisites

- Install Rust: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
- Install Solana CLI: Follow the setup guide on the [Solana documentation](https://docs.solana.com/cli/install-solana-cli-tools).
- Install Anchor: Ensure you have Anchor installed by following the [Anchor installation guide](https://www.anchor-lang.com/docs/installation).
- Node.js and yarn: Required for running scripts and tests. Go and install from [yarnpkg.com](https://yarnpkg.com/getting-started/install).

### Installation

1. **Clone the Repository**

   ```sh
   git clone https://github.com/dskydiver/tic_tac_toe.git
   cd tic_tac_toe
   ```

2. **Install Dependencies**

   ```sh
   yarn
   ```

3. **Build the Smart Contract**

   ```sh
   anchor build
   ```

4. **Test the Smart Contract**
   ```sh
   anchor test
   ```
