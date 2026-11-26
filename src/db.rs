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

        // ==========================================
        // BUỔI 1: HỆ THỐNG 4 THÌ HIỆN TẠI (PRESENT TENSES)
        // ==========================================
        let s1 = self.create_session(CreateSessionReq {
            session_number: Some(1),
            title: "Buổi 1: Hệ thống 4 thì Hiện tại (Present Simple, Continuous, Perfect, Perfect Continuous)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Mr. David Miller".to_string()),
            goals: Some("Làm chủ trọn vẹn bản chất, công thức và ngữ cảnh sử dụng của cả 4 thì Hiện tại trong tiếng Anh chuẩn quốc tế.".to_string()),
            notes: Some("Phân biệt rõ: Hiện tại đơn (thói quen, chân lý) vs Tiếp diễn (tại thời điểm nói) vs Hoàn thành (kết quả đến nay) vs Hoàn thành tiếp diễn (quá trình liên tục).".to_string()),
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
            title: "1. Thì Hiện tại đơn (Present Simple Tense)".to_string(),
            structure: "(+) S + V(s/es) | (-) S + do/does not + V_inf | (?) Do/Does + S + V_inf?\nVới To-be: (+) S + am/is/are | (-) S + am/is/are not | (?) Am/Is/Are + S...?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Chân lý khoa học, sự thật hiển nhiên (Water boils at 100°C).\n2. Thói quen, hành động lặp đi lặp lại thường xuyên (I brush my teeth twice a day).\n3. Lịch trình tàu xe, máy bay, thời khóa biểu cố định (The train departs at 6:00 PM).\n4. Dấu hiệu nhận biết: always, usually, often, sometimes, seldom, never, every day/week, once a year...".to_string(),
            examples: "+ Water boils at 100 degrees Celsius under standard atmospheric pressure.\n- He does not drink coffee in the evening because it disrupts his sleep.\n? Do you practice English pronunciation on a daily basis?".to_string(),
            tips: Some("Thêm 'es' khi động từ tận cùng bằng: -o, -s, -z, -ch, -x, -sh (Mẹo nhớ: 'Ông Sáu Chạy Xe SH').\nVí dụ: watch ➡️ watches, fix ➡️ fixes, go ➡️ goes.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "2. Thì Hiện tại tiếp diễn (Present Continuous Tense)".to_string(),
            structure: "(+) S + am/is/are + V-ing | (-) S + am/is/are not + V-ing | (?) Am/Is/Are + S + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động đang diễn ra ngay tại thời điểm nói (now, right now, at the moment, Look!, Listen!).\n2. Hành động đang xảy ra xung quanh thời điểm nói dù không nhất thiết ngay lúc này (I am reading an inspiring book).\n3. Kế hoạch hoặc lịch hẹn chắc chắn trong tương lai gần (We are meeting the CEO tomorrow morning).\n4. Phàn nàn về một thói quen xấu lặp lại gây khó chịu với 'always' (He is always forgetting his room key!).".to_string(),
            examples: "+ Look! The children are playing joyfully in the freshly fallen snow.\n- She is not attending the lecture today because she feels under the weather.\n? Are you working on the final graduation thesis right now?".to_string(),
            tips: Some("TUYỆT ĐỐI KHÔNG dùng thì tiếp diễn với động từ trạng thái/cảm xúc (Stative verbs): know, believe, understand, love, hate, want, like, belong, smell, taste...".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "3. Thì Hiện tại hoàn thành (Present Perfect Tense)".to_string(),
            structure: "(+) S + have/has + V3/ed | (-) S + haven't/hasn't + V3/ed | (?) Have/Has + S + V3/ed?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động bắt đầu trong quá khứ và vẫn còn kéo dài đến hiện tại (since + mốc thời gian, for + khoảng thời gian).\n2. Hành động vừa mới xảy ra xong (just, recently, lately).\n3. Hành động đã hoàn tất tính đến thời điểm hiện tại (already, yet trong câu phủ định/nghi vấn).\n4. Trải nghiệm, kinh nghiệm từ trước tới nay (ever, never, so far, up to now).\n5. Nói về số lần đã làm gì đó (This is the first/second time...).".to_string(),
            examples: "+ She has worked as a senior software architect for more than eight years.\n- The research committee has not published the official findings yet.\n? Have you ever visited any English-speaking countries before?".to_string(),
            tips: Some("Phân biệt cốt lõi:\n- 'Have gone to': Đã đi đến nơi nào đó và hiện CHƯA VỀ.\n- 'Have been to': Đã từng đến nơi đó và HIỆN ĐÃ TRỞ VỀ.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s1.id,
            title: "4. Thì Hiện tại hoàn thành tiếp diễn (Present Perfect Continuous Tense)".to_string(),
            structure: "(+) S + have/has been + V-ing | (-) S + haven't/hasn't been + V-ing | (?) Have/Has + S + been + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Nhấn mạnh TÍNH LIÊN TỤC và KÉO DÀI của hành động bắt đầu từ quá khứ và vẫn đang tiếp diễn ở hiện tại.\n2. Hành động vừa mới kết thúc nhưng để lại KẾT QUẢ/HẬU QUẢ rõ rệt ngay tại hiện tại.\n3. Dấu hiệu nhận biết: all day, all morning, for hours, since early morning, how long...".to_string(),
            examples: "+ It has been raining non-stop for four hours and the roads are completely flooded.\n- He hasn't been sleeping well lately due to intense project deadlines.\n? How long have you been learning English through this application?".to_string(),
            tips: Some("Phân biệt với Present Perfect:\n- Present Perfect nhấn mạnh KẾT QUẢ/SỐ LƯỢNG: 'I have read 5 chapters'.\n- Present Perfect Continuous nhấn mạnh QUÁ TRÌNH/THỜI GIAN: 'I have been reading for 3 hours'.".to_string()),
        })?;

        // ==========================================
        // BUỔI 2: HỆ THỐNG 4 THÌ QUÁ KHỨ (PAST TENSES)
        // ==========================================
        let s2 = self.create_session(CreateSessionReq {
            session_number: Some(2),
            title: "Buổi 2: Hệ thống 4 thì Quá khứ (Past Simple, Continuous, Perfect, Perfect Continuous)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Ms. Sarah Jenkins".to_string()),
            goals: Some("Nắm vững trục thời gian quá khứ: Quá khứ đơn, Quá khứ tiếp diễn, Quá khứ hoàn thành và Quá khứ hoàn thành tiếp diễn.".to_string()),
            notes: Some("Quy tắc vàng: Hành động xảy ra trước một hành động khác trong quá khứ BẮT BUỘC dùng Quá khứ hoàn thành (Past Perfect).".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "chronological".to_string(),
            ipa: Some("/ˌkrɑːnəˈlɑːdʒɪkl/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "theo thứ tự thời gian, niên đại".to_string(),
            example_en: Some("The historical events are listed in chronological order.".to_string()),
            example_vi: Some("Các sự kiện lịch sử được liệt kê theo đúng thứ tự thời gian.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Collocation: in chronological order".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "retrospect".to_string(),
            ipa: Some("/ˈretrəspekt/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "sự nhìn lại dĩ vãng, hồi tưởng".to_string(),
            example_en: Some("In retrospect, that challenging decision was the turning point.".to_string()),
            example_vi: Some("Nhìn lại quá khứ, quyết định đầy thử thách đó chính là bước ngoặt.".to_string()),
            status: Some("reviewing".to_string()),
            notes: Some("Idiom: In retrospect (= nhìn lại quá khứ)".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s2.id,
            word: "unprecedented".to_string(),
            ipa: Some("/ʌnˈpresɪdentɪd/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "chưa từng có tiền lệ, vô tiền khoáng hậu".to_string(),
            example_en: Some("The economy experienced an unprecedented level of growth.".to_string()),
            example_vi: Some("Nền kinh tế đã trải qua mức tăng trưởng chưa từng có tiền lệ.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Academic IELTS Reading & Writing vocabulary".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "1. Thì Quá khứ đơn (Past Simple Tense)".to_string(),
            structure: "(+) S + V2/ed | (-) S + did not (didn't) + V_inf | (?) Did + S + V_inf?\nVới To-be: (+) S + was/were | (-) S + was/were not | (?) Was/Were + S...?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động đã bắt đầu và kết thúc trọn vẹn trong quá khứ, có mốc thời gian xác định rõ ràng.\n2. Chuỗi các hành động xảy ra kế tiếp nhau trong quá khứ (He locked the door, turned off the lights, and left).\n3. Dấu hiệu nhận biết: yesterday, last week/month/year, in 2020, ago, when I was a child...".to_string(),
            examples: "+ Our engineering team launched the production application three weeks ago.\n- She didn't participate in the symposium yesterday due to a prior commitment.\n? Did you receive the feedback document from Professor Miller?".to_string(),
            tips: Some("Khi đã mượn trợ động từ 'DID' trong câu phủ định hoặc nghi vấn, động từ chính BẮT BUỘC trở về dạng NGUYÊN THỂ (V_inf).".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "2. Thì Quá khứ tiếp diễn (Past Continuous Tense)".to_string(),
            structure: "(+) S + was/were + V-ing | (-) S + wasn't/weren't + V-ing | (?) Was/Were + S + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động đang xảy ra tại một thời điểm chính xác trong quá khứ (at 8 PM yesterday, at this time last week).\n2. Hai hay nhiều hành động diễn ra đồng thời song song trong quá khứ (kết hợp với 'While').\n3. Một hành động dài đang diễn ra trong quá khứ thì một hành động ngắn bất chợt xen vào.".to_string(),
            examples: "+ At 9 PM last night, I was reviewing my vocabulary flashcards for the final exam.\n+ While the teacher was explaining the grammar rules, the students were taking notes.\n? What were you doing when the emergency power outage occurred?".to_string(),
            tips: Some("Công thức phối thì kinh điển:\n- When + Quá khứ đơn (hành động ngắn cắt ngang), Quá khứ tiếp diễn (hành động dài đang xảy ra).\n- While + Quá khứ tiếp diễn, Quá khứ đơn.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "3. Thì Quá khứ hoàn thành (Past Perfect Tense)".to_string(),
            structure: "(+) S + had + V3/ed | (-) S + had not (hadn't) + V3/ed | (?) Had + S + V3/ed?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động đã xảy ra và hoàn tất TRƯỚC một hành động khác trong quá khứ (Hành động trước: Past Perfect; Hành động sau: Past Simple).\n2. Hành động xảy ra trước một mốc thời gian xác định trong quá khứ.\n3. Dấu hiệu nhận biết: by the time, before, after, by 2015, until then...".to_string(),
            examples: "+ By the time the rescue team arrived at the summit, the storm had already passed.\n+ After she had completed her master's thesis, she travelled around Europe.\n? Had you ever studied computer science before you joined this university?".to_string(),
            tips: Some("Công thức vàng dễ thuộc:\n- By the time / Before + Quá khứ đơn, Quá khứ hoàn thành.\n- After + Quá khứ hoàn thành, Quá khứ đơn.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s2.id,
            title: "4. Thì Quá khứ hoàn thành tiếp diễn (Past Perfect Continuous Tense)".to_string(),
            structure: "(+) S + had been + V-ing | (-) S + hadn't been + V-ing | (?) Had + S + been + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Nhấn mạnh TÍNH LIÊN TỤC và KÉO DÀI của một hành động xảy ra trước một thời điểm hoặc một hành động khác trong quá khứ.\n2. Chỉ nguyên nhân trực tiếp cho một trạng thái/kết quả trong quá khứ (He was exhausted because he had been driving for 8 hours).".to_string(),
            examples: "+ When the train finally arrived, passengers had been waiting on the platform for two hours.\n+ Her eyes were strained because she had been staring at the monitor all morning.\n? How long had they been negotiating before they officially signed the contract?".to_string(),
            tips: Some("Khi câu muốn nhấn mạnh 'khoảng thời gian bao lâu' (for hours, for 5 years) trước một mốc quá khứ, hãy ưu tiên dùng Quá khứ hoàn thành tiếp diễn.".to_string()),
        })?;

        // ==========================================
        // BUỔI 3: HỆ THỐNG 4 THÌ TƯƠNG LAI (FUTURE TENSES)
        // ==========================================
        let s3 = self.create_session(CreateSessionReq {
            session_number: Some(3),
            title: "Buổi 3: Hệ thống 4 thì Tương lai (Future Simple, Continuous, Perfect, Perfect Continuous)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Dr. Alexander Hayes".to_string()),
            goals: Some("Thành thạo Will, Be going to, Tương lai tiếp diễn, Tương lai hoàn thành và Tương lai hoàn thành tiếp diễn.".to_string()),
            notes: Some("Trong mệnh đề thời gian (when, as soon as, before, by the time), động từ chia ở Hiện tại đơn thay vì dùng Will.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "anticipate".to_string(),
            ipa: Some("/ænˈtɪsɪpeɪt/".to_string()),
            pos: Some("verb".to_string()),
            meaning: "lường trước, dự đoán trước".to_string(),
            example_en: Some("We anticipate that artificial intelligence will transform education.".to_string()),
            example_vi: Some("Chúng tôi lường trước rằng trí tuệ nhân tạo sẽ thay đổi nền giáo dục.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Synonym: foresee, predict, expect".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "feasible".to_string(),
            ipa: Some("/ˈfiːzəbl/".to_string()),
            pos: Some("adjective".to_string()),
            meaning: "khả thi, có thể thực hiện được".to_string(),
            example_en: Some("With disciplined execution, our futuristic goals are entirely feasible.".to_string()),
            example_vi: Some("Với việc thực thi có kỷ luật, các mục tiêu tương lai của chúng ta hoàn toàn khả thi.".to_string()),
            status: Some("learning".to_string()),
            notes: Some("Antonym: infeasible, impossible".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s3.id,
            word: "milestone".to_string(),
            ipa: Some("/ˈmaɪlstəʊn/".to_string()),
            pos: Some("noun".to_string()),
            meaning: "cột mốc quan trọng, dấu mốc lịch sử".to_string(),
            example_en: Some("Graduating from university will be a major milestone in your life.".to_string()),
            example_vi: Some("Tốt nghiệp đại học sẽ là một cột mốc trọng đại trong cuộc đời bạn.".to_string()),
            status: Some("mastered".to_string()),
            notes: Some("Collocation: reach / achieve a milestone".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "1. Thì Tương lai đơn (Future Simple) & Be Going To".to_string(),
            structure: "Will: (+) S + will + V_inf | (-) S + won't + V_inf | (?) Will + S + V_inf?\nBe going to: (+) S + am/is/are going to + V_inf | (-) S + am/is/are not going to + V_inf".to_string(),
            explanation: "Phân biệt bản chất 2 cấu trúc tương lai phổ biến nhất:\n1. Will: Quyết định bộc phát ngay tại thời điểm nói (không có dự định trước), lời hứa, đề nghị giúp đỡ, dự đoán chủ quan cảm tính.\n2. Be going to: Dự định, kế hoạch đã được tính toán từ trước khi nói, hoặc dự đoán dựa trên BẰNG CHỨNG RÕ RÀNG trước mắt (Look at the dark clouds!).".to_string(),
            examples: "+ Look at those black clouds! It is going to rain very heavily.\n+ Don't worry about the heavy suitcases, I will give you a hand.\n- We are not going to deploy the update until all unit tests pass completely.".to_string(),
            tips: Some("LUẬT BẤT THÀNH VĂN: Trong mệnh đề trạng ngữ chỉ thời gian (bắt đầu bằng When, As soon as, Before, After, Until) ở tương lai, động từ PHẢI chia ở Hiện tại đơn, KHÔNG dùng 'will'.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "2. Thì Tương lai tiếp diễn (Future Continuous Tense)".to_string(),
            structure: "(+) S + will be + V-ing | (-) S + won't be + V-ing | (?) Will + S + be + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động đang diễn ra tại một thời điểm cụ thể xác định trong tương lai (at 9 AM tomorrow, this time next week).\n2. Hành động đang xảy ra trong tương lai thì có một hành động khác xen vào (mệnh đề xen vào chia Hiện tại đơn).\n3. Hành động diễn ra như một phần trong lịch trình thông thường không thể tránh khỏi.".to_string(),
            examples: "+ This time next Monday, I will be flying across the Pacific to San Francisco.\n+ When you arrive at the station tomorrow, our representative will be waiting for you.\n? Will you be using the laboratory computer tomorrow morning?".to_string(),
            tips: Some("Dấu hiệu nhận biết đặc trưng: 'At this time tomorrow', 'At 8 o'clock tonight', 'At this moment next year'...".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "3. Thì Tương lai hoàn thành (Future Perfect Tense)".to_string(),
            structure: "(+) S + will have + V3/ed | (-) S + won't have + V3/ed | (?) Will + S + have + V3/ed?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Hành động sẽ được hoàn tất TRƯỚC một mốc thời gian hoặc trước một hành động khác trong tương lai.\n2. Dấu hiệu nhận biết: By + mốc tương lai (By next month, By 2030), By the time + mệnh đề hiện tại đơn, Before + mốc tương lai.".to_string(),
            examples: "+ By the end of this semester, we will have completed all core coursework.\n+ By the time you get home tonight, dinner will have been prepared.\n? Will you have submitted your final application before the deadline expires?".to_string(),
            tips: Some("Công thức vàng trong bài thi TOEIC & IELTS:\n'By the time + S + V(hiện tại đơn), S + will have + V3/ed'.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s3.id,
            title: "4. Thì Tương lai hoàn thành tiếp diễn (Future Perfect Continuous Tense)".to_string(),
            structure: "(+) S + will have been + V-ing | (-) S + won't have been + V-ing | (?) Will + S + have been + V-ing?".to_string(),
            explanation: "Dùng để diễn tả:\n1. Nhấn mạnh khoảng thời gian kéo dài liên tục của một hành động tính tới một mốc thời điểm xác định trong tương lai.\n2. Dấu hiệu nhận biết: By the end of this year, for 10 years; By next month, for 3 years...".to_string(),
            examples: "+ By next October, Dr. Watson will have been lecturing at this institute for twenty years.\n+ By 8 PM tonight, they will have been coding continuously for twelve hours.\n? How long will you have been residing in this apartment by next summer?".to_string(),
            tips: Some("Thì này là 'vũ khí bí mật' giúp nâng cao band điểm Grammatical Range and Accuracy trong bài thi IELTS Writing Task 2 khi cần nhấn mạnh khoảng thời gian kéo dài liên tục.".to_string()),
        })?;

        // ==========================================
        // BUỔI 4: MỆNH ĐỀ QUAN HỆ & RÚT GỌN (RELATIVE CLAUSES)
        // ==========================================
        let s4 = self.create_session(CreateSessionReq {
            session_number: Some(4),
            title: "Buổi 4: Mệnh đề quan hệ & Rút gọn mệnh đề (Relative Clauses)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Prof. Arthur Pendelton".to_string()),
            goals: Some("Thành thạo đại từ/trạng từ quan hệ (Who, Whom, Which, That, Whose, Where...) và 3 quy tắc rút gọn mệnh đề đỉnh cao.".to_string()),
            notes: Some("Tuyệt đối không dùng 'That' sau dấu phẩy (mệnh đề không xác định) và sau giới từ.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s4.id,
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
            session_id: s4.id,
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
            session_id: s4.id,
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
            session_id: s4.id,
            title: "1. Đại từ & Trạng từ quan hệ (Who, Whom, Which, That, Whose...)".to_string(),
            structure: "Người: WHO + V / WHOM + S + V | Vật: WHICH/THAT + V/S+V | Sở hữu: N + WHOSE + N | Trạng từ: WHERE, WHEN, WHY".to_string(),
            explanation: "Dùng để nối 2 câu đơn thành câu phức gãy gọn. Phân biệt:\n- Mệnh đề xác định (Defining): Không có dấu phẩy, bắt buộc phải có để câu rõ nghĩa, dùng được 'that'.\n- Mệnh đề không xác định (Non-defining): Có dấu phẩy phân tách, chỉ bổ sung thông tin cho danh từ đã rõ ràng (tên riêng, có this/that/my...).".to_string(),
            examples: "+ The engineer who developed this application received the award.\n+ Dr. Harrison, whom we interviewed yesterday, has accepted the position.\n+ The company whose headquarters is in Tokyo announced a new strategy.".to_string(),
            tips: Some("LUẬT VÀNG: Sau dấu phẩy và sau giới từ (in, on, with...) TUYỆT ĐỐI KHÔNG BAO GIỜ dùng 'THAT'.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s4.id,
            title: "2. Rút gọn mệnh đề quan hệ đỉnh cao (Reduced Relative Clauses)".to_string(),
            structure: "1. Chủ động ➡️ V-ing | 2. Bị động ➡️ V3/ed | 3. Sau the first, last, only, so sánh nhất ➡️ To V".to_string(),
            explanation: "Biến đổi giúp câu văn ngắn gọn, học thuật và đạt điểm cao trong bài thi tiếng Anh:\n- Chủ động: The girl who is talking to Nam ➡️ The girl talking to Nam.\n- Bị động: The laptops which were manufactured in 2025 ➡️ The laptops manufactured in 2025.\n- Cụm thứ tự: Yuri Gagarin was the first person who flew into space ➡️ the first person to fly into space.".to_string(),
            examples: "+ Active: Documents submitted after 5 PM will not be processed.\n+ Passive: The keynote address delivered by the CEO inspired everyone.\n+ To-V: She is the only candidate to achieve a perfect band score.".to_string(),
            tips: Some("Luôn xác định xem danh từ phía trước là CHỦ THỂ (hành động ➡️ V-ing) hay ĐỐI TƯỢNG (bị tác động ➡️ V3/ed).".to_string()),
        })?;

        // ==========================================
        // BUỔI 5: QUY TẮC PHÁT ÂM ĐUÔI -ED
        // ==========================================
        let s5 = self.create_session(CreateSessionReq {
            session_number: Some(5),
            title: "Buổi 5: Quy tắc phát âm đuôi -ED chuẩn quốc tế (/id/, /t/, /d/)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Dr. Rachel Green".to_string()),
            goals: Some("Nắm trọn vẹn 3 quy tắc phát âm đuôi -ED theo ngữ âm học quốc tế và danh sách tính từ ngoại lệ.".to_string()),
            notes: Some("Ghi nhớ câu thần chú âm vô thanh: 'Chính Phủ Phát Sách Không Thèm / Thảo' để đọc đuôi /t/ chuẩn xác.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s5.id,
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
            session_id: s5.id,
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
            session_id: s5.id,
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
            session_id: s5.id,
            title: "1. Ba quy tắc vàng phát âm đuôi -ED (/id/, /t/, /d/)".to_string(),
            structure: "1. Đọc là /id/: sau âm /t/ và /d/\n2. Đọc là /t/: sau âm vô thanh (/p, k, f, s, ʃ, tʃ/)\n3. Đọc là /d/: các âm hữu thanh còn lại".to_string(),
            explanation: "- Nhóm 1 (/id/): wanted, needed, decided, started, invited, landed.\n- Nhóm 2 (/t/): Âm kết thúc là các âm vô thanh (không rung thanh quản, mẹo: 'Chính Phủ Phát Sách Không Thèm / Thảo'): stopped, looked, laughed, washed, watched, kissed, fixed.\n- Nhóm 3 (/d/): Các âm hữu thanh (rung cổ họng + nguyên âm): played, opened, lived, cleaned, called, enjoyed, preferred.".to_string(),
            examples: "+ /id/: They decided to postpone the tournament until next week.\n+ /t/: He walked to the office, checked his email, and fixed the bug.\n+ /d/: She called her parents and explained the whole situation.".to_string(),
            tips: Some("Nhìn vào ÂM CUỐI CÙNG được phát âm, KHÔNG nhìn vào chữ cái viết (ví dụ: 'laughed' tận cùng chữ gh nhưng phát âm là /f/ ➡️ đọc là /t/).".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s5.id,
            title: "2. Danh sách tính từ ngoại lệ bắt buộc phát âm là /id/".to_string(),
            structure: "Danh sách từ đặc biệt: naked, wicked, learned, ragged, sacred, blessed, crooked...".to_string(),
            explanation: "Khi các từ này đóng vai trò là TÍNH TỪ (Adjectives), đuôi -ed luôn phát âm là /id/ thay vì /t/ hay /d/ như quy tắc thông thường.".to_string(),
            examples: "+ naked /ˈneɪkɪd/ (trần trụi, khỏa thân)\n+ wicked /ˈwɪkɪd/ (độc ác, xấu xa)\n+ learned /ˈlɜːnɪd/ (uyên bác, có học thức - e.g. a learned professor)\n+ ragged /ˈræɡɪd/ (rách rưới, tả tơi)\n+ sacred /ˈseɪkrɪd/ (thiêng liêng, trang trọng)\n+ blessed /ˈblesɪd/ (may mắn, thần thánh)\n+ crooked /ˈkrʊkɪd/ (quanh co, cong queo)".to_string(),
            tips: Some("Trong bài thi trắc nghiệm phát âm THPTQG và TOEIC, các từ ngoại lệ này xuất hiện với tần suất trên 80%!".to_string()),
        })?;

        // ==========================================
        // BUỔI 6: NỐI ÂM & NGỮ ĐIỆU (CONNECTED SPEECH)
        // ==========================================
        let s6 = self.create_session(CreateSessionReq {
            session_number: Some(6),
            title: "Buổi 6: Quy tắc đọc nối đuôi & Nói chuẩn ngữ điệu (Connected Speech)".to_string(),
            date: Some(today.clone()),
            teacher: Some("Coach Emma Watson".to_string()),
            goals: Some("Làm chủ 4 quy tắc nối âm: Phụ âm - Nguyên âm, Nguyên âm - Nguyên âm, Biến âm (Assimilation) và Nuốt âm (Elision).".to_string()),
            notes: Some("Luyện phát âm theo cụm luồng hơi liền mạch thay vì ngắt từng từ đơn lẻ.".to_string()),
        })?;

        self.create_vocab(CreateVocabReq {
            session_id: s6.id,
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
            session_id: s6.id,
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
            session_id: s6.id,
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
            session_id: s6.id,
            title: "1. Nối âm Phụ âm - Nguyên âm & Nguyên âm - Nguyên âm".to_string(),
            structure: "Phụ âm + Nguyên âm: C + V ➡️ Nối dính | Nguyên âm + Nguyên âm: V + V ➡️ Chèn âm /j/ hoặc /w/".to_string(),
            explanation: "1. Phụ âm nối nguyên âm (C+V): 'Hold on' ➡️ /həʊl-dɒn/, 'Turn off' ➡️ /tɜːn-nɒf/, 'An apple' ➡️ /ən-næpl/.\n2. Nguyên âm nối nguyên âm (V+V):\n   - Miệng dẹt (/i:, ɪ, eɪ, aɪ/) ➡️ Chèn âm /j/: 'I agree' ➡️ /aɪ-j-əˈɡriː/, 'See it' ➡️ /siː-j-ɪt/.\n   - Miệng tròn (/u:, əʊ, aʊ/) ➡️ Chèn âm /w/: 'Go out' ➡️ /ɡəʊ-w-aʊt/, 'You are' ➡️ /juː-w-ɑːr/.".to_string(),
            examples: "+ C+V: Can I have a cup of water? ➡️ /kæn-aɪ hæv-ə kʌp-əv wɔːtər/\n+ V+V (/j/): She asked me to say it again. ➡️ /seɪ-j-ɪt/\n+ V+V (/w/): Let's do it right away. ➡️ /duː-w-ɪt/".to_string(),
            tips: Some("Khi nói tiếng Anh, đừng cố phát âm rời rạc như gõ máy; hãy để luồng hơi nối trơn tru giữa các âm tiết.".to_string()),
        })?;

        self.create_grammar(CreateGrammarReq {
            session_id: s6.id,
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
