# Nexus zkVM Platform — Documentation Hub

Welcome to the Nexus zkVM Platform documentation. Here you are able to find integration SDKs, the API Reference, cryptographic specifics, and deployment runbooks for on-prem infrastructures.

## Table of Contents
1. [Platform Architecture](#architecture)
2. [Cryptographic Assumptions](#crypto)
3. [API Reference v1](#api)
4. [Deployment Runbooks](#deployment)

---

<a name="architecture"></a>
## 1. Platform Architecture
The zkVM platform leverages a distributed mesh of specialized nodes to scale proof generation reliably:
- **Compiler**: Ahead-of-time compilation of generic code down to constrained Algebraic Intermediate Representations.
- **Prover Cluster**: Horizontally scaled GPU instances processing STARK generations in parallel using Job queues (Kafka).
- **Verifier API**: Lightweight border nodes validating the mathematical soundness of STARK proofs recursively.

<a name="crypto"></a>
## 2. Cryptographic Assumptions
The system depends on the following properties for achieving **Soundness**, **Zero-Knowledge**, and **Completeness**:
- **Hash Functions**: We use **Poseidon** inside the algebraic circuit for transcript hashing due to its low arithmetization cost. Outside the circuit (e.g. Merkle Tree roots) we rely on **Blake3** for speed. 
- **FRI**: The Fast Reed-Solomon Interactive Oracle Proof of Proximity provides post-quantum security without requiring a Trusted Setup.
- **Fiat-Shamir Heuristic**: Used to transform the interactive STARK protocol into a non-interactive proof.

> **Warning:** A break in the Poseidon hash function collision-resistance would compromise the soundness of the proofs. 

<a name="api"></a>
## 3. API Reference (Enterprise Version)
Base URL: `https://api.zkvm.enterprise.local/v1`

- **POST /compile**: Accepts `{ "source": "..." }`, returns a bytecode pointer.
- **POST /execute**: Accepts `{ "bytecodeId": "...", "privateInputs": {} }`, returns `{ "sessionId": "..." }`.
- **POST /prove**: Accepts `{ "sessionId": "..." }`, asynchronously triggers proof cluster. Returns `{ "jobId": "..." }`.
- **POST /verify**: Accepts `{ "proofData": "..." }`, returns `{ "valid": true|false }` in <200ms.

<a name="deployment"></a>
## 4. Deployment Runbooks (Self-Hosted)
For enterprises hosting on AWS:
1. Navigate to `infrastructure/terraform/` and run `terraform apply` to provision EKS, IAM, and S3.
2. Initialize `kubectl` config from the generated EKS cluster.
3. Deploy the Helm charts: `helm install zkvm-cluster ./infrastructure/helm/`
4. Register the initial Administrator API keys via the Verifier's KMS configuration manager.
