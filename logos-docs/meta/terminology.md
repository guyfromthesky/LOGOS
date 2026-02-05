# Terminology

## Core Entities

### Node
An atomic unit of computation, written in Rust and compiled to WebAssembly (WASI). It is identified by the cryptographic hash of its compiled binary and is practically immutable.

### Edge
A typed dataflow and state propagation contract between two nodes. It ensures type safety and carries state context across the graph lifecycle.

### Ledger
An append-only, Merkle-hashed governance log that records all system events, including node admission, edge creation, and capability grants.

### Supervisor
The trusted root execution engine responsible for enforcing protocol rules, sandboxing nodes, and validating graph invariants.

## Layers

### Sediment Layer
A mutable sandbox environment where autonomous AI agents can propose, execute, and iterate on logic. Code in this layer is constrained and not yet fully trusted.

### Bedrock Layer
A locked, immutable execution layer for human-verified logic. Nodes in this layer are stable and require explicit migration tokens to be modified (replaced).

## Governance

### Migration Token
A time-limited, scope-restricted token issued by a human operator that allows for the deprecation or replacement of nodes in the Bedrock layer.
