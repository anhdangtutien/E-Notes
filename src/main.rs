mod db;
mod handlers;
mod http;
mod models;

use std::env;
use std::fs;
use std::path::Path;

use db::Db;
use http::Server;

fn main() -> std::io::Result<()> {
    println!("--------------------------------------------------");
    println!("       E-Notes: English Learning & Notebook       ");
    println!("               Built 100% with Rust               ");
    println!("--------------------------------------------------");

    // Ensure static directory exists
    if !Path::new("static").exists() {
        let _ = fs::create_dir_all("static");
    }

    let db_path = env::var("ENOTES_DB").unwrap_or_else(|_| "enotes.db".to_string());
    let db = match Db::open(&db_path) {
        Ok(d) => {
            println!("✓ SQLite database connected: {}", db_path);
            d
        }
        Err(e) => {
            eprintln!("✗ Failed to open database: {}", e);
            std::process::exit(1);
        }
    };

    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str.parse().unwrap_or(8080);
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let server = Server::new(db)
        // Sessions API
        .route("GET", "/api/sessions", handlers::list_sessions)
        .route("POST", "/api/sessions", handlers::create_session)
        .route("GET", "/api/sessions/{id}", handlers::get_session)
        .route("PUT", "/api/sessions/{id}", handlers::update_session)
        .route("DELETE", "/api/sessions/{id}", handlers::delete_session)
        // Vocabulary API
        .route("GET", "/api/sessions/{id}/vocabularies", handlers::list_vocab_by_session)
        .route("POST", "/api/vocabularies", handlers::create_vocab)
        .route("PUT", "/api/vocabularies/{id}", handlers::update_vocab)
        .route("DELETE", "/api/vocabularies/{id}", handlers::delete_vocab)
        // Grammar API
        .route("GET", "/api/sessions/{id}/grammars", handlers::list_grammar_by_session)
        .route("POST", "/api/grammars", handlers::create_grammar)
        .route("PUT", "/api/grammars/{id}", handlers::update_grammar)
        .route("DELETE", "/api/grammars/{id}", handlers::delete_grammar)
        // Stats & Search & Backup
        .route("GET", "/api/stats", handlers::get_stats)
        .route("GET", "/api/search", handlers::search)
        .route("GET", "/api/backup/export", handlers::export_backup)
        .route("POST", "/api/backup/import", handlers::import_backup)
        // Static assets & Web UI
        .route("GET", "/", handlers::serve_static)
        .route("GET", "/index.html", handlers::serve_static)
        .route("GET", "/style.css", handlers::serve_static)
        .route("GET", "/app.js", handlers::serve_static)
        .route("GET", "/static/index.html", handlers::serve_static)
        .route("GET", "/static/style.css", handlers::serve_static)
        .route("GET", "/static/app.js", handlers::serve_static);

    server.run_with_fallback(&host, port)
}
