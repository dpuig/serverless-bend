# Implementation Plan: Serverless-Bend

## 1. Major Components & Dependencies
- **Workspace:** Cargo workspace orchestrating the project.
- **HTTP Wrapper (`wrapper` crate):** 
  - Dependencies: `axum`, `tokio`, `serde`, `serde_json`.
  - Responsibilities: Listen on a port, parse JSON, execute `bend run-c` using `std::process`, return JSON response.
- **CLI (`cli` crate):**
  - Dependencies: `clap`, `fs`, `process`.
  - Responsibilities: Parse `deploy <script>` command, generate a Dockerfile bundling the wrapper and script, execute `gcloud run deploy`.

## 2. Implementation Order
1. **Repository Setup:** Scaffold Cargo workspace and both crates.
2. **HTTP Wrapper Core:** Build the Axum server, implement the endpoint that takes JSON, formats the command-line argument, shells out to `bend run-c`, and captures the output.
3. **HTTP Wrapper Verification:** Test the wrapper locally against a mock script.
4. **CLI Core:** Implement `bend-cloud deploy <script>`.
5. **CLI Docker generation:** Programmatically generate the Dockerfile that installs Bend, copies the wrapper, copies the script, and exposes the port.
6. **Deployment Orchestration:** Shell out to `gcloud run deploy`.

## 3. Risks & Mitigations
- **Risk:** Bend CLI outputs errors or warnings alongside stdout, breaking JSON responses.
  - **Mitigation:** The wrapper must strictly capture only `stdout` and handle `stderr` separately, logging it rather than returning it to the user in the success response payload.
- **Risk:** Dockerizing the Bend compiler/runtime is slow or large.
  - **Mitigation:** Use a lightweight base image (like Debian or Ubuntu) and ensure we only install exactly what Bend needs (Rust toolchain, C compiler if needed).

## 4. Parallel vs Sequential Work
- **Sequential:** The Workspace setup must happen first. The Wrapper must be built before the Dockerfile generation can be tested (since the Dockerfile packages the wrapper).
- **Parallel:** The `clap` CLI argument parsing and the `gcloud` orchestration script could theoretically be built independently of the Wrapper, but given they are tightly coupled, we will implement sequentially.

## 5. Verification Checkpoints
- **Checkpoint 1:** `cargo run --bin bend-wrapper` can successfully take a `curl` POST request and return a JSON payload after executing a local dummy `.bend` file.
- **Checkpoint 2:** `cargo run --bin bend-cloud -- deploy hello.bend` successfully creates a Dockerfile containing everything needed.
- **Checkpoint 3:** A fully deployed Cloud Run endpoint successfully responds to the same `curl` POST request as Checkpoint 1.
