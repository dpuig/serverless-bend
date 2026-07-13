<div align="center">
  <h1>Serverless-Bend</h1>
  <p><strong>A frictionless CLI framework to instantly deploy <a href="https://github.com/HigherOrderCO/Bend">Bend</a> scripts as serverless HTTP endpoints.</strong></p>
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/rust-latest-orange.svg)](https://www.rust-lang.org/)
</div>

<br />

Bend gives you massive, implicit GPU parallelism. **Serverless-Bend** lets you deploy that power to the cloud in seconds. 

No managing Dockerfiles. No configuring CI/CD pipelines. No writing boilerplate HTTP wrappers. Just write your `script.bend`, run `bend-cloud deploy`, and instantly get a live, highly scalable API endpoint powered by Google Cloud Run.

---

## The 60-Second "Hello World"

**1. Write your Bend script (`hello.bend`)**
```python
def main(arg):
  return arg
```

**2. Deploy it to the cloud**
By default, this deploys to Google Cloud Run:
```bash
$ bend-cloud deploy hello.bend
Target script to deploy: hello.bend
Target provider: gcp
Successfully generated Docker environment...
Deploying to Google Cloud Run...
Deploy successful! Endpoint URL: https://serverless-bend-endpoint-abc123.a.run.app
```

**Multi-Cloud Support**
You can deploy to AWS (Lambda) or Azure (Container Apps) by specifying a provider:
```bash
$ bend-cloud deploy hello.bend --provider aws
$ bend-cloud deploy hello.bend --provider azure
```

**3. Hit your new massively parallel API**
```bash
$ curl -X POST -H "Content-Type: application/json" -d '{"data": 99}' https://serverless-bend-endpoint-abc123.a.run.app/execute

{"result":"Result: \"{\\\"data\\\":99}\"","status":"success"}
```

## How it Works

Under the hood, `Serverless-Bend` is heavily inspired by the Serverless Framework:
1. **The CLI (`bend-cloud`)** dynamically scaffolds a production-ready Dockerfile for your exact script.
2. **The Wrapper** transparently bundles your script with a high-performance Rust (`axum` + `tokio`) HTTP server.
3. **The Orchestrator** pushes the container to Google Cloud Run, automatically abstracting away the infrastructure.

Your Bend scripts stay completely pure. We handle the JSON serialization, the HTTP layer, and the network I/O.

## Installation

Ensure you have Rust and the Google Cloud SDK (`gcloud`) installed and configured.

```bash
git clone https://github.com/dpuig/serverless-bend.git
cd serverless-bend
cargo install --path cli
```

## Contributing

We built this MVP to prove how easy it *should* be to deploy Bend code. Now we want to make it unstoppable, and this is an open-source project meant for the community. 

Here are some areas we'd love help with:
- **Cloudflare Workers & Vercel:** Bring the deployment magic to other serverless platforms.
- **Zero-Copy Serialization:** Optimize the data bridge between the Rust HTTP wrapper and the Bend runtime.
- **Advanced JSON Parsing:** Help build or expose Bend primitives for natively parsing deeply nested HTTP payloads.

Check out the [Open Issues](https://github.com/dpuig/serverless-bend/issues) or drop a PR!

## License

MIT
