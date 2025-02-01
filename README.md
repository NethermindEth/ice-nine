<img src="https://nethermindeth.github.io/ice-nine/assets/ice-nine-logo.png" width="300px"/>

# Nine - Nethermind Intelligent Node Environment

A flexible framework for building a distributed network of AI agents that work everywhere (STD, WASM, TEE) with a dynamic interface and hot-swappable components.

[Discord](https://discord.gg/sXCEBQMkyZ)

## Overview

###  Cases

- Personal assistants (Mechs)
- AI-driven trading bots
- Chatbots

###  Features (goals)

- Built on Rust and implemented as hybrid actor-state machines.
- Supports various LLMs, tools, and extensibility.
- Hot model swapping without restarting.
- Real-time configuration adjustment.
- Distributed agents, the ability to run components on different machines.
- Provides a dynamic user interface (*UI9*) that is automatically generated for interacting with a network of agents.

## Usage

An agent is a `substance` that assembles from components (`particles`). Connections automatically form between them, bringing the agent to life:

```rust
let mut substance = Substance::arise();
substance.add_particle::<OpenAIParticle>()?;
substance.add_particle::<TelegramParticle>()?;
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/NethermindEth/ice-nine/blob/trunk/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, shall be licensed as MIT, without any additional
terms or conditions.
