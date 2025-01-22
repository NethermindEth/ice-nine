# UI9

**State** is a core element of DUI (distributed UI), it's represented by implementation of the `Flow` trait.

Three types of messages:

- **Event** - changes the state from the agent.
- **Action** - control actions to influence the state.
- **Trace** - a logging message (used for debugging or profiling purposes) | separate layer?

Components that developer should use:

- **Tracer** - a handle to produce events and subscribe to actions.
