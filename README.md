<img src="https://nethermindeth.github.io/ice-nine/assets/ice-nine-logo.png" width="300px"/>

# Nine - Nethermind Intelligent Node Environment

A flexible framework for building a distributed network of AI agents that work everywhere (STD, WASM, TEE) with a dynamic interface and hot-swappable components.

[Discord](https://discord.gg/sXCEBQMkyZ)

## Overview

### Project Structure

The project is built using Rust (full-stack) and organized as a workspace consisting of two major groups:

- **`substance/`** - The core components of the system, responsible for interaction.
- **`particles/`** - Plugins for the system that enable additional functionalities.
- **`examples/`** - Usage examples of the framework.

###  Use cases

The following cases will have a minimal implementation, and they will be used to track the progress of the framework and its flexibility in building such systems.

- ☑️ **Chatbots** - AI-driven natural language chatbots for customer support, virtual assistants, and automation.
- ⬜ **AI-governed blockchains (ChaosChain)** - Self-regulating and intelligent blockchain ecosystems with automated decision-making.
- ⬜ **Personal AI Assistant with dynamic UI** - AI that generates adaptive and context-aware user interfaces on demand.
- ☑️ **AI-powered trading bots** - Autonomous financial agents for high-frequency trading and portfolio management.
- ⬜ **Intelligent email assistant** - AI for reading, summarizing, filtering, and responding to emails autonomously.
- ⬜ **Interactivity in home appliances** - AI-powered automation for home appliances, making them responsive and adaptive.
- ⬜ **On-demand observability and awareness in DevOps** - AI-driven insights, predictive monitoring, and automated issue detection in IT systems.
- ⬜ **AI-powered developer tools** - AI agents assisting with code generation, debugging, and software optimization.
- ⬜ **Autonomous research agent** - Self-learning AI for data analysis, knowledge discovery, and hypothesis testing.

Status: ⬜ Not started | ☑️ In Progress | ✅ Completed

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
