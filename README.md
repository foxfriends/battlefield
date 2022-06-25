# Battlefield

Some sort of turn-based grid battle game, to feature **lots** of customization,
allowing players to build their own turn based strategy game.

## Project Structure

This project is split into a multiple crates, each achieving one specific task:
*   [`battlefield-core`](./battlefield-core) implements the game engine and core data structures
*   [`battlefield-server`](./battlefield-server) implements a server that exposes core functionality via a web API
