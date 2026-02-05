# RFC-0004: Ledger & Governance

*   **Status**: Draft
*   **Category**: Governance

## 1. Ledger Design

The Ledger is:
*   Append-only
*   Merkle-hashed
*   Locally verifiable

It records:
*   Node admission
*   Edge creation
*   Capability grants
*   Migration events

## 2. Practical Immutability

Nodes are never modified. They may only be:
*   Deprecated
*   Forked
*   Replaced via migration

## 3. Migration Tokens

Migration Tokens:
*   Issued by humans
*   Time-limited
*   Scope-restricted

They enable emergency intervention only.
