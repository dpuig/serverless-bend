# Tasks: Serverless-Bend

- [x] **Task: Scaffold Cargo Workspace**
  - **Acceptance:** A Cargo workspace exists containing two empty binary crates (`cli` and `wrapper`).
  - **Verify:** `cargo build` at the root succeeds without errors.
  - **Files:** `Cargo.toml`, `cli/Cargo.toml`, `cli/src/main.rs`, `wrapper/Cargo.toml`, `wrapper/src/main.rs`

- [x] **Task: Build Wrapper HTTP Server Skeleton**
  - **Acceptance:** The `wrapper` crate runs an Axum HTTP server on port 8080 with a single POST endpoint `/execute` that accepts and logs JSON.
  - **Verify:** `cargo run --bin bend-wrapper`, then `curl -X POST -H "Content-Type: application/json" -d '{"data": 42}' http://localhost:8080/execute` returns a 200 OK.
  - **Files:** `wrapper/Cargo.toml`, `wrapper/src/main.rs`

- [x] **Task: Implement Wrapper Shell Execution**
  - **Acceptance:** The `/execute` endpoint stringifies the incoming JSON, calls `bend run-c script.bend "<json>"`, captures `stdout`, and returns it as a JSON payload.
  - **Verify:** Create a dummy `script.bend` that echoes its argument. Hitting the endpoint returns the echoed value.
  - **Files:** `wrapper/src/main.rs`

- [x] **Task: Build CLI Argument Parsing**
  - **Acceptance:** The `cli` crate parses the command `bend-cloud deploy <script.bend>` using `clap`.
  - **Verify:** `cargo run --bin bend-cloud -- deploy hello.bend` prints the target script name.
  - **Files:** `cli/Cargo.toml`, `cli/src/main.rs`

- [x] **Task: Implement CLI Dockerfile Generation**
  - **Acceptance:** The CLI programmatically generates a `Dockerfile` in the local directory that installs the Rust toolchain, Bend, copies the wrapper, copies the script, and exposes port 8080.
  - **Verify:** Run the CLI command; inspect the generated `Dockerfile` for correctness.
  - **Files:** `cli/src/main.rs`

- [x] **Task: Implement CLI Google Cloud Orchestration**
  - **Acceptance:** The CLI uses `std::process::Command` to execute `gcloud run deploy` using the generated Dockerfile, parsing the output to display the final URL.
  - **Verify:** Run the CLI with a valid GCP project configured, and verify it successfully deploys to Cloud Run and prints the URL.
  - **Files:** `cli/src/main.rs`
