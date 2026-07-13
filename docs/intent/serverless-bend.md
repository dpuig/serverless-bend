# Serverless-Bend Intent

- **Outcome:** A unified repository containing `Serverless-Bend`: a CLI tool (`bend-cloud`) and a Rust HTTP wrapper that packages and deploys Bend scripts to Google Cloud Run.
- **User:** Cloud engineers and the open-source community who want to run heavy parallel compute without managing clusters.
- **Why now:** Bend's massive GPU parallelism is powerful, but there's too much friction to deploy it. Removing that friction unlocks it for the community.
- **Success:** A developer can run `bend-cloud deploy` on a visually impressive "Hello World" Bend script and instantly get a working HTTP endpoint, feeling exactly like the Serverless Framework.
- **Constraint:** Frictionless Time-to-Value. The developer experience must be so magical and easy that it drives immediate community engagement and virality.
- **Out of scope:** AWS Lambda (Cloud Run only for MVP), complex API routing, database connectors, and over-optimizing the Rust serialization bridge before we prove people want the abstraction.
