use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bend-cloud")]
#[command(about = "Serverless deployment CLI for Bend", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy a Bend script to the cloud
    Deploy {
        /// The path to the .bend script to deploy
        script: String,
        
        /// The cloud provider to deploy to: gcp, aws, or azure
        #[arg(short, long, default_value = "gcp")]
        provider: String,
    },
}

use std::fs;
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Deploy { script, provider } => {
            println!("Target script to deploy: {}", script);
            println!("Target provider: {}", provider);
            
            let build_dir = Path::new(".bend-cloud");
            fs::create_dir_all(build_dir).expect("Failed to create build directory");
            
            // Copy the target script
            fs::copy(script, build_dir.join("script.bend")).expect("Failed to copy script");
            
            // Write wrapper Cargo.toml
            let cargo_toml = r#"[package]
name = "bend-wrapper"
version = "0.1.0"
edition = "2024"

[workspace]

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"#;
            fs::write(build_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");
            
            // Write wrapper main.rs
            let src_dir = build_dir.join("src");
            fs::create_dir_all(&src_dir).expect("Failed to create src directory");
            let main_rs = r#"use axum::{routing::post, Router, Json, response::IntoResponse};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::process::Command;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/execute", post(execute_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn execute_handler(Json(payload): Json<Value>) -> impl IntoResponse {
    let bend_term = if let Some(data) = payload.get("data") {
        if data.is_number() {
            data.to_string()
        } else if let Some(s) = data.as_str() {
            format!("{:?}", s)
        } else {
            format!("{:?}", data.to_string())
        }
    } else {
        format!("{:?}", payload.to_string())
    };
    let output = Command::new("bend").arg("run-c").arg("script.bend").arg(&bend_term).output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Json(json!({"status": "success", "result": String::from_utf8_lossy(&out.stdout).to_string().trim()}))
            } else {
                Json(json!({"status": "error", "error": String::from_utf8_lossy(&out.stderr).to_string().trim()}))
            }
        },
        Err(e) => Json(json!({"status": "error", "error": format!("Failed to execute bend: {}", e)}))
    }
}
"#;
            fs::write(src_dir.join("main.rs"), main_rs).expect("Failed to write main.rs");
            
            // Write Dockerfile
            let dockerfile = r#"FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=public.ecr.aws/awsguru/aws-lambda-adapter:0.9.0 /lambda-adapter /opt/extensions/lambda-adapter
RUN cargo install hvm
RUN cargo install bend-lang
COPY --from=builder /app/target/release/bend-wrapper /usr/local/bin/bend-wrapper
COPY script.bend /app/script.bend
ENV PORT=8080
EXPOSE 8080
CMD ["bend-wrapper"]
"#;
            fs::write(build_dir.join("Dockerfile"), dockerfile).expect("Failed to write Dockerfile");
            println!("Successfully generated Docker environment in .bend-cloud/");
            
            match provider.as_str() {
                "gcp" => deploy_gcp(build_dir),
                "azure" => deploy_azure(build_dir),
                "aws" => deploy_aws(build_dir),
                _ => eprintln!("Unknown provider: {}. Use gcp, aws, or azure.", provider),
            }
        }
    }
}

fn deploy_gcp(build_dir: &Path) {
    println!("Deploying to Google Cloud Run...");
    let output = std::process::Command::new("gcloud")
        .arg("run")
        .arg("deploy")
        .arg("serverless-bend-endpoint")
        .arg("--source")
        .arg(build_dir.to_str().unwrap())
        .arg("--allow-unauthenticated")
        .arg("--region")
        .arg("us-central1")
        .arg("--format")
        .arg("json")
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    if let Some(url) = json.get("status").and_then(|s| s.get("url")).and_then(|u| u.as_str()) {
                        println!("Deploy successful! Endpoint URL: {}", url);
                    } else {
                        println!("Deploy successful, but failed to parse URL from output.");
                    }
                } else {
                    println!("Deploy successful!");
                }
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("Deployment failed:\n{}", stderr);
            }
        },
        Err(e) => {
            eprintln!("Failed to execute gcloud (is it installed and configured?): {}", e);
        }
    }
}

fn deploy_azure(build_dir: &Path) {
    println!("Deploying to Azure Container Apps...");
    let output = std::process::Command::new("az")
        .arg("containerapp")
        .arg("up")
        .arg("--name")
        .arg("serverless-bend-endpoint")
        .arg("--source")
        .arg(build_dir.to_str().unwrap())
        .arg("--ingress")
        .arg("external")
        .arg("--target-port")
        .arg("8080")
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                println!("Deploy successful! Check your Azure portal for the Container App URL.");
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("Deployment failed:\n{}", stderr);
            }
        },
        Err(e) => {
            eprintln!("Failed to execute az (is the Azure CLI installed?): {}", e);
        }
    }
}

fn deploy_aws(build_dir: &Path) {
    println!("Generating AWS SAM template...");
    let template = r#"AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Resources:
  BendFunction:
    Type: AWS::Serverless::Function
    Properties:
      PackageType: Image
      MemorySize: 2048
      Timeout: 30
      Events:
        ApiEvent:
          Type: HttpApi
    Metadata:
      Dockerfile: Dockerfile
      DockerContext: .
      DockerTag: bend-func
"#;
    std::fs::write(build_dir.join("template.yaml"), template).expect("Failed to write AWS SAM template");
    
    println!("Successfully generated AWS SAM template in .bend-cloud/template.yaml");
    println!("To deploy to AWS, run the following commands:");
    println!("  cd .bend-cloud");
    println!("  sam build");
    println!("  sam deploy --guided");
}
