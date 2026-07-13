use axum::{
    routing::post,
    Router,
    Json,
    response::IntoResponse,
};
use serde_json::{Value, json};
use std::net::SocketAddr;

pub fn app() -> Router {
    Router::new().route("/execute", post(execute_handler))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

use std::process::Command;

async fn execute_handler(Json(payload): Json<Value>) -> impl IntoResponse {
    println!("Received payload: {:?}", payload);
    
    let bend_term = if let Some(data) = payload.get("data") {
        if data.is_number() {
            data.to_string()
        } else {
            format!("{:?}", data.as_str().unwrap_or(&data.to_string()))
        }
    } else {
        format!("{:?}", payload.to_string())
    };

    let output = Command::new("bend")
        .arg("run-c")
        .arg("script.bend")
        .arg(&bend_term)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                Json(json!({
                    "status": "success",
                    "result": stdout.trim()
                }))
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Json(json!({
                    "status": "error",
                    "error": stderr.trim()
                }))
            }
        },
        Err(e) => {
            Json(json!({
                "status": "error",
                "error": format!("Failed to execute bend: {}", e)
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode, header};
    use axum::body::Body;
    use tower::ServiceExt;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_execute_handler() {
        let app = app();

        // Create a dummy script.bend in the current working directory
        std::fs::write("script.bend", "def main(arg):\n  return arg\n").unwrap();

        let req = Request::builder()
            .method("POST")
            .uri("/execute")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"data": 42}"#))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();

        // Clean up the script file
        let _ = std::fs::remove_file("script.bend");

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8_lossy(&body);
        let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        println!("Response JSON: {:?}", json);

        assert_eq!(json.get("status").unwrap(), "success");
        // We can't guarantee what `bend` will return, but it should contain something since we set it up
        assert!(json.get("result").is_some());
    }
}
