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

        // --- BUỔI 1: CÁC THÌ CỐT LÕI (HIỆN TẠI, QUÁ KHỨ, TƯƠNG LAI) ---
        let s1 = self.create_session(CreateSessionReq {
            session_number: Some(1),
            title: "Buổi 1: Hệ thống các thì cốt lõi (Hiện tại, Quá khứ, Tương lai)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Mr. David Miller".to_string()),
            goals: Some("Nắm vững bản chất và cách phối hợp các thì Hiện tại, Quá khứ, Tương lai trong văn phong học thuật & giao tiếp.".to_string()),
            notes: Some("Chú ý phân biệt rõ mốc thời gian xác định (Past Simple) và hành động kéo dài đến nay (Present Perfect).".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "prioritize".to_string(),
            ipa: Some("/praɪˈɔːrətaɪz/".to_string()),
            pos: Some("verb".to_string()),
            meaning: "ưu tiên việc gì đó lên hàng đầu".to_string(),
            example_en: Some("You must prioritize tasks based on their urgency and importance.".to_string()),
            example_vi: Some("Bạn phải ưu tiên các nhiệm vụ dựa trên mức độ khẩn cấp và tầm quan trọng.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Collocation: prioritize A over B".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "simultaneously".to_string(),
            ipa: Some("/ˌsaɪmlˈteɪniəsli/".to_string()),
            pos: Some("adverb".to_string()),
            meaning: "đồng thời, cùng một lúc".to_string(),
            example_en: Some("Two research projects were carried out simultaneously.".to_string()),
            example_vi: Some("Hai dự án nghiên cứu đã được tiến hành cùng một lúc.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Synonym: at the same time, concurrently".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s1.id,
            word: "procrastinate".to_string(),
            ipa: Some("/prəˈkræstɪneɪt/".to_string()),
            pos: Some("verb".to_string()),
            meaning: "trì hoãn, chần chừ".to_string(),
            example_en: Some("People often procrastinate when facing difficult decisions.".to_string()),
            example_vi: Some("Mọi người thường chần chừ khi phải đối mặt với các quyết định khó khăn.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Noun form: procrastination".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "1. Các thì Hiện tại (Present Simple & Present Continuous)".to_string(),
            structure: "Hiện tại đơn: S + V(s/es) | Hiện tại tiếp diễn: S + am/is/are + V-ing".to_string(),
            explanation: "- Hiện tại đơn: Diễn tả chân lý, thói quen lặp lại thường xuyên.\n- Hiện tại tiếp diễn: Hành động đang xảy ra tại thời điểm nói hoặc kế hoạch gần.".to_string(),
            examples: "+ He usually drives to work, but this week he is taking the subway.\n- Water does not boil at 50 degrees Celsius.\n? Are you working on any special assignment right now?".to_string(),
            tips: Some("Động từ trạng thái (know, believe, understand, belong...) KHÔNG chia ở dạng tiếp diễn (-ing).".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "2. Các thì Quá khứ (Past Simple & Past Continuous)".to_string(),
            structure: "Quá khứ đơn: S + V2/ed | Quá khứ tiếp diễn: S + was/were + V-ing".to_string(),
            explanation: "- Quá khứ đơn: Hành động đã kết thúc trọn vẹn trong quá khứ có mốc thời gian rõ ràng.\n- Quá khứ tiếp diễn: Hành động đang xảy ra thì hành động khác cắt ngang.".to_string(),
            examples: "+ I was walking home yesterday when it suddenly started to pour rain.\n- They did not participate in the symposium last month.\n? What were you doing when the emergency siren went off?".to_string(),
            tips: Some("Phối hợp: 'When + Quá khứ đơn (hành động ngắn cắt ngang), Quá khứ tiếp diễn (hành động dài đang xảy ra)'.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "3. Hiện tại hoàn thành (Present Perfect)".to_string(),
            structure: "S + have/has + V3/ed (Phủ định: have/has not + V3/ed)".to_string(),
            explanation: "Diễn tả hành động xảy ra trong quá khứ và vẫn kéo dài đến hiện tại, hoặc kinh nghiệm từ trước tới nay (already, yet, just, ever, never, since, for).".to_string(),
            examples: "+ She has worked as a software engineer for six years.\n- We haven't deployed the production build yet.\n? Have you ever visited Singapore?".to_string(),
            tips: Some("Phân biệt: 'have gone to' (đã đi và chưa về) vs 'have been to' (đã từng đến và đã về).".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "4. Các thì Tương lai (Will vs Be Going To vs Future Perfect)".to_string(),
            structure: "Will: S + will + V | Be going to: S + am/is/are going to + V | Tương lai hoàn thành: S + will have + V3/ed".to_string(),
            explanation: "- Will: Quyết định tức thời tại thời điểm nói.\n- Be going to: Dự định có kế hoạch trước, hoặc dự đoán có bằng chứng cụ thể.\n- Tương lai hoàn thành: Hành động sẽ hoàn tất TRƯỚC một mốc thời gian tương lai.".to_string(),
            examples: "+ By the end of this month, our team will have finished the entire project.\n+ Don't worry, I will give you a hand with those luggage.\n- Look at the timetable, the flight isn't going to take off on time.".to_string(),
            tips: Some("Mệnh đề thời gian (when, as soon as, before) ở tương lai PHẢI chia ở Hiện tại đơn, KHÔNG dùng 'will'.".to_string()),
        })?;

        // --- BUỔI 2: MỆNH ĐỀ QUAN HỆ & RÚT GỌN MỆNH ĐỀ ---
        let s2 = self.create_session(CreateSessionReq {
            session_number: Some(2),
            title: "Buổi 2: Mệnh đề quan hệ & Rút gọn mệnh đề (Relative Clauses)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Ms. Sarah Jenkins".to_string()),
            goals: Some("Thành thạo đại từ/trạng từ quan hệ (Who, Whom, Which, That, Whose...) và 3 quy tắc rút gọn mệnh đề đỉnh cao.".to_string()),
            notes: Some("Tuyệt đối không dùng 'That' sau dấu phẩy (mệnh đề không xác định) và sau giới từ.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "comprehensive".to_string(),
            ipa: Some("/ˌkɑːmprɪˈhensɪv/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "toàn diện, bao quát mọi khía cạnh".to_string(),
            example_en: Some("The study provides a comprehensive overview of global climate patterns.".to_string()),
            example_vi: Some("Nghiên cứu cung cấp một cái nhìn toàn diện về các mô hình khí hậu toàn cầu.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Tránh nhầm với 'comprehensible' (dễ hiểu)".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "indispensable".to_string(),
            ipa: Some("/ˌɪndɪˈspensəbl/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "không thể thiếu được, thiết yếu".to_string(),
            example_en: Some("Critical thinking is indispensable for academic success.".to_string()),
            example_vi: Some("Tư duy phản biện là điều không thể thiếu cho thành công trong học tập.".to_string()),
            status: Some("reviewing".to_string()),
            notes: Some("Collocation: an indispensable tool / element".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "facilitate".to_string(),
            ipa: Some("/fəˈsɪlɪteɪt/".to_string()),
            pos: Some("verb".to_string()),
            meaning: "tạo điều kiện thuận lợi, hỗ trợ dễ dàng hơn".to_string(),
            example_en: Some("The new digital tools facilitate collaboration among remote teams.".to_string()),
            example_vi: Some("Các công cụ kỹ thuật số mới tạo điều kiện thuận lợi cho việc cộng tác giữa các đội nhóm từ xa.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Formal academic verb".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "1. Đại từ & Trạng từ quan hệ (Who, Whom, Which, That, Whose...)".to_string(),
            structure: "Người: WHO + V / WHOM + S + V | Vật: WHICH/THAT + V/S+V | Sở hữu: N + WHOSE + N | Trạng từ: WHERE, WHEN, WHY".to_string(),
            explanation: "Dùng để nối 2 câu đơn thành câu phức. Phân biệt mệnh đề xác định (không dấu phẩy) và không xác định (có dấu phẩy, KHÔNG dùng 'that').".to_string(),
            examples: "+ The engineer who developed this application received the award.\n+ Dr. Harrison, whom we interviewed yesterday, has accepted the position.\n+ The company whose headquarters is in Tokyo announced a new strategy.".to_string(),
            tips: Some("Sau dấu phẩy và sau giới từ (in, on, with...) TUYỆT ĐỐI KHÔNG DÙNG 'THAT'.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "2. Rút gọn mệnh đề quan hệ (Reduced Relative Clauses)".to_string(),
            structure: "1. Chủ động ➡️ V-ing | 2. Bị động ➡️ V3/ed | 3. Sau the first, last, only ➡️ To V".to_string(),
            explanation: "Biến đổi giúp câu văn ngắn gọn, học thuật hơn:\n- Chủ động: The girl who is talking ➡️ The girl talking.\n- Bị động: The laptops which were manufactured ➡️ The laptops manufactured.\n- Sau first/only: The only person who passed ➡️ The only person to pass.".to_string(),
            examples: "+ Active: Documents submitted after 5 PM will not be processed.\n+ Passive: The keynote address delivered by the CEO inspired everyone.\n+ To-V: She is the only candidate to achieve a perfect band score.".to_string(),
            tips: Some("Luôn xác định xem danh từ phía trước là CHỦ THỂ (hành động ➡️ V-ing) hay ĐỐI TƯỢNG (bị tác động ➡️ V3/ed).".to_string()),
        })?;

        // --- BUỔI 3: QUY TẮC PHÁT ÂM ĐUÔI -ED ---
        let s3 = self.create_session(CreateSessionReq {
            session_number: Some(3),
            title: "Buổi 3: Quy tắc phát âm đuôi -ED chuẩn quốc tế (/id/, /t/, /d/)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Dr. Alexander Hayes".to_string()),
            goals: Some("Nắm trọn vẹn 3 quy tắc phát âm đuôi -ED theo ngữ âm học quốc tế và danh sách tính từ ngoại lệ.".to_string()),
            notes: Some("Ghi nhớ câu thần chú âm vô thanh: 'Chính Phủ Phát Sách Không Thèm / Thảo' để đọc đuôi /t/ chuẩn xác.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "articulate".to_string(),
            ipa: Some("/ɑːrˈtɪkjuleɪt/".to_string()),
            pos: Some("verb / adjective".to_string()),
            meaning: "phát âm rõ ràng, diễn đạt lưu loát gãy gọn".to_string(),
            example_en: Some("She gave an articulate and persuasive presentation yesterday.".to_string()),
            example_vi: Some("Cô ấy đã có một bài thuyết trình lưu loát và đầy sức thuyết phục ngày hôm qua.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("V: /ɑːrˈtɪkjuleɪt/ - Adj: /ɑːrˈtɪkjulət/".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "distinguish".to_string(),
            ipa: Some("/dɪˈstɪŋɡwɪʃ/".to_string()),
            pos: Some("verb".to_string()),
            meaning: "phân biệt, nhận diện sự khác nhau".to_string(),
            example_en: Some("Can you distinguish between voiceless and voiced consonants?".to_string()),
            example_vi: Some("Bạn có thể phân biệt giữa phụ âm vô thanh và hữu thanh không?".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Collocation: distinguish between A and B".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "exception".to_string(),
            ipa: Some("/ɪkˈsepʃn/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "trường hợp ngoại lệ".to_string(),
            example_en: Some("Words like 'naked' and 'wicked' are special exceptions in -ed pronunciation.".to_string()),
            example_vi: Some("Các từ như 'naked' và 'wicked' là những ngoại lệ đặc biệt trong cách phát âm đuôi -ed.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Idiom: With the exception of (= ngoại trừ)".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "1. Ba quy tắc vàng phát âm đuôi -ED (/id/, /t/, /d/)".to_string(),
            structure: "1. Đọc là /id/: sau âm /t/ và /d/\n2. Đọc là /t/: sau âm vô thanh (/p, k, f, s, ʃ, tʃ/)\n3. Đọc là /d/: các âm hữu thanh còn lại".to_string(),
            explanation: "- Nhóm 1 (/id/): wanted, needed, decided, started, invited, landed.\n- Nhóm 2 (/t/): Âm vô thanh ('Chính Phủ Phát Sách Không Thèm / Thảo'): stopped, looked, laughed, washed, watched, fixed.\n- Nhóm 3 (/d/): Các âm hữu thanh (rung cổ họng): played, opened, lived, cleaned, called.".to_string(),
            examples: "+ /id/: They decided to postpone the tournament until next week.\n+ /t/: He walked to the office, checked his email, and fixed the bug.\n+ /d/: She called her parents and explained the whole situation.".to_string(),
            tips: Some("Nhìn vào ÂM CUỐI CÙNG được phát âm, KHÔNG nhìn vào chữ cái viết (ví dụ: 'laughed' tận cùng chữ gh nhưng phát âm là /f/ ➡️ đọc là /t/).".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "2. Danh sách tính từ ngoại lệ bắt buộc phát âm là /id/".to_string(),
            structure: "Danh sách từ đặc biệt: naked, wicked, learned, ragged, sacred, blessed, crooked...".to_string(),
            explanation: "Khi các từ này đóng vai trò là TÍNH TỪ (Adjectives), đuôi -ed luôn phát âm là /id/ thay vì /t/ hay /d/ như quy tắc thông thường.".to_string(),
            examples: "+ naked /ˈneɪkɪd/ (trần trụi, khỏa thân)\n+ wicked /ˈwɪkɪd/ (độc ác, xấu xa)\n+ learned /ˈlɜːnɪd/ (uyên bác, có học thức - e.g. a learned professor)\n+ ragged /ˈræɡɪd/ (rách rưới, tả tơi)\n+ sacred /ˈseɪkrɪd/ (thiêng liêng, trang trọng)\n+ blessed /ˈblesɪd/ (may mắn, thần thánh)\n+ crooked /ˈkrʊkɪd/ (quanh co, cong queo)".to_string(),
            tips: Some("Trong bài thi trắc nghiệm phát âm THPTQG và TOEIC, các từ ngoại lệ này xuất hiện với tần suất trên 80%!".to_string()),
        })?;

        // --- BUỔI 4: QUY TẮC NỐI ÂM (CONNECTED SPEECH) ---
        let s4 = self.create_session(CreateSessionReq {
            session_number: Some(4),
            title: "Buổi 4: Quy tắc đọc nối đuôi & Nói chuẩn ngữ điệu (Connected Speech)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Coach Emma Watson".to_string()),
            goals: Some("Làm chủ 4 quy tắc nối âm: Phụ âm - Nguyên âm, Nguyên âm - Nguyên âm, Biến âm (Assimilation) và Nuốt âm (Elision).".to_string()),
            notes: Some("Luyện phát âm theo cụm luồng hơi liền mạch thay vì ngắt từng từ đơn lẻ.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s4.id,
            word: "fluency".to_string(),
            ipa: Some("/ˈfluːənsi/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "sự lưu loát, trôi chảy trong ngôn ngữ".to_string(),
            example_en: Some("Linking sounds properly will significantly boost your speaking fluency.".to_string()),
            example_vi: Some("Việc nối âm đúng cách sẽ nâng cao đáng kể sự trôi chảy khi bạn nói.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Adjective: fluent".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s4.id,
            word: "intonation".to_string(),
            ipa: Some("/ˌɪntəˈneɪʃn/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "ngữ điệu lên xuống của câu nói".to_string(),
            example_en: Some("Natural intonation helps listeners grasp your emotions and intentions.".to_string()),
            example_vi: Some("Ngữ điệu tự nhiên giúp người nghe nắm bắt cảm xúc và chủ ý của bạn.".to_string()),
            status: Some("reviewing".to_string()),
            notes: Some("Rising intonation (câu hỏi Yes/No) vs Falling intonation (câu trần thuật)".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s4.id,
            word: "spontaneous".to_string(),
            ipa: Some("/spɑːnˈteɪniəs/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "tự nhiên, bộc phát không cần chuẩn bị trước".to_string(),
            example_en: Some("Daily practice makes your conversational responses more spontaneous.".to_string()),
            example_vi: Some("Việc luyện tập mỗi ngày giúp phản xạ giao tiếp của bạn trở nên tự nhiên hơn.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Adverb: spontaneously".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s4.id,
            title: "1. Nối âm Phụ âm - Nguyên âm & Nguyên âm - Nguyên âm".to_string(),
            structure: "Phụ âm + Nguyên âm: C + V ➡️ Nối dính | Nguyên âm + Nguyên âm: V + V ➡️ Chèn âm /j/ hoặc /w/".to_string(),
            explanation: "1. Phụ âm nối nguyên âm (C+V): 'Hold on' ➡️ /həʊl-dɒn/, 'Turn off' ➡️ /tɜːn-nɒf/, 'An apple' ➡️ /ən-næpl/.\n2. Nguyên âm nối nguyên âm (V+V):\n   - Miệng dẹt (/i:, ɪ, eɪ, aɪ/) ➡️ Chèn âm /j/: 'I agree' ➡️ /aɪ-j-əˈɡriː/, 'See it' ➡️ /siː-j-ɪt/.\n   - Miệng tròn (/u:, əʊ, aʊ/) ➡️ Chèn âm /w/: 'Go out' ➡️ /ɡəʊ-w-aʊt/, 'You are' ➡️ /juː-w-ɑːr/.".to_string(),
            examples: "+ C+V: Can I have a cup of water? ➡️ /kæn-aɪ hæv-ə kʌp-əv wɔːtər/\n+ V+V (/j/): She asked me to say it again. ➡️ /seɪ-j-ɪt/\n+ V+V (/w/): Let's do it right away. ➡️ /duː-w-ɪt/".to_string(),
            tips: Some("Khi nói tiếng Anh, đừng cố phát âm rời rạc như gõ máy; hãy để luồng hơi nối trơn tru giữa các âm tiết.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s4.id,
            title: "2. Biến âm (Assimilation) & Nuốt âm (Elision) trong giao tiếp".to_string(),
            structure: "/t/ + /j/ ➡️ /tʃ/ | /d/ + /j/ ➡️ /dʒ/ | Nuốt /t/, /d/ kẹt giữa 2 phụ âm".to_string(),
            explanation: "1. Biến âm (Assimilation):\n   - /t/ gặp /j/: 'Nice to meet you' ➡️ /miːtʃu/, 'Don't you' ➡️ /dəʊntʃu/.\n   - /d/ gặp /j/: 'Did you' ➡️ /dɪdʒu/, 'Could you' ➡️ /kʊdʒu/.\n2. Nuốt âm (Elision): Bỏ hẳn âm /t/ hoặc /d/ khi kẹt giữa hai phụ âm khác:\n   - 'Last night' ➡️ /lɑːs naɪt/ (bỏ /t/)\n   - 'Next door' ➡️ /neks dɔːr/ (bỏ /t/)".to_string(),
            examples: "+ Could you tell me what you need? ➡️ /kʊdʒu tel miː wʌtʃu niːd/\n+ I saw him last night next door. ➡️ /lɑːs naɪt neks dɔːr/\n+ Don't you want to come? ➡️ /dəʊntʃu wɒntə kʌm/".to_string(),
            tips: Some("Nắm được quy tắc này bạn sẽ nghe hiểu người bản xứ khi họ nói nhanh (Fast Native Speech) mà không bị bỡ ngỡ.".to_string()),
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
