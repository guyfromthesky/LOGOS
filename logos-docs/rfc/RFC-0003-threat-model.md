# RFC-0003: Threat Model

*   **Status**: Draft
*   **Category**: Security

## 1. Assumptions

*   **AI agents are malicious or hallucinating by default.**
*   **The Supervisor is trusted.**
*   **Human intervention is rare and explicit.**

## 2. Adversary Model

**Attackers may:**
*   Generate malicious Rust code
*   Attempt graph poisoning
*   Exploit undefined behavior

**Attackers may NOT:**
*   Break cryptographic hashes
*   Escalate WASI capabilities
*   Mutate locked nodes

## 3. Security Guarantees

*   **Compile-time safety via Rust**
*   **Runtime isolation via WASM**
*   **Architectural constraints via graph validation**
