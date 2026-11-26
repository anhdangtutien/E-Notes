use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonSession {
    pub id: i64,
    pub session_number: i32,
    pub title: String,
    pub date: String,
    pub teacher: Option<String>,
    pub goals: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    pub id: i64,
    pub session_id: i64,
    pub word: String,
    pub ipa: Option<String>,
    pub pos: Option<String>,         // noun, verb, adj, adv, idiom, phrase
    pub meaning: String,            // Tiếng Việt
    pub example_en: Option<String>, // Example in English
    pub example_vi: Option<String>, // Meaning of example in Vietnamese
    pub status: String,             // "learning", "reviewing", "mastered"
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    pub id: i64,
    pub session_id: i64,
    pub title: String,              // e.g. "Third Conditional"
    pub structure: String,          // e.g. "If + S + had + V3/ed, S + would have + V3/ed"
    pub explanation: String,        // Usage explanation
    pub examples: String,           // Positive, negative, question examples
    pub tips: Option<String>,       // Common mistakes or notes
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetail {
    pub session: LessonSession,
    pub vocabularies: Vec<Vocabulary>,
    pub grammars: Vec<Grammar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStats {
    pub total_sessions: i64,
    pub total_vocab: i64,
    pub total_grammar: i64,
    pub mastered_vocab: i64,
    pub learning_vocab: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub sessions: Vec<LessonSession>,
    pub vocabularies: Vec<Vocabulary>,
    pub grammars: Vec<Grammar>,
}

// Request DTOs
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSessionReq {
    pub session_number: Option<i32>,
    pub title: String,
    pub date: Option<String>,
    pub teacher: Option<String>,
    pub goals: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSessionReq {
    pub session_number: Option<i32>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub teacher: Option<String>,
    pub goals: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateVocabReq {
    pub session_id: i64,
    pub word: String,
    pub ipa: Option<String>,
    pub pos: Option<String>,
    pub meaning: String,
    pub example_en: Option<String>,
    pub example_vi: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateVocabReq {
    pub word: Option<String>,
    pub ipa: Option<String>,
    pub pos: Option<String>,
    pub meaning: Option<String>,
    pub example_en: Option<String>,
    pub example_vi: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGrammarReq {
    pub session_id: i64,
    pub title: String,
    pub structure: String,
    pub explanation: String,
    pub examples: String,
    pub tips: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGrammarReq {
    pub title: Option<String>,
    pub structure: Option<String>,
    pub explanation: Option<String>,
    pub examples: Option<String>,
    pub tips: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupData {
    pub sessions: Vec<LessonSession>,
    pub vocabularies: Vec<Vocabulary>,
    pub grammars: Vec<Grammar>,
    pub exported_at: String,
}
