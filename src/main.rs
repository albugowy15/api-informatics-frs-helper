use axum::{Router, routing::get, routing::post, response::Json};
use serde::Deserialize;
use serde_json::{Value, json};

async fn hello_world() -> &'static str {
    "Hello World hey"
}

async fn json() -> Json<Value> {
    Json(json!({"data": 42}))
}

#[derive(Deserialize)]
struct RequestBody {
    name: String,
    age: u16
}

async fn json_with_data(Json(payload): Json<RequestBody>) -> Json<Value> {
    Json(json!({
        "message": format!("Get a request from {}", payload.name),
        "data": {
            "age": payload.age
        }
    }))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(json))
        .route("/json/data", post(json_with_data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
