# Design Invariants

These invariants are enforced by the LOGOS protocol by construction and cannot be bypassed.

1.  **Immutability of Nodes**
    Nodes cannot be modified after creation. All changes must result in a new node with a new identity (hash).

2.  **Compile-Time Type Safety**
    Edges must be type-compatible at compile time. The output type of a source node must strictly match the input type of the destination node.

3.  **Explicit Execution Paths**
    All execution paths must be explicitly encoded in the graph structure. No implicit control flow or side-channel communication is allowed.

4.  **Append-Only Ledger**
    The governance ledger is strictly append-only. History cannot be rewritten, only added to.

5.  **Capability Gating**
    Capabilities (network, disk, etc.) are non-transitive and must be explicitly granted to each node.

6.  **Human Override Constraint**
    AI agents cannot modify or deprecate nodes in the Bedrock layer without a valid, human-issued Migration Token.
