<img src="https://nethermindeth.github.io/ice-nine/assets/ice-nine-logo.png" width="300px"/>

# Ice-Nine

AI agents that work everywhere.

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
let substance = Substance::new();
let mut addr: SubstanceLink = substance.spawn().equip();
addr.add_particle::<OpenAIParticle>()?;
addr.add_particle::<TelegramParticle>()?;
```
