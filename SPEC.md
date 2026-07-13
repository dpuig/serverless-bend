# Spec: Serverless-Bend

## Objective
**Serverless-Bend** is a CLI tool and an HTTP wrapper that allows cloud engineers to easily deploy Bend scripts as serverless endpoints. The goal is to provide a frictionless "Time-to-Value" developer experience, similar to the Serverless Framework. A developer should be able to run `bend-cloud deploy` and instantly get a working Google Cloud Run endpoint without writing Dockerfiles or dealing with infrastructure boilerplate.

## Tech Stack
- **Language:** Rust (Latest Stable Version)
- **CLI Framework:** `clap`
- **HTTP Server:** `axum` with `tokio`
- **Serialization:** `serde_json`
- **Target Infrastructure:** Google Cloud Run (via generated Dockerfile and `gcloud` CLI execution)
- **Target Runtime:** HVM2/Bend (CLI installed in the container)

## Commands
**Build Workspace:**
```bash
cargo build --release
```

**Run CLI Locally (Test):**
```bash
cargo run --bin bend-cloud -- deploy --script hello.bend
```

**Run Wrapper Locally (Test):**
```bash
cargo run --bin bend-wrapper
```

**Test and Lint:**
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Project Structure
We will use a Cargo Workspace to manage the two tightly coupled crates.
```
serverless-bend/
├── Cargo.toml            # Workspace definition
├── docs/                 # Documentation and intent specs
├── cli/                  # The `bend-cloud` CLI tool
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Clap entrypoint, gcloud orchestration
└── wrapper/              # The HTTP runtime wrapper
    ├── Cargo.toml
    └── src/
        └── main.rs       # Axum server, std::process execution of bend
```

## Code Style
Idiomatic Rust using `Result` for error handling and standard `clap`/`axum` paradigms.
```rust
// Example Axum Handler in wrapper/src/main.rs
use axum::{Json, response::IntoResponse};
use serde_json::{Value, json};
use std::process::Command;

pub async fn execute_bend_handler(Json(payload): Json<Value>) -> impl IntoResponse {
    // 1. Serialize payload to string/format Bend expects
    // 2. Execute `bend run-c`
    let output = Command::new("bend")
        .arg("run-c")
        .arg("script.bend")
        .output()
        .expect("Failed to execute Bend");
        
    // 3. Return results
    Json(json!({ "result": String::from_utf8_lossy(&output.stdout).to_string() }))
}
```

## Testing Strategy
- **Framework:** standard Rust `cargo test`.
- **Unit Tests:** Inside the `cli` and `wrapper` crates to test argument parsing and JSON transformation logic.
- **Integration Tests:** A simple `tests/` directory at the workspace level that starts the `wrapper` process and hits it with a local HTTP request (mocking the `bend` binary).

## Boundaries
- **Always:** Use `cargo clippy` and `cargo fmt` before committing. Handle all `Result` types explicitly (no `.unwrap()` in production paths).
- **Ask first:** Before adding large third-party crates (to keep compile times and binary size small). Before changing the Cloud Run deployment strategy.
- **Never:** Never silently swallow errors from the `bend` process execution—these must be bubbled up to the HTTP response for visibility.

## Success Criteria
- [ ] A user can write a `hello.bend` file.
- [ ] Running `bend-cloud deploy hello.bend` successfully generates a Dockerfile, builds the container, and deploys it to Google Cloud Run.
- [ ] The user receives a URL that, when sent a POST request with JSON, successfully executes the Bend script via the Rust wrapper and returns the result.

## Open Questions
- [x] **Resolved:** Incoming JSON data will be passed into the Bend script as a stringified command-line argument for the MVP (e.g., `bend run-c script.bend "{\"key\":\"value\"}"`).
