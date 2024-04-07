> ⚠️ This project has once again sat for too long, and is getting archived.
>
> It will be revived once again as [Paper Wars](https://github.com/foxfriends/paper-wars), having relocated the previously defunct attempt.

# Battlefield

Some sort of turn-based grid battle game, to feature **lots** of customization,
allowing players to build their own turn based strategy game.

## Project Structure

This project is split into a multiple crates, each achieving one specific task:
*   [`battlefield-core`](./battlefield-core) implements the game engine and core data structures
*   [`battlefield-server`](./battlefield-server) implements a server that exposes core functionality via a web API
*   [`battlefield-web`](./battlefield-web) implements a webapp frontend to the server
*   [`battlefield-api`](./battlefield-api) implements shared data types for communication between web and server

Additionally the `maps`, `modules`, and `scenarios` include game runtime code and data files, which configure
the server with the components needed for the game engine.
