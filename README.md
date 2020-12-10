# soko-test

Both my first Rust and my first Piston project, an attempt at prototyping a turn-based Sokoban-styled puzzle game with Rust and the [Piston game engine](https://www.piston.rs).  

## Current Thoughts

 - **12.09.20** -- Rust requires an alternative approach to many problems and I'm not confident in how rustic or optimal my solutions are. However, Piston more than makes up for the learning curve through solid examples, thorough documentation, and overall understandable and traceable code and I highly enjoy the experience using the game engine so far. I will definitely be seeing the project through and considering other projects to use Piston for.

## Requirements

- To Run -- OpenGL v3.2 or better.
- To Compile -- Rust, Cargo package manager, Possible C libraries as required by Piston.

## Changelog

- **v0.1.0** -- Windowing and Movement Prototyping  
- **v0.1.1** -- First x86-64-pc-windows binary  
- **v0.2.0** -- Entity Interactions: Player-Wall Collision, Player-Box Movement, Box-Goal Notification
- **v0.3.0** -- (WIP) Level Progression, Level Loading

## Running With Binary

- Donwload the binary for your system from your chosen release.
- Run the binary, all binaries should run natively.

**Current Dev Platform Focus:** *x86_64-pc-windows*

## Running Without Binary

**Requires both Rust and the Cargo package manager and possible C libraries as required by Piston.**

```bash
cargo run
```

## License

This project is open-sourced under the [MIT](https://choosealicense.com/licenses/mit/) license, full text in LICENSE.