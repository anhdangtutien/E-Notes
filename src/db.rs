use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use chrono::Local;

use crate::models::{
    AppStats, BackupData, CreateGrammarReq, CreateSessionReq, CreateVocabReq, Grammar,
    LessonSession, SearchResult, SessionDetail, UpdateGrammarReq, UpdateSessionReq,
    UpdateVocabReq, Vocabulary,
};

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_number INTEGER NOT NULL,
                title TEXT NOT NULL,
                date TEXT NOT NULL,
                teacher TEXT,
                goals TEXT,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS vocabularies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
                word TEXT NOT NULL,
                ipa TEXT,
                pos TEXT,
                meaning TEXT NOT NULL,
                example_en TEXT,
                example_vi TEXT,
                status TEXT NOT NULL DEFAULT 'learning',
                notes TEXT,
                created_at TEXT NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS grammars (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                structure TEXT NOT NULL,
                explanation TEXT NOT NULL,
                examples TEXT NOT NULL,
                tips TEXT,
                created_at TEXT NOT NULL
            );",
            [],
        )?;

        // Check if empty, then seed
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))?;
        drop(conn);

        if count == 0 {
            self.seed_initial_data()?;
        }

        Ok(())
    }

    fn seed_initial_data(&self) -> Result<()> {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let s1 = self.create_session(CreateSessionReq {
            session_number: Some(1),
            title: "Buổi 1: Business English & Negotiations".to_string(),
            date: Some(today.clone()),
            teacher: Some("Mr. David Miller".to_string()),
            goals: Some("Nắm vững kỹ năng thuyết trình dự án và đàm phán hợp đồng thương mại.".to_string()),
            notes: Some("Cần chú ý cách diễn đạt gián tiếp (diplomatic language) khi từ chối đối tác lịch sự.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "articulate".to_string(),
            ipa: Some("/ɑːrˈtɪkjuleɪt/".to_string()),
            pos: Some("verb / adjective".to_string()),
            meaning: "diễn đạt gãy gọn, lưu loát / ăn nói rõ ràng".to_string(),
            example_en: Some("She gave a witty, entertaining, and articulate presentation.".to_string()),
            example_vi: Some("Cô ấy đã có một bài thuyết trình hóm hỉnh, cuốn hút và diễn đạt rất lưu loát.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Thường đi với 'articulate an idea / opinion'".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "compromise".to_string(),
            ipa: Some("/ˈkɑːmprəmaɪz/".to_string()),
            pos: Some("noun / verb".to_string()),
            meaning: "sự thỏa hiệp, nhượng bộ lẫn nhau".to_string(),
            example_en: Some("We reached a compromise after two hours of tough debate.".to_string()),
            example_vi: Some("Chúng tôi đã đạt được thỏa hiệp sau hai giờ tranh luận căng thẳng.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Collocation: reach/arrive at a compromise".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "concession".to_string(),
            ipa: Some("/kənˈseʃn/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "sự nhượng bộ trong đàm phán".to_string(),
            example_en: Some("Both parties made major concessions to secure the deal.".to_string()),
            example_vi: Some("Cả hai bên đều đã có những nhượng bộ lớn để ký kết được hợp đồng.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Make a concession: nhượng bộ điều gì đó".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "Diplomatic Language (Ngôn ngữ ngoại giao / Giảm nhẹ)".to_string(),
            structure: "Subject + seem / appear / tend + to V  |  I was wondering if... | Wouldn't it be better if...".to_string(),
            explanation: "Sử dụng để giảm bớt sự gay gắt khi nêu ý kiến trái chiều hoặc chỉ trích đối tác trong môi trường công sở quốc tế.".to_string(),
            examples: "Direct: 'Your price is too high.'\nDiplomatic: 'Your pricing seems to be slightly above our current budget constraints.'\nDirect: 'Send me the report now.'\nDiplomatic: 'I was wondering if you could possibly send over the report by 3 PM.'".to_string(),
            tips: Some("Tránh dùng 'You must' hoặc 'You are wrong' trong đàm phán chuyên nghiệp.".to_string()),
        })?;

        let s2 = self.create_session(CreateSessionReq {
            session_number: Some(2),
            title: "Buổi 2: Conditional Sentences & Hypothetical Scenarios".to_string(),
            date: Some(today.clone()),
            teacher: Some("Ms. Sarah Jenkins".to_string()),
            goals: Some("Thành thạo câu điều kiện hỗn hợp (Mixed Conditionals) và cấu trúc Wish/If only.".to_string()),
            notes: Some("Luyện tập phân biệt điều kiện loại 2 (hiện tại) và loại 3 (quá khứ).".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "contingency".to_string(),
            ipa: Some("/kənˈtɪndʒənsi/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "kế hoạch dự phòng, trường hợp khẩn cấp có thể xảy ra".to_string(),
            example_en: Some("We need a contingency plan in case the supplier delays shipments.".to_string()),
            example_vi: Some("Chúng ta cần một kế hoạch dự phòng đề phòng trường hợp nhà cung ứng chậm giao hàng.".to_string()),
            status: Some("reviewing".to_string()),
            notes: Some("Contingency plan = Plan B".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "hindsight".to_string(),
            ipa: Some("/ˈhaɪndsaɪt/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "sự suy xét lại sau khi sự việc đã xảy ra".to_string(),
            example_en: Some("With hindsight, we should have invested in automated testing much sooner.".to_string()),
            example_vi: Some("Nhìn lại quá khứ, đáng lẽ chúng ta nên đầu tư vào kiểm thử tự động sớm hơn.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Idiom: 'With hindsight' hoặc 'In hindsight'".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "Mixed Conditional: Past Action with Present Result".to_string(),
            structure: "If + S + had + V3/ed (Quá khứ), S + would + V-inf (Hiện tại)".to_string(),
            explanation: "Diễn tả một giả định trái ngược với quá khứ, dẫn đến một kết quả ảnh hưởng trực tiếp đến hiện tại.".to_string(),
            examples: "+ If I had taken that training course last year, I would speak fluent English today.\n- If he hadn't missed the flight yesterday, he wouldn't be stuck at the airport now.\n? Would you have this high position today if you hadn't worked so hard?".to_string(),
            tips: Some("Chú ý trạng từ thời gian: vế If có dấu hiệu quá khứ (yesterday, last year), vế chính có dấu hiệu hiện tại (now, today).".to_string()),
        })?;

        Ok(())
    }

    // Sessions
    pub fn get_sessions(&self) -> Result<Vec<LessonSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_number, title, date, teacher, goals, notes, created_at, updated_at 
             FROM sessions ORDER BY session_number DESC, id DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(LessonSession {
                id: row.get(0)?,
                session_number: row.get(1)?,
                title: row.get(2)?,
                date: row.get(3)?,
                teacher: row.get(4)?,
                goals: row.get(5)?,
                notes: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn get_session(&self, id: i64) -> Result<Option<LessonSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_number, title, date, teacher, goals, notes, created_at, updated_at 
             FROM sessions WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(LessonSession {
                id: row.get(0)?,
                session_number: row.get(1)?,
                title: row.get(2)?,
                date: row.get(3)?,
                teacher: row.get(4)?,
                goals: row.get(5)?,
                notes: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        if let Some(res) = rows.next() {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub fn get_session_detail(&self, id: i64) -> Result<Option<SessionDetail>> {
        let session = self.get_session(id)?;
        if let Some(session) = session {
            let vocabularies = self.get_vocab_by_session(id)?;
            let grammars = self.get_grammar_by_session(id)?;
            Ok(Some(SessionDetail {
                session,
                vocabularies,
                grammars,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn create_session(&self, req: CreateSessionReq) -> Result<LessonSession> {
        let conn = self.conn.lock().unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let today = Local::now().format("%Y-%m-%d").to_string();

        let session_number = req.session_number.unwrap_or_else(|| {
            let max: Option<i32> = conn
                .query_row("SELECT MAX(session_number) FROM sessions", [], |r| r.get(0))
                .unwrap_or(None);
            max.unwrap_or(0) + 1
        });

        let date = req.date.unwrap_or(today);

        conn.execute(
            "INSERT INTO sessions (session_number, title, date, teacher, goals, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                session_number,
                req.title,
                date,
                req.teacher,
                req.goals,
                req.notes,
                now,
                now
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(LessonSession {
            id,
            session_number,
            title: req.title,
            date,
            teacher: req.teacher,
            goals: req.goals,
            notes: req.notes,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn update_session(&self, id: i64, req: UpdateSessionReq) -> Result<Option<LessonSession>> {
        let existing = self.get_session(id)?;
        if existing.is_none() {
            return Ok(None);
        }
        let current = existing.unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let session_number = req.session_number.unwrap_or(current.session_number);
        let title = req.title.unwrap_or(current.title);
        let date = req.date.unwrap_or(current.date);
        let teacher = if req.teacher.is_some() { req.teacher } else { current.teacher };
        let goals = if req.goals.is_some() { req.goals } else { current.goals };
        let notes = if req.notes.is_some() { req.notes } else { current.notes };

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sessions 
             SET session_number = ?1, title = ?2, date = ?3, teacher = ?4, goals = ?5, notes = ?6, updated_at = ?7
             WHERE id = ?8",
            params![
                session_number,
                title,
                date,
                teacher,
                goals,
                notes,
                now,
                id
            ],
        )?;

        Ok(Some(LessonSession {
            id,
            session_number,
            title,
            date,
            teacher,
            goals,
            notes,
            created_at: current.created_at,
            updated_at: now,
        }))
    }

    pub fn delete_session(&self, id: i64) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }

    // Vocabulary
    pub fn get_vocab_by_session(&self, session_id: i64) -> Result<Vec<Vocabulary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at
             FROM vocabularies WHERE session_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(Vocabulary {
                id: row.get(0)?,
                session_id: row.get(1)?,
                word: row.get(2)?,
                ipa: row.get(3)?,
                pos: row.get(4)?,
                meaning: row.get(5)?,
                example_en: row.get(6)?,
                example_vi: row.get(7)?,
                status: row.get(8)?,
                notes: row.get(9)?,
                created_at: row.get(10)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn create_vocab(&self, req: CreateVocabReq) -> Result<Vocabulary> {
        let conn = self.conn.lock().unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let status = req.status.unwrap_or_else(|| "learning".to_string());

        conn.execute(
            "INSERT INTO vocabularies (session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                req.session_id,
                req.word,
                req.ipa,
                req.pos,
                req.meaning,
                req.example_en,
                req.example_vi,
                status,
                req.notes,
                now
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Vocabulary {
            id,
            session_id: req.session_id,
            word: req.word,
            ipa: req.ipa,
            pos: req.pos,
            meaning: req.meaning,
            example_en: req.example_en,
            example_vi: req.example_vi,
            status,
            notes: req.notes,
            created_at: now,
        })
    }

    pub fn update_vocab(&self, id: i64, req: UpdateVocabReq) -> Result<Option<Vocabulary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at 
             FROM vocabularies WHERE id = ?1",
        )?;
        let current = stmt.query_row(params![id], |row| {
            Ok(Vocabulary {
                id: row.get(0)?,
                session_id: row.get(1)?,
                word: row.get(2)?,
                ipa: row.get(3)?,
                pos: row.get(4)?,
                meaning: row.get(5)?,
                example_en: row.get(6)?,
                example_vi: row.get(7)?,
                status: row.get(8)?,
                notes: row.get(9)?,
                created_at: row.get(10)?,
            })
        });

        if current.is_err() {
            return Ok(None);
        }
        let current = current.unwrap();

        let word = req.word.unwrap_or(current.word);
        let ipa = if req.ipa.is_some() { req.ipa } else { current.ipa };
        let pos = if req.pos.is_some() { req.pos } else { current.pos };
        let meaning = req.meaning.unwrap_or(current.meaning);
        let example_en = if req.example_en.is_some() { req.example_en } else { current.example_en };
        let example_vi = if req.example_vi.is_some() { req.example_vi } else { current.example_vi };
        let status = req.status.unwrap_or(current.status);
        let notes = if req.notes.is_some() { req.notes } else { current.notes };

        conn.execute(
            "UPDATE vocabularies 
             SET word = ?1, ipa = ?2, pos = ?3, meaning = ?4, example_en = ?5, example_vi = ?6, status = ?7, notes = ?8
             WHERE id = ?9",
            params![
                word,
                ipa,
                pos,
                meaning,
                example_en,
                example_vi,
                status,
                notes,
                id
            ],
        )?;

        Ok(Some(Vocabulary {
            id,
            session_id: current.session_id,
            word,
            ipa,
            pos,
            meaning,
            example_en,
            example_vi,
            status,
            notes,
            created_at: current.created_at,
        }))
    }

    pub fn delete_vocab(&self, id: i64) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM vocabularies WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }

    // Grammar
    pub fn get_grammar_by_session(&self, session_id: i64) -> Result<Vec<Grammar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, title, structure, explanation, examples, tips, created_at
             FROM grammars WHERE session_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(Grammar {
                id: row.get(0)?,
                session_id: row.get(1)?,
                title: row.get(2)?,
                structure: row.get(3)?,
                explanation: row.get(4)?,
                examples: row.get(5)?,
                tips: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn create_grammar(&self, req: CreateGrammarReq) -> Result<Grammar> {
        let conn = self.conn.lock().unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        conn.execute(
            "INSERT INTO grammars (session_id, title, structure, explanation, examples, tips, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                req.session_id,
                req.title,
                req.structure,
                req.explanation,
                req.examples,
                req.tips,
                now
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Grammar {
            id,
            session_id: req.session_id,
            title: req.title,
            structure: req.structure,
            explanation: req.explanation,
            examples: req.examples,
            tips: req.tips,
            created_at: now,
        })
    }

    pub fn update_grammar(&self, id: i64, req: UpdateGrammarReq) -> Result<Option<Grammar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, title, structure, explanation, examples, tips, created_at 
             FROM grammars WHERE id = ?1",
        )?;
        let current = stmt.query_row(params![id], |row| {
            Ok(Grammar {
                id: row.get(0)?,
                session_id: row.get(1)?,
                title: row.get(2)?,
                structure: row.get(3)?,
                explanation: row.get(4)?,
                examples: row.get(5)?,
                tips: row.get(6)?,
                created_at: row.get(7)?,
            })
        });

        if current.is_err() {
            return Ok(None);
        }
        let current = current.unwrap();

        let title = req.title.unwrap_or(current.title);
        let structure = req.structure.unwrap_or(current.structure);
        let explanation = req.explanation.unwrap_or(current.explanation);
        let examples = req.examples.unwrap_or(current.examples);
        let tips = if req.tips.is_some() { req.tips } else { current.tips };

        conn.execute(
            "UPDATE grammars 
             SET title = ?1, structure = ?2, explanation = ?3, examples = ?4, tips = ?5
             WHERE id = ?6",
            params![
                title,
                structure,
                explanation,
                examples,
                tips,
                id
            ],
        )?;

        Ok(Some(Grammar {
            id,
            session_id: current.session_id,
            title,
            structure,
            explanation,
            examples,
            tips,
            created_at: current.created_at,
        }))
    }

    pub fn delete_grammar(&self, id: i64) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM grammars WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }

    // Stats
    pub fn get_stats(&self) -> Result<AppStats> {
        let conn = self.conn.lock().unwrap();
        let total_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get(0))?;
        let total_vocab: i64 = conn.query_row("SELECT COUNT(*) FROM vocabularies", [], |r| r.get(0))?;
        let total_grammar: i64 = conn.query_row("SELECT COUNT(*) FROM grammars", [], |r| r.get(0))?;
        let mastered_vocab: i64 = conn.query_row(
            "SELECT COUNT(*) FROM vocabularies WHERE status = 'mastered'",
            [],
            |r| r.get(0),
        )?;
        let learning_vocab: i64 = conn.query_row(
            "SELECT COUNT(*) FROM vocabularies WHERE status != 'mastered'",
            [],
            |r| r.get(0),
        )?;

        Ok(AppStats {
            total_sessions,
            total_vocab,
            total_grammar,
            mastered_vocab,
            learning_vocab,
        })
    }

    // Search across sessions, vocab, grammar
    pub fn search(&self, q: &str) -> Result<SearchResult> {
        let conn = self.conn.lock().unwrap();
        let query_pattern = format!("%{}%", q.trim());

        // Sessions
        let mut s_stmt = conn.prepare(
            "SELECT id, session_number, title, date, teacher, goals, notes, created_at, updated_at
             FROM sessions 
             WHERE title LIKE ?1 OR teacher LIKE ?1 OR goals LIKE ?1 OR notes LIKE ?1
             ORDER BY session_number DESC",
        )?;
        let s_rows = s_stmt.query_map(params![query_pattern], |r| {
            Ok(LessonSession {
                id: r.get(0)?,
                session_number: r.get(1)?,
                title: r.get(2)?,
                date: r.get(3)?,
                teacher: r.get(4)?,
                goals: r.get(5)?,
                notes: r.get(6)?,
                created_at: r.get(7)?,
                updated_at: r.get(8)?,
            })
        })?;
        let mut sessions = Vec::new();
        for r in s_rows {
            sessions.push(r?);
        }

        // Vocab
        let mut v_stmt = conn.prepare(
            "SELECT id, session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at
             FROM vocabularies
             WHERE word LIKE ?1 OR meaning LIKE ?1 OR example_en LIKE ?1 OR example_vi LIKE ?1 OR notes LIKE ?1
             ORDER BY id DESC",
        )?;
        let v_rows = v_stmt.query_map(params![query_pattern], |r| {
            Ok(Vocabulary {
                id: r.get(0)?,
                session_id: r.get(1)?,
                word: r.get(2)?,
                ipa: r.get(3)?,
                pos: r.get(4)?,
                meaning: r.get(5)?,
                example_en: r.get(6)?,
                example_vi: r.get(7)?,
                status: r.get(8)?,
                notes: r.get(9)?,
                created_at: r.get(10)?,
            })
        })?;
        let mut vocabularies = Vec::new();
        for r in v_rows {
            vocabularies.push(r?);
        }

        // Grammar
        let mut g_stmt = conn.prepare(
            "SELECT id, session_id, title, structure, explanation, examples, tips, created_at
             FROM grammars
             WHERE title LIKE ?1 OR structure LIKE ?1 OR explanation LIKE ?1 OR examples LIKE ?1 OR tips LIKE ?1
             ORDER BY id DESC",
        )?;
        let g_rows = g_stmt.query_map(params![query_pattern], |r| {
            Ok(Grammar {
                id: r.get(0)?,
                session_id: r.get(1)?,
                title: r.get(2)?,
                structure: r.get(3)?,
                explanation: r.get(4)?,
                examples: r.get(5)?,
                tips: r.get(6)?,
                created_at: r.get(7)?,
            })
        })?;
        let mut grammars = Vec::new();
        for r in g_rows {
            grammars.push(r?);
        }

        Ok(SearchResult {
            sessions,
            vocabularies,
            grammars,
        })
    }

    // Export all data
    pub fn export_all(&self) -> Result<BackupData> {
        let conn = self.conn.lock().unwrap();
        let sessions = {
            let mut stmt = conn.prepare("SELECT id, session_number, title, date, teacher, goals, notes, created_at, updated_at FROM sessions ORDER BY session_number ASC")?;
            let rows = stmt.query_map([], |r| {
                Ok(LessonSession {
                    id: r.get(0)?,
                    session_number: r.get(1)?,
                    title: r.get(2)?,
                    date: r.get(3)?,
                    teacher: r.get(4)?,
                    goals: r.get(5)?,
                    notes: r.get(6)?,
                    created_at: r.get(7)?,
                    updated_at: r.get(8)?,
                })
            })?;
            let mut list = Vec::new();
            for r in rows { list.push(r?); }
            list
        };

        let vocabularies = {
            let mut stmt = conn.prepare("SELECT id, session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at FROM vocabularies ORDER BY id ASC")?;
            let rows = stmt.query_map([], |r| {
                Ok(Vocabulary {
                    id: r.get(0)?,
                    session_id: r.get(1)?,
                    word: r.get(2)?,
                    ipa: r.get(3)?,
                    pos: r.get(4)?,
                    meaning: r.get(5)?,
                    example_en: r.get(6)?,
                    example_vi: r.get(7)?,
                    status: r.get(8)?,
                    notes: r.get(9)?,
                    created_at: r.get(10)?,
                })
            })?;
            let mut list = Vec::new();
            for r in rows { list.push(r?); }
            list
        };

        let grammars = {
            let mut stmt = conn.prepare("SELECT id, session_id, title, structure, explanation, examples, tips, created_at FROM grammars ORDER BY id ASC")?;
            let rows = stmt.query_map([], |r| {
                Ok(Grammar {
                    id: r.get(0)?,
                    session_id: r.get(1)?,
                    title: r.get(2)?,
                    structure: r.get(3)?,
                    explanation: r.get(4)?,
                    examples: r.get(5)?,
                    tips: r.get(6)?,
                    created_at: r.get(7)?,
                })
            })?;
            let mut list = Vec::new();
            for r in rows { list.push(r?); }
            list
        };

        Ok(BackupData {
            sessions,
            vocabularies,
            grammars,
            exported_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    // Import data
    pub fn import_backup(&self, data: BackupData) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        // Clean tables
        tx.execute("DELETE FROM grammars", [])?;
        tx.execute("DELETE FROM vocabularies", [])?;
        tx.execute("DELETE FROM sessions", [])?;

        for s in data.sessions {
            tx.execute(
                "INSERT INTO sessions (id, session_number, title, date, teacher, goals, notes, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    s.id,
                    s.session_number,
                    s.title,
                    s.date,
                    s.teacher,
                    s.goals,
                    s.notes,
                    s.created_at,
                    s.updated_at
                ],
            )?;
        }

        for v in data.vocabularies {
            tx.execute(
                "INSERT INTO vocabularies (id, session_id, word, ipa, pos, meaning, example_en, example_vi, status, notes, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    v.id,
                    v.session_id,
                    v.word,
                    v.ipa,
                    v.pos,
                    v.meaning,
                    v.example_en,
                    v.example_vi,
                    v.status,
                    v.notes,
                    v.created_at
                ],
            )?;
        }

        for g in data.grammars {
            tx.execute(
                "INSERT INTO grammars (id, session_id, title, structure, explanation, examples, tips, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    g.id,
                    g.session_id,
                    g.title,
                    g.structure,
                    g.explanation,
                    g.examples,
                    g.tips,
                    g.created_at
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}
