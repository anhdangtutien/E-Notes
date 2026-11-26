use std::fs;
use std::path::Path;

use crate::db::Db;
use crate::http::{extract_param, Request, Response};
use crate::models::{
    BackupData, CreateGrammarReq, CreateSessionReq, CreateVocabReq, UpdateGrammarReq,
    UpdateSessionReq, UpdateVocabReq,
};

// --- Sessions ---

pub fn list_sessions(_req: &Request, db: &Db) -> Response {
    match db.get_sessions() {
        Ok(list) => Response::json(&list),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn get_session(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/sessions/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing session id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid session id"),
    };

    match db.get_session_detail(id) {
        Ok(Some(detail)) => Response::json(&detail),
        Ok(None) => Response::not_found("Session not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn create_session(req: &Request, db: &Db) -> Response {
    let payload: CreateSessionReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    if payload.title.trim().is_empty() {
        return Response::bad_request("Title cannot be empty");
    }

    match db.create_session(payload) {
        Ok(session) => Response::json(&session),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn update_session(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/sessions/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing session id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid session id"),
    };

    let payload: UpdateSessionReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    match db.update_session(id, payload) {
        Ok(Some(session)) => Response::json(&session),
        Ok(None) => Response::not_found("Session not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn delete_session(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/sessions/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing session id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid session id"),
    };

    match db.delete_session(id) {
        Ok(true) => Response::json(&serde_json::json!({ "deleted": true, "id": id })),
        Ok(false) => Response::not_found("Session not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

// --- Vocabulary ---

pub fn list_vocab_by_session(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/sessions/{id}/vocabularies", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing session id"),
    };
    let session_id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid session id"),
    };

    match db.get_vocab_by_session(session_id) {
        Ok(list) => Response::json(&list),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn create_vocab(req: &Request, db: &Db) -> Response {
    let payload: CreateVocabReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    if payload.word.trim().is_empty() {
        return Response::bad_request("Word cannot be empty");
    }
    if payload.meaning.trim().is_empty() {
        return Response::bad_request("Meaning cannot be empty");
    }

    match db.create_vocab(payload) {
        Ok(vocab) => Response::json(&vocab),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn update_vocab(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/vocabularies/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing vocabulary id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid vocabulary id"),
    };

    let payload: UpdateVocabReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    match db.update_vocab(id, payload) {
        Ok(Some(vocab)) => Response::json(&vocab),
        Ok(None) => Response::not_found("Vocabulary not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn delete_vocab(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/vocabularies/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing vocabulary id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid vocabulary id"),
    };

    match db.delete_vocab(id) {
        Ok(true) => Response::json(&serde_json::json!({ "deleted": true, "id": id })),
        Ok(false) => Response::not_found("Vocabulary not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

// --- Grammar ---

pub fn list_grammar_by_session(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/sessions/{id}/grammars", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing session id"),
    };
    let session_id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid session id"),
    };

    match db.get_grammar_by_session(session_id) {
        Ok(list) => Response::json(&list),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn create_grammar(req: &Request, db: &Db) -> Response {
    let payload: CreateGrammarReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    if payload.title.trim().is_empty() {
        return Response::bad_request("Grammar title cannot be empty");
    }

    match db.create_grammar(payload) {
        Ok(g) => Response::json(&g),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn update_grammar(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/grammars/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing grammar id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid grammar id"),
    };

    let payload: UpdateGrammarReq = match req.json() {
        Ok(p) => p,
        Err(e) => return Response::bad_request(&format!("Invalid JSON: {}", e)),
    };

    match db.update_grammar(id, payload) {
        Ok(Some(g)) => Response::json(&g),
        Ok(None) => Response::not_found("Grammar item not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn delete_grammar(req: &Request, db: &Db) -> Response {
    let id_str = match extract_param("/api/grammars/{id}", &req.path, "id") {
        Some(s) => s,
        None => return Response::bad_request("Missing grammar id"),
    };
    let id: i64 = match id_str.parse() {
        Ok(i) => i,
        Err(_) => return Response::bad_request("Invalid grammar id"),
    };

    match db.delete_grammar(id) {
        Ok(true) => Response::json(&serde_json::json!({ "deleted": true, "id": id })),
        Ok(false) => Response::not_found("Grammar item not found"),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

// --- Search, Stats, Backup ---

pub fn get_stats(_req: &Request, db: &Db) -> Response {
    match db.get_stats() {
        Ok(stats) => Response::json(&stats),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn search(req: &Request, db: &Db) -> Response {
    let q = req.query_param("q").map(|s| s.as_str()).unwrap_or("");
    match db.search(q) {
        Ok(res) => Response::json(&res),
        Err(e) => Response::internal_error(&format!("Database error: {}", e)),
    }
}

pub fn export_backup(_req: &Request, db: &Db) -> Response {
    match db.export_all() {
        Ok(backup) => {
            let mut res = Response::json(&backup);
            res.headers.insert(
                "Content-Disposition".to_string(),
                "attachment; filename=\"enotes_backup.json\"".to_string(),
            );
            res
        }
        Err(e) => Response::internal_error(&format!("Export error: {}", e)),
    }
}

pub fn import_backup(req: &Request, db: &Db) -> Response {
    let backup: BackupData = match req.json() {
        Ok(b) => b,
        Err(e) => return Response::bad_request(&format!("Invalid backup format: {}", e)),
    };

    match db.import_backup(backup) {
        Ok(()) => Response::json(&serde_json::json!({ "success": true, "message": "Import completed successfully" })),
        Err(e) => Response::internal_error(&format!("Import failed: {}", e)),
    }
}

// --- Static files ---

const EMBEDDED_INDEX_HTML: &str = include_str!("../static/index.html");
const EMBEDDED_STYLE_CSS: &str = include_str!("../static/style.css");
const EMBEDDED_APP_JS: &str = include_str!("../static/app.js");

pub fn serve_static(req: &Request, _db: &Db) -> Response {
    let clean_path = req.path.trim_start_matches('/');

    // Handle root and main assets with embedded fallback
    if clean_path.is_empty() || clean_path == "index.html" || clean_path == "static/index.html" {
        // Try reading from disk first for hot-reload
        if let Ok(bytes) = fs::read("static/index.html") {
            let mut res = Response::ok();
            res.set_body("text/html; charset=utf-8", bytes);
            return res;
        }
        let mut res = Response::ok();
        res.set_body("text/html; charset=utf-8", EMBEDDED_INDEX_HTML.as_bytes().to_vec());
        return res;
    }

    if clean_path == "style.css" || clean_path == "static/style.css" {
        if let Ok(bytes) = fs::read("static/style.css") {
            let mut res = Response::ok();
            res.set_body("text/css; charset=utf-8", bytes);
            return res;
        }
        let mut res = Response::ok();
        res.set_body("text/css; charset=utf-8", EMBEDDED_STYLE_CSS.as_bytes().to_vec());
        return res;
    }

    if clean_path == "app.js" || clean_path == "static/app.js" {
        if let Ok(bytes) = fs::read("static/app.js") {
            let mut res = Response::ok();
            res.set_body("application/javascript; charset=utf-8", bytes);
            return res;
        }
        let mut res = Response::ok();
        res.set_body("application/javascript; charset=utf-8", EMBEDDED_APP_JS.as_bytes().to_vec());
        return res;
    }

    // Try disk path relative to static/ or root
    let disk_path = if clean_path.starts_with("static/") {
        clean_path.to_string()
    } else {
        format!("static/{}", clean_path)
    };

    let p = Path::new(&disk_path);
    if p.exists() && p.is_file() {
        let mime_type = match p.extension().and_then(|s| s.to_str()).unwrap_or("") {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "application/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "png" => "image/png",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            _ => "application/octet-stream",
        };

        if let Ok(bytes) = fs::read(p) {
            let mut res = Response::ok();
            res.set_body(mime_type, bytes);
            return res;
        }
    }

    Response::not_found("File not found")
}
