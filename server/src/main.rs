use axum::{
    Router,
    routing::{post},
};
use smol::eval; // Use the function from your core crate
use tower_http::services::ServeDir;

// This handler will run code sent from the browser
async fn handle_run_api(code: String) -> String {
    // Run the code using your core interpreter
    eval(&code)
}

#[tokio::main]
async fn main() {
    // Build your application router
    let app = Router::new()
        // API endpoint to run code
        .route("/api/run", post(handle_run_api))
        // Serve the static files (HTML/JS/WASM) from the 'www' directory
        .fallback_service(ServeDir::new("www"));

    println!("Server listening on http://127.0.0.1:3000");

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
