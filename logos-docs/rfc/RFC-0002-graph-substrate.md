# RFC-0002: Graph Substrate

*   **Status**: Draft
*   **Category**: Architecture

## 1. Overview

The LOGOS Graph Substrate is the primary execution structure.
All computation is expressed as interactions between Nodes connected by Edges.

## 2. Nodes

*   Authored in Rust
*   Compiled to WASM (WASI)
*   Addressed by cryptographic hash
*   Immutable after admission

Nodes contain pure logic only. They do not own persistent state.

## 3. Edges

Edges define:
*   Typed dataflow
*   Execution ordering
*   State propagation across the graph lifecycle

Edges carry not just data, but state context, enabling durable execution and crash recovery.

## 4. Capabilities & IO

Nodes operate under explicit capabilities:
*   Network
*   Time
*   Randomness
*   Storage (if granted)

Capabilities are granted per-node and enforced by WASI.

## 5. Execution Semantics

Execution proceeds by:
*   Resolving graph topology
*   Validating contracts
*   Executing nodes deterministically
*   Recording outcomes in the Ledger
