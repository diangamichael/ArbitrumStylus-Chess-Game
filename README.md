# ArbitrumStylus-Chess-Game

## Overview

This project is a smart contract implementation of a chess game using Rust and Arbitrum Stylus. The game allows two players to engage in a classic game of chess on the blockchain, leveraging the benefits of decentralized applications (dApps) such as transparency, security, and immutability.

## Features

- **Smart Contract**: The chess game is built as a smart contract, ensuring that all game moves are securely recorded on the blockchain.
- **Move Tracking**: All moves are stored and can be accessed for auditing purposes.
- **Access Control**: The game enforces access controls to ensure that only the designated players can make moves.
- **Game Finalization**: The smart contract includes methods to finalize the game and declare a winner.

## Technologies Used

- **Rust**: The primary programming language used for developing the smart contract.
- **Arbitrum Stylus**: A platform for building Ethereum-compatible dApps with improved scalability and performance.
- **Serde**: A Rust library for serializing and deserializing data formats, used for handling game state.

## Getting Started

### Prerequisites

- Rust installed on your machine. Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
- Cargo, the Rust package manager.
- The `wasm32-unknown-unknown` target added to your Rust toolchain:

  ```bash
  rustup target add wasm32-unknown-unknown
