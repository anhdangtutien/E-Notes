/**
 * E-NOTES: Minimalist Vanilla JS Client
 * Dual-Mode Engine:
 * 1. Full REST API mode when running with Rust Backend (Local / Docker)
 * 2. Automatic LocalStorage Edge mode when deployed on Vercel / Static CDN
 */

// Application State
const state = {
  sessions: [],
  currentSessionId: null,
  currentDetail: null,
  activeTab: 'vocab',
  flashcardIdx: 0,
  flashcardFlipped: false,
};

let useLocalStore = false;

// Seed data for standalone/Vercel mode with complete core curriculum (12 Tenses + Rel Clauses + Pronunciation + Linking)
const SEED_DATA = {
  sessions: [
    {
      id: 1,
      session_number: 1,
      title: "Buổi 1: Hệ thống 4 thì Hiện tại (Present Simple, Continuous, Perfect, Perfect Continuous)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Mr. David Miller",
      goals: "Làm chủ trọn vẹn bản chất, công thức và ngữ cảnh sử dụng của cả 4 thì Hiện tại trong tiếng Anh chuẩn quốc tế.",
      notes: "Phân biệt rõ: Hiện tại đơn (thói quen, chân lý) vs Tiếp diễn (tại thời điểm nói) vs Hoàn thành (kết quả đến nay) vs Hoàn thành tiếp diễn (quá trình liên tục).",
      created_at: "2026-11-25 08:00:00",
      updated_at: "2026-11-25 08:00:00"
    },
    {
      id: 2,
      session_number: 2,
      title: "Buổi 2: Hệ thống 4 thì Quá khứ (Past Simple, Continuous, Perfect, Perfect Continuous)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Ms. Sarah Jenkins",
      goals: "Nắm vững trục thời gian quá khứ: Quá khứ đơn, Quá khứ tiếp diễn, Quá khứ hoàn thành và Quá khứ hoàn thành tiếp diễn.",
      notes: "Quy tắc vàng: Hành động xảy ra trước một hành động khác trong quá khứ BẮT BUỘC dùng Quá khứ hoàn thành (Past Perfect).",
      created_at: "2026-11-25 09:00:00",
      updated_at: "2026-11-25 09:00:00"
    },
    {
      id: 3,
      session_number: 3,
      title: "Buổi 3: Hệ thống 4 thì Tương lai (Future Simple, Continuous, Perfect, Perfect Continuous)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Dr. Alexander Hayes",
      goals: "Thành thạo Will, Be going to, Tương lai tiếp diễn, Tương lai hoàn thành và Tương lai hoàn thành tiếp diễn.",
      notes: "Trong mệnh đề thời gian (when, as soon as, before, by the time), động từ chia ở Hiện tại đơn thay vì dùng Will.",
      created_at: "2026-11-25 10:00:00",
      updated_at: "2026-11-25 10:00:00"
    },
    {
      id: 4,
      session_number: 4,
      title: "Buổi 4: Mệnh đề quan hệ & Rút gọn mệnh đề (Relative Clauses)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Prof. Arthur Pendelton",
      goals: "Thành thạo đại từ/trạng từ quan hệ (Who, Whom, Which, That, Whose, Where...) và 3 quy tắc rút gọn mệnh đề đỉnh cao.",
      notes: "Tuyệt đối không dùng 'That' sau dấu phẩy (mệnh đề không xác định) và sau giới từ.",
      created_at: "2026-11-25 11:00:00",
      updated_at: "2026-11-25 11:00:00"
    },
    {
      id: 5,
      session_number: 5,
      title: "Buổi 5: Quy tắc phát âm đuôi -ED chuẩn quốc tế (/id/, /t/, /d/)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Dr. Rachel Green",
      goals: "Nắm trọn vẹn 3 quy tắc phát âm đuôi -ED theo ngữ âm học quốc tế và danh sách tính từ ngoại lệ.",
      notes: "Ghi nhớ câu thần chú âm vô thanh: 'Chính Phủ Phát Sách Không Thèm / Thảo' để đọc đuôi /t/ chuẩn xác.",
      created_at: "2026-11-25 12:00:00",
      updated_at: "2026-11-25 12:00:00"
    },
    {
      id: 6,
      session_number: 6,
      title: "Buổi 6: Quy tắc đọc nối đuôi & Nói chuẩn ngữ điệu (Connected Speech)",
      date: new Date().toISOString().split('T')[0],
      teacher: "Coach Emma Watson",
      goals: "Làm chủ 4 quy tắc nối âm: Phụ âm - Nguyên âm, Nguyên âm - Nguyên âm, Biến âm (Assimilation) và Nuốt âm (Elision).",
      notes: "Luyện phát âm theo cụm luồng hơi liền mạch thay vì ngắt từng từ đơn lẻ.",
      created_at: "2026-11-25 13:00:00",
      updated_at: "2026-11-25 13:00:00"
    }
  ],
  vocabularies: [
    // Buổi 1: Hiện tại
    {
      id: 1,
      session_id: 1,
      word: "prioritize",
      ipa: "/praɪˈɔːrətaɪz/",
      pos: "verb",
      meaning: "ưu tiên việc gì đó lên hàng đầu",
      example_en: "You must prioritize tasks based on their urgency and importance.",
      example_vi: "Bạn phải ưu tiên các nhiệm vụ dựa trên mức độ khẩn cấp và tầm quan trọng.",
      status: "mastered",
      notes: "Collocation: prioritize A over B",
      created_at: "2026-11-25 08:10:00"
    },
    {
      id: 2,
      session_id: 1,
      word: "simultaneously",
      ipa: "/ˌsaɪmlˈteɪniəsli/",
      pos: "adverb",
      meaning: "đồng thời, cùng một lúc",
      example_en: "Two research projects were carried out simultaneously.",
      example_vi: "Hai dự án nghiên cứu đã được tiến hành cùng một lúc.",
      status: "learning",
      notes: "Synonym: at the same time, concurrently",
      created_at: "2026-11-25 08:15:00"
    },
    {
      id: 3,
      session_id: 1,
      word: "procrastinate",
      ipa: "/prəˈkræstɪneɪt/",
      pos: "verb",
      meaning: "trì hoãn, chần chừ",
      example_en: "People often procrastinate when facing difficult decisions.",
      example_vi: "Mọi người thường chần chừ khi phải đối mặt với các quyết định khó khăn.",
      status: "learning",
      notes: "Noun form: procrastination",
      created_at: "2026-11-25 08:20:00"
    },

    // Buổi 2: Quá khứ
    {
      id: 4,
      session_id: 2,
      word: "chronological",
      ipa: "/ˌkrɑːnəˈlɑːdʒɪkl/",
      pos: "adjective",
      meaning: "theo thứ tự thời gian, niên đại",
      example_en: "The historical events are listed in chronological order.",
      example_vi: "Các sự kiện lịch sử được liệt kê theo đúng thứ tự thời gian.",
      status: "mastered",
      notes: "Collocation: in chronological order",
      created_at: "2026-11-25 09:10:00"
    },
    {
      id: 5,
      session_id: 2,
      word: "retrospect",
      ipa: "/ˈretrəspekt/",
      pos: "noun",
      meaning: "sự nhìn lại dĩ vãng, hồi tưởng",
      example_en: "In retrospect, that challenging decision was the turning point.",
      example_vi: "Nhìn lại quá khứ, quyết định đầy thử thách đó chính là bước ngoặt.",
      status: "reviewing",
      notes: "Idiom: In retrospect (= nhìn lại quá khứ)",
      created_at: "2026-11-25 09:15:00"
    },
    {
      id: 6,
      session_id: 2,
      word: "unprecedented",
      ipa: "/ʌnˈpresɪdentɪd/",
      pos: "adjective",
      meaning: "chưa từng có tiền lệ, vô tiền khoáng hậu",
      example_en: "The economy experienced an unprecedented level of growth.",
      example_vi: "Nền kinh tế đã trải qua mức tăng trưởng chưa từng có tiền lệ.",
      status: "learning",
      notes: "Academic IELTS Reading & Writing vocabulary",
      created_at: "2026-11-25 09:20:00"
    },

    // Buổi 3: Tương lai
    {
      id: 7,
      session_id: 3,
      word: "anticipate",
      ipa: "/ænˈtɪsɪpeɪt/",
      pos: "verb",
      meaning: "lường trước, dự đoán trước",
      example_en: "We anticipate that artificial intelligence will transform education.",
      example_vi: "Chúng tôi lường trước rằng trí tuệ nhân tạo sẽ thay đổi nền giáo dục.",
      status: "mastered",
      notes: "Synonym: foresee, predict, expect",
      created_at: "2026-11-25 10:10:00"
    },
    {
      id: 8,
      session_id: 3,
      word: "feasible",
      ipa: "/ˈfiːzəbl/",
      pos: "adjective",
      meaning: "khả thi, có thể thực hiện được",
      example_en: "With disciplined execution, our futuristic goals are entirely feasible.",
      example_vi: "Với việc thực thi có kỷ luật, các mục tiêu tương lai của chúng ta hoàn toàn khả thi.",
      status: "learning",
      notes: "Antonym: infeasible, impossible",
      created_at: "2026-11-25 10:15:00"
    },
    {
      id: 9,
      session_id: 3,
      word: "milestone",
      ipa: "/ˈmaɪlstəʊn/",
      pos: "noun",
      meaning: "cột mốc quan trọng, dấu mốc lịch sử",
      example_en: "Graduating from university will be a major milestone in your life.",
      example_vi: "Tốt nghiệp đại học sẽ là một cột mốc trọng đại trong cuộc đời bạn.",
      status: "mastered",
      notes: "Collocation: reach / achieve a milestone",
      created_at: "2026-11-25 10:20:00"
    },

    // Buổi 4: Mệnh đề quan hệ
    {
      id: 10,
      session_id: 4,
      word: "comprehensive",
      ipa: "/ˌkɑːmprɪˈhensɪv/",
      pos: "adjective",
      meaning: "toàn diện, bao quát mọi khía cạnh",
      example_en: "The study provides a comprehensive overview of global climate patterns.",
      example_vi: "Nghiên cứu cung cấp một cái nhìn toàn diện về các mô hình khí hậu toàn cầu.",
      status: "mastered",
      notes: "Tránh nhầm với 'comprehensible' (dễ hiểu)",
      created_at: "2026-11-25 11:10:00"
    },
    {
      id: 11,
      session_id: 4,
      word: "indispensable",
      ipa: "/ˌɪndɪˈspensəbl/",
      pos: "adjective",
      meaning: "không thể thiếu được, thiết yếu",
      example_en: "Critical thinking is indispensable for academic success.",
      example_vi: "Tư duy phản biện là điều không thể thiếu cho thành công trong học tập.",
      status: "reviewing",
      notes: "Collocation: an indispensable tool / element",
      created_at: "2026-11-25 11:15:00"
    },
    {
      id: 12,
      session_id: 4,
      word: "facilitate",
      ipa: "/fəˈsɪlɪteɪt/",
      pos: "verb",
      meaning: "tạo điều kiện thuận lợi, hỗ trợ dễ dàng hơn",
      example_en: "The new digital tools facilitate collaboration among remote teams.",
      example_vi: "Các công cụ kỹ thuật số mới tạo điều kiện thuận lợi cho việc cộng tác giữa các đội nhóm từ xa.",
      status: "learning",
      notes: "Formal academic verb",
      created_at: "2026-11-25 11:20:00"
    },

    // Buổi 5: Phát âm -ED
    {
      id: 13,
      session_id: 5,
      word: "articulate",
      ipa: "/ɑːrˈtɪkjuleɪt/",
      pos: "verb / adjective",
      meaning: "phát âm rõ ràng, diễn đạt lưu loát gãy gọn",
      example_en: "She gave an articulate and persuasive presentation yesterday.",
      example_vi: "Cô ấy đã có một bài thuyết trình lưu loát và đầy sức thuyết phục ngày hôm qua.",
      status: "mastered",
      notes: "V: /ɑːrˈtɪkjuleɪt/ - Adj: /ɑːrˈtɪkjulət/",
      created_at: "2026-11-25 12:10:00"
    },
    {
      id: 14,
      session_id: 5,
      word: "distinguish",
      ipa: "/dɪˈstɪŋɡwɪʃ/",
      pos: "verb",
      meaning: "phân biệt, nhận diện sự khác nhau",
      example_en: "Can you distinguish between voiceless and voiced consonants?",
      example_vi: "Bạn có thể phân biệt giữa phụ âm vô thanh và hữu thanh không?",
      status: "learning",
      notes: "Collocation: distinguish between A and B",
      created_at: "2026-11-25 12:15:00"
    },
    {
      id: 15,
      session_id: 5,
      word: "exception",
      ipa: "/ɪkˈsepʃn/",
      pos: "noun",
      meaning: "trường hợp ngoại lệ",
      example_en: "Words like 'naked' and 'wicked' are special exceptions in -ed pronunciation.",
      example_vi: "Các từ như 'naked' và 'wicked' là những ngoại lệ đặc biệt trong cách phát âm đuôi -ed.",
      status: "mastered",
      notes: "Idiom: With the exception of (= ngoại trừ)",
      created_at: "2026-11-25 12:20:00"
    },

    // Buổi 6: Nối âm
    {
      id: 16,
      session_id: 6,
      word: "fluency",
      ipa: "/ˈfluːənsi/",
      pos: "noun",
      meaning: "sự lưu loát, trôi chảy trong ngôn ngữ",
      example_en: "Linking sounds properly will significantly boost your speaking fluency.",
      example_vi: "Việc nối âm đúng cách sẽ nâng cao đáng kể sự trôi chảy khi bạn nói.",
      status: "mastered",
      notes: "Adjective: fluent",
      created_at: "2026-11-25 13:10:00"
    },
    {
      id: 17,
      session_id: 6,
      word: "intonation",
      ipa: "/ˌɪntəˈneɪʃn/",
      pos: "noun",
      meaning: "ngữ điệu lên xuống của câu nói",
      example_en: "Natural intonation helps listeners grasp your emotions and intentions.",
      example_vi: "Ngữ điệu tự nhiên giúp người nghe nắm bắt cảm xúc và chủ ý của bạn.",
      status: "reviewing",
      notes: "Rising intonation (câu hỏi Yes/No) vs Falling intonation (câu trần thuật)",
      created_at: "2026-11-25 13:15:00"
    },
    {
      id: 18,
      session_id: 6,
      word: "spontaneous",
      ipa: "/spɑːnˈteɪniəs/",
      pos: "adjective",
      meaning: "tự nhiên, bộc phát không cần chuẩn bị trước",
      example_en: "Daily practice makes your conversational responses more spontaneous.",
      example_vi: "Việc luyện tập mỗi ngày giúp phản xạ giao tiếp của bạn trở nên tự nhiên hơn.",
      status: "learning",
      notes: "Adverb: spontaneously",
      created_at: "2026-11-25 13:20:00"
    }
  ],
  grammars: [
    // ==========================================
    // BUỔI 1: HỆ THỐNG 4 THÌ HIỆN TẠI (PRESENT TENSES)
    // ==========================================
    {
      id: 1,
      session_id: 1,
      title: "1. Thì Hiện tại đơn (Present Simple Tense)",
      structure: "(+) S + V(s/es) | (-) S + do/does not + V_inf | (?) Do/Does + S + V_inf?\nVới To-be: (+) S + am/is/are | (-) S + am/is/are not | (?) Am/Is/Are + S...?",
      explanation: "Dùng để diễn tả:\n1. Chân lý khoa học, sự thật hiển nhiên (Water boils at 100°C).\n2. Thói quen, hành động lặp đi lặp lại thường xuyên (I brush my teeth twice a day).\n3. Lịch trình tàu xe, máy bay, thời khóa biểu cố định (The train departs at 6:00 PM).\n4. Dấu hiệu nhận biết: always, usually, often, sometimes, seldom, never, every day/week, once a year...",
      examples: "+ Water boils at 100 degrees Celsius under standard atmospheric pressure.\n- He does not drink coffee in the evening because it disrupts his sleep.\n? Do you practice English pronunciation on a daily basis?",
      tips: "Thêm 'es' khi động từ tận cùng bằng: -o, -s, -z, -ch, -x, -sh (Mẹo nhớ: 'Ông Sáu Chạy Xe SH').\nVí dụ: watch ➡️ watches, fix ➡️ fixes, go ➡️ goes.",
      created_at: "2026-11-25 08:30:00"
    },
    {
      id: 2,
      session_id: 1,
      title: "2. Thì Hiện tại tiếp diễn (Present Continuous Tense)",
      structure: "(+) S + am/is/are + V-ing | (-) S + am/is/are not + V-ing | (?) Am/Is/Are + S + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Hành động đang diễn ra ngay tại thời điểm nói (now, right now, at the moment, Look!, Listen!).\n2. Hành động đang xảy ra xung quanh thời điểm nói dù không nhất thiết ngay lúc này (I am reading an inspiring book).\n3. Kế hoạch hoặc lịch hẹn chắc chắn trong tương lai gần (We are meeting the CEO tomorrow morning).\n4. Phàn nàn về một thói quen xấu lặp lại gây khó chịu với 'always' (He is always forgetting his room key!).",
      examples: "+ Look! The children are playing joyfully in the freshly fallen snow.\n- She is not attending the lecture today because she feels under the weather.\n? Are you working on the final graduation thesis right now?",
      tips: "TUYỆT ĐỐI KHÔNG dùng thì tiếp diễn với động từ trạng thái/cảm xúc (Stative verbs): know, believe, understand, love, hate, want, like, belong, smell, taste...",
      created_at: "2026-11-25 08:35:00"
    },
    {
      id: 3,
      session_id: 1,
      title: "3. Thì Hiện tại hoàn thành (Present Perfect Tense)",
      structure: "(+) S + have/has + V3/ed | (-) S + haven't/hasn't + V3/ed | (?) Have/Has + S + V3/ed?",
      explanation: "Dùng để diễn tả:\n1. Hành động bắt đầu trong quá khứ và vẫn còn kéo dài đến hiện tại (since + mốc thời gian, for + khoảng thời gian).\n2. Hành động vừa mới xảy ra xong (just, recently, lately).\n3. Hành động đã hoàn tất tính đến thời điểm hiện tại (already, yet trong câu phủ định/nghi vấn).\n4. Trải nghiệm, kinh nghiệm từ trước tới nay (ever, never, so far, up to now).\n5. Nói về số lần đã làm gì đó (This is the first/second time...).",
      examples: "+ She has worked as a senior software architect for more than eight years.\n- The research committee has not published the official findings yet.\n? Have you ever visited any English-speaking countries before?",
      tips: "Phân biệt cốt lõi:\n- 'Have gone to': Đã đi đến nơi nào đó và hiện CHƯA VỀ.\n- 'Have been to': Đã từng đến nơi đó và HIỆN ĐÃ TRỞ VỀ.",
      created_at: "2026-11-25 08:40:00"
    },
    {
      id: 4,
      session_id: 1,
      title: "4. Thì Hiện tại hoàn thành tiếp diễn (Present Perfect Continuous Tense)",
      structure: "(+) S + have/has been + V-ing | (-) S + haven't/hasn't been + V-ing | (?) Have/Has + S + been + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Nhấn mạnh TÍNH LIÊN TỤC và KÉO DÀI của hành động bắt đầu từ quá khứ và vẫn đang tiếp diễn ở hiện tại.\n2. Hành động vừa mới kết thúc nhưng để lại KẾT QUẢ/HẬU QUẢ rõ rệt ngay tại hiện tại.\n3. Dấu hiệu nhận biết: all day, all morning, for hours, since early morning, how long...",
      examples: "+ It has been raining non-stop for four hours and the roads are completely flooded.\n- He hasn't been sleeping well lately due to intense project deadlines.\n? How long have you been learning English through this application?",
      tips: "Phân biệt với Present Perfect:\n- Present Perfect nhấn mạnh KẾT QUẢ/SỐ LƯỢNG: 'I have read 5 chapters'.\n- Present Perfect Continuous nhấn mạnh QUÁ TRÌNH/THỜI GIAN: 'I have been reading for 3 hours'.",
      created_at: "2026-11-25 08:45:00"
    },

    // ==========================================
    // BUỔI 2: HỆ THỐNG 4 THÌ QUÁ KHỨ (PAST TENSES)
    // ==========================================
    {
      id: 5,
      session_id: 2,
      title: "1. Thì Quá khứ đơn (Past Simple Tense)",
      structure: "(+) S + V2/ed | (-) S + did not (didn't) + V_inf | (?) Did + S + V_inf?\nVới To-be: (+) S + was/were | (-) S + was/were not | (?) Was/Were + S...?",
      explanation: "Dùng để diễn tả:\n1. Hành động đã bắt đầu và kết thúc trọn vẹn trong quá khứ, có mốc thời gian xác định rõ ràng.\n2. Chuỗi các hành động xảy ra kế tiếp nhau trong quá khứ (He locked the door, turned off the lights, and left).\n3. Dấu hiệu nhận biết: yesterday, last week/month/year, in 2020, ago, when I was a child...",
      examples: "+ Our engineering team launched the production application three weeks ago.\n- She didn't participate in the symposium yesterday due to a prior commitment.\n? Did you receive the feedback document from Professor Miller?",
      tips: "Khi đã mượn trợ động từ 'DID' trong câu phủ định hoặc nghi vấn, động từ chính BẮT BUỘC trở về dạng NGUYÊN THỂ (V_inf).",
      created_at: "2026-11-25 09:30:00"
    },
    {
      id: 6,
      session_id: 2,
      title: "2. Thì Quá khứ tiếp diễn (Past Continuous Tense)",
      structure: "(+) S + was/were + V-ing | (-) S + wasn't/weren't + V-ing | (?) Was/Were + S + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Hành động đang xảy ra tại một thời điểm chính xác trong quá khứ (at 8 PM yesterday, at this time last week).\n2. Hai hay nhiều hành động diễn ra đồng thời song song trong quá khứ (kết hợp với 'While').\n3. Một hành động dài đang diễn ra trong quá khứ thì một hành động ngắn bất chợt xen vào.",
      examples: "+ At 9 PM last night, I was reviewing my vocabulary flashcards for the final exam.\n+ While the teacher was explaining the grammar rules, the students were taking notes.\n? What were you doing when the emergency power outage occurred?",
      tips: "Công thức phối thì kinh điển:\n- When + Quá khứ đơn (hành động ngắn cắt ngang), Quá khứ tiếp diễn (hành động dài đang xảy ra).\n- While + Quá khứ tiếp diễn, Quá khứ đơn.",
      created_at: "2026-11-25 09:35:00"
    },
    {
      id: 7,
      session_id: 2,
      title: "3. Thì Quá khứ hoàn thành (Past Perfect Tense)",
      structure: "(+) S + had + V3/ed | (-) S + had not (hadn't) + V3/ed | (?) Had + S + V3/ed?",
      explanation: "Dùng để diễn tả:\n1. Hành động đã xảy ra và hoàn tất TRƯỚC một hành động khác trong quá khứ (Hành động trước: Past Perfect; Hành động sau: Past Simple).\n2. Hành động xảy ra trước một mốc thời gian xác định trong quá khứ.\n3. Dấu hiệu nhận biết: by the time, before, after, by 2015, until then...",
      examples: "+ By the time the rescue team arrived at the summit, the storm had already passed.\n+ After she had completed her master's thesis, she travelled around Europe.\n? Had you ever studied computer science before you joined this university?",
      tips: "Công thức vàng dễ thuộc:\n- By the time / Before + Quá khứ đơn, Quá khứ hoàn thành.\n- After + Quá khứ hoàn thành, Quá khứ đơn.",
      created_at: "2026-11-25 09:40:00"
    },
    {
      id: 8,
      session_id: 2,
      title: "4. Thì Quá khứ hoàn thành tiếp diễn (Past Perfect Continuous Tense)",
      structure: "(+) S + had been + V-ing | (-) S + hadn't been + V-ing | (?) Had + S + been + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Nhấn mạnh TÍNH LIÊN TỤC và KÉO DÀI của một hành động xảy ra trước một thời điểm hoặc một hành động khác trong quá khứ.\n2. Chỉ nguyên nhân trực tiếp cho một trạng thái/kết quả trong quá khứ (He was exhausted because he had been driving for 8 hours).",
      examples: "+ When the train finally arrived, passengers had been waiting on the platform for two hours.\n+ Her eyes were strained because she had been staring at the monitor all morning.\n? How long had they been negotiating before they officially signed the contract?",
      tips: "Khi câu muốn nhấn mạnh 'khoảng thời gian bao lâu' (for hours, for 5 years) trước một mốc quá khứ, hãy ưu tiên dùng Quá khứ hoàn thành tiếp diễn.",
      created_at: "2026-11-25 09:45:00"
    },

    // ==========================================
    // BUỔI 3: HỆ THỐNG 4 THÌ TƯƠNG LAI (FUTURE TENSES)
    // ==========================================
    {
      id: 9,
      session_id: 3,
      title: "1. Thì Tương lai đơn (Future Simple) & Be Going To",
      structure: "Will: (+) S + will + V_inf | (-) S + won't + V_inf | (?) Will + S + V_inf?\nBe going to: (+) S + am/is/are going to + V_inf | (-) S + am/is/are not going to + V_inf",
      explanation: "Phân biệt bản chất 2 cấu trúc tương lai phổ biến nhất:\n1. Will: Quyết định bộc phát ngay tại thời điểm nói (không có dự định trước), lời hứa, đề nghị giúp đỡ, dự đoán chủ quan cảm tính.\n2. Be going to: Dự định, kế hoạch đã được tính toán từ trước khi nói, hoặc dự đoán dựa trên BẰNG CHỨNG RÕ RÀNG trước mắt (Look at the dark clouds!).",
      examples: "+ Look at those black clouds! It is going to rain very heavily.\n+ Don't worry about the heavy suitcases, I will give you a hand.\n- We are not going to deploy the update until all unit tests pass completely.",
      tips: "LUẬT BẤT THÀNH VĂN: Trong mệnh đề trạng ngữ chỉ thời gian (bắt đầu bằng When, As soon as, Before, After, Until) ở tương lai, động từ PHẢI chia ở Hiện tại đơn, KHÔNG dùng 'will'.",
      created_at: "2026-11-25 10:30:00"
    },
    {
      id: 10,
      session_id: 3,
      title: "2. Thì Tương lai tiếp diễn (Future Continuous Tense)",
      structure: "(+) S + will be + V-ing | (-) S + won't be + V-ing | (?) Will + S + be + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Hành động đang diễn ra tại một thời điểm cụ thể xác định trong tương lai (at 9 AM tomorrow, this time next week).\n2. Hành động đang xảy ra trong tương lai thì có một hành động khác xen vào (mệnh đề xen vào chia Hiện tại đơn).\n3. Hành động diễn ra như một phần trong lịch trình thông thường không thể tránh khỏi.",
      examples: "+ This time next Monday, I will be flying across the Pacific to San Francisco.\n+ When you arrive at the station tomorrow, our representative will be waiting for you.\n? Will you be using the laboratory computer tomorrow morning?",
      tips: "Dấu hiệu nhận biết đặc trưng: 'At this time tomorrow', 'At 8 o'clock tonight', 'At this moment next year'...",
      created_at: "2026-11-25 10:35:00"
    },
    {
      id: 11,
      session_id: 3,
      title: "3. Thì Tương lai hoàn thành (Future Perfect Tense)",
      structure: "(+) S + will have + V3/ed | (-) S + won't have + V3/ed | (?) Will + S + have + V3/ed?",
      explanation: "Dùng để diễn tả:\n1. Hành động sẽ được hoàn tất TRƯỚC một mốc thời gian hoặc trước một hành động khác trong tương lai.\n2. Dấu hiệu nhận biết: By + mốc tương lai (By next month, By 2030), By the time + mệnh đề hiện tại đơn, Before + mốc tương lai.",
      examples: "+ By the end of this semester, we will have completed all core coursework.\n+ By the time you get home tonight, dinner will have been prepared.\n? Will you have submitted your final application before the deadline expires?",
      tips: "Công thức vàng trong bài thi TOEIC & IELTS:\n'By the time + S + V(hiện tại đơn), S + will have + V3/ed'.",
      created_at: "2026-11-25 10:40:00"
    },
    {
      id: 12,
      session_id: 3,
      title: "4. Thì Tương lai hoàn thành tiếp diễn (Future Perfect Continuous Tense)",
      structure: "(+) S + will have been + V-ing | (-) S + won't have been + V-ing | (?) Will + S + have been + V-ing?",
      explanation: "Dùng để diễn tả:\n1. Nhấn mạnh khoảng thời gian kéo dài liên tục của một hành động tính tới một mốc thời điểm xác định trong tương lai.\n2. Dấu hiệu nhận biết: By the end of this year, for 10 years; By next month, for 3 years...",
      examples: "+ By next October, Dr. Watson will have been lecturing at this institute for twenty years.\n+ By 8 PM tonight, they will have been coding continuously for twelve hours.\n? How long will you have been residing in this apartment by next summer?",
      tips: "Thì này là 'vũ khí bí mật' giúp nâng cao band điểm Grammatical Range and Accuracy trong bài thi IELTS Writing Task 2 khi cần nhấn mạnh khoảng thời gian kéo dài liên tục.",
      created_at: "2026-11-25 10:45:00"
    },

    // ==========================================
    // BUỔI 4: MỆNH ĐỀ QUAN HỆ & RÚT GỌN (RELATIVE CLAUSES)
    // ==========================================
    {
      id: 13,
      session_id: 4,
      title: "1. Đại từ & Trạng từ quan hệ (Who, Whom, Which, That, Whose...)",
      structure: "Người: WHO + V / WHOM + S + V | Vật: WHICH/THAT + V/S+V | Sở hữu: N + WHOSE + N | Trạng từ: WHERE, WHEN, WHY",
      explanation: "Dùng để nối 2 câu đơn thành câu phức gãy gọn. Phân biệt:\n- Mệnh đề xác định (Defining): Không có dấu phẩy, bắt buộc phải có để câu rõ nghĩa, dùng được 'that'.\n- Mệnh đề không xác định (Non-defining): Có dấu phẩy phân tách, chỉ bổ sung thông tin cho danh từ đã rõ ràng (tên riêng, có this/that/my...).",
      examples: "+ The engineer who developed this application received the award.\n+ Dr. Harrison, whom we interviewed yesterday, has accepted the position.\n+ The company whose headquarters is in Tokyo announced a new strategy.",
      tips: "LUẬT VÀNG: Sau dấu phẩy và sau giới từ (in, on, with...) TUYỆT ĐỐI KHÔNG BAO GIỜ dùng 'THAT'.",
      created_at: "2026-11-25 11:30:00"
    },
    {
      id: 14,
      session_id: 4,
      title: "2. Rút gọn mệnh đề quan hệ đỉnh cao (Reduced Relative Clauses)",
      structure: "1. Chủ động ➡️ V-ing | 2. Bị động ➡️ V3/ed | 3. Sau the first, last, only, so sánh nhất ➡️ To V",
      explanation: "Biến đổi giúp câu văn ngắn gọn, học thuật và đạt điểm cao trong bài thi tiếng Anh:\n- Chủ động: The girl who is talking to Nam ➡️ The girl talking to Nam.\n- Bị động: The laptops which were manufactured in 2025 ➡️ The laptops manufactured in 2025.\n- Cụm thứ tự: Yuri Gagarin was the first person who flew into space ➡️ the first person to fly into space.",
      examples: "+ Active: Documents submitted after 5 PM will not be processed.\n+ Passive: The keynote address delivered by the CEO inspired everyone.\n+ To-V: She is the only candidate to achieve a perfect band score.",
      tips: "Luôn xác định xem danh từ phía trước là CHỦ THỂ (hành động ➡️ V-ing) hay ĐỐI TƯỢNG (bị tác động ➡️ V3/ed).",
      created_at: "2026-11-25 11:35:00"
    },

    // ==========================================
    // BUỔI 5: QUY TẮC PHÁT ÂM ĐUÔI -ED
    // ==========================================
    {
      id: 15,
      session_id: 5,
      title: "1. Ba quy tắc vàng phát âm đuôi -ED (/id/, /t/, /d/)",
      structure: "1. Đọc là /id/: sau âm /t/ và /d/\n2. Đọc là /t/: sau âm vô thanh (/p, k, f, s, ʃ, tʃ/)\n3. Đọc là /d/: các âm hữu thanh còn lại",
      explanation: "- Nhóm 1 (/id/): wanted, needed, decided, started, invited, landed.\n- Nhóm 2 (/t/): Âm kết thúc là các âm vô thanh (không rung thanh quản, mẹo: 'Chính Phủ Phát Sách Không Thèm / Thảo'): stopped, looked, laughed, washed, watched, kissed, fixed.\n- Nhóm 3 (/d/): Các âm hữu thanh (rung cổ họng + nguyên âm): played, opened, lived, cleaned, called, enjoyed, preferred.",
      examples: "+ /id/: They decided to postpone the tournament until next week.\n+ /t/: He walked to the office, checked his email, and fixed the bug.\n+ /d/: She called her parents and explained the whole situation.",
      tips: "Nhìn vào ÂM CUỐI CÙNG được phát âm, KHÔNG nhìn vào chữ cái viết (ví dụ: 'laughed' tận cùng chữ gh nhưng phát âm là /f/ ➡️ đọc là /t/).",
      created_at: "2026-11-25 12:30:00"
    },
    {
      id: 16,
      session_id: 5,
      title: "2. Danh sách tính từ ngoại lệ bắt buộc phát âm là /id/",
      structure: "Danh sách từ đặc biệt: naked, wicked, learned, ragged, sacred, blessed, crooked...",
      explanation: "Khi các từ này đóng vai trò là TÍNH TỪ (Adjectives), đuôi -ed luôn phát âm là /id/ thay vì /t/ hay /d/ như quy tắc thông thường.",
      examples: "+ naked /ˈneɪkɪd/ (trần trụi, khỏa thân)\n+ wicked /ˈwɪkɪd/ (độc ác, xấu xa)\n+ learned /ˈlɜːnɪd/ (uyên bác, có học thức - e.g. a learned professor)\n+ ragged /ˈræɡɪd/ (rách rưới, tả tơi)\n+ sacred /ˈseɪkrɪd/ (thiêng liêng, trang trọng)\n+ blessed /ˈblesɪd/ (may mắn, thần thánh)\n+ crooked /ˈkrʊkɪd/ (quanh co, cong queo)",
      tips: "Trong bài thi trắc nghiệm phát âm THPTQG và TOEIC, các từ ngoại lệ này xuất hiện với tần suất trên 80%!",
      created_at: "2026-11-25 12:35:00"
    },

    // ==========================================
    // BUỔI 6: NỐI ÂM & NGỮ ĐIỆU (CONNECTED SPEECH)
    // ==========================================
    {
      id: 17,
      session_id: 6,
      title: "1. Nối âm Phụ âm - Nguyên âm & Nguyên âm - Nguyên âm",
      structure: "Phụ âm + Nguyên âm: C + V ➡️ Nối dính | Nguyên âm + Nguyên âm: V + V ➡️ Chèn âm /j/ hoặc /w/",
      explanation: "1. Phụ âm nối nguyên âm (C+V): Âm cuối từ trước nối thẳng sang nguyên âm từ sau:\n   - 'Hold on' ➡️ /həʊl-dɒn/\n   - 'Turn off' ➡️ /tɜːn-nɒf/\n   - 'Wake up' ➡️ /weɪ-kʌp/\n   - 'An apple' ➡️ /ən-næpl/\n2. Nguyên âm nối nguyên âm (V+V):\n   - Kết thúc miệng dẹt (/i:, ɪ, eɪ, aɪ, ɔɪ/) ➡️ Chèn âm /j/ (y): 'I agree' ➡️ /aɪ-j-əˈɡriː/, 'See it' ➡️ /siː-j-ɪt/.\n   - Kết thúc miệng tròn (/u:, əʊ, aʊ/) ➡️ Chèn âm /w/: 'Go out' ➡️ /ɡəʊ-w-aʊt/, 'You are' ➡️ /juː-w-ɑːr/.",
      examples: "+ C+V: Can I have a cup of water? ➡️ /kæn-aɪ hæv-ə kʌp-əv wɔːtər/\n+ V+V (/j/): She asked me to say it again. ➡️ /seɪ-j-ɪt/\n+ V+V (/w/): Let's do it right away. ➡️ /duː-w-ɪt/",
      tips: "Khi nói tiếng Anh, đừng cố phát âm rời rạc như gõ máy; hãy để luồng hơi nối trơn tru giữa các âm tiết.",
      created_at: "2026-11-25 13:30:00"
    },
    {
      id: 18,
      session_id: 6,
      title: "2. Biến âm (Assimilation) & Nuốt âm (Elision) trong giao tiếp",
      structure: "/t/ + /j/ ➡️ /tʃ/ | /d/ + /j/ ➡️ /dʒ/ | Nuốt /t/, /d/ kẹt giữa 2 phụ âm",
      explanation: "1. Biến âm (Assimilation):\n   - /t/ gặp /j/: 'Nice to meet you' ➡️ /miːtʃu/, 'Don't you' ➡️ /dəʊntʃu/.\n   - /d/ gặp /j/: 'Did you' ➡️ /dɪdʒu/, 'Could you' ➡️ /kʊdʒu/.\n2. Nuốt âm (Elision): Bỏ hẳn âm /t/ hoặc /d/ khi đứng giữa hai phụ âm khác để nói nhanh hơn:\n   - 'Last night' ➡️ /lɑːs naɪt/ (bỏ /t/)\n   - 'Next door' ➡️ /neks dɔːr/ (bỏ /t/)\n   - 'Post card' ➡️ /pəʊs kɑːd/",
      examples: "+ Could you tell me what you need? ➡️ /kʊdʒu tel miː wʌtʃu niːd/\n+ I saw him last night next door. ➡️ /lɑːs naɪt neks dɔːr/\n+ Don't you want to come? ➡️ /dəʊntʃu wɒntə kʌm/",
      tips: "Nắm được quy tắc này bạn sẽ nghe hiểu người bản xứ khi họ nói nhanh (Fast Native Speech) mà không bị bỡ ngỡ.",
      created_at: "2026-11-25 13:35:00"
    }
  ]
};

// LocalStorage Controller with auto-upgrade
const CURRENT_CURRICULUM_VERSION = 'v4_all_12_tenses';

const localDb = {
  get(key) {
    const ver = localStorage.getItem('enotes_curriculum_version');
    if (ver !== CURRENT_CURRICULUM_VERSION) {
      // Auto-upgrade local storage to full curriculum
      localStorage.setItem('enotes_sessions', JSON.stringify(SEED_DATA.sessions));
      localStorage.setItem('enotes_vocabularies', JSON.stringify(SEED_DATA.vocabularies));
      localStorage.setItem('enotes_grammars', JSON.stringify(SEED_DATA.grammars));
      localStorage.setItem('enotes_curriculum_version', CURRENT_CURRICULUM_VERSION);
      return JSON.parse(JSON.stringify(SEED_DATA[key]));
    }

    const raw = localStorage.getItem('enotes_' + key);
    if (!raw) {
      if (SEED_DATA[key]) {
        localStorage.setItem('enotes_' + key, JSON.stringify(SEED_DATA[key]));
        return JSON.parse(JSON.stringify(SEED_DATA[key]));
      }
      return [];
    }
    try {
      return JSON.parse(raw);
    } catch (e) {
      return [];
    }
  },
  set(key, val) {
    localStorage.setItem('enotes_' + key, JSON.stringify(val));
  }
};

// LocalStorage API Emulator
function handleLocalApi(path, options = {}) {
  const method = (options.method || 'GET').toUpperCase();
  const body = options.body ? JSON.parse(options.body) : {};
  const now = new Date().toISOString().replace('T', ' ').substring(0, 19);

  // Stats
  if (path === '/api/stats') {
    const s = localDb.get('sessions');
    const v = localDb.get('vocabularies');
    const g = localDb.get('grammars');
    const mastered = v.filter(item => item.status === 'mastered').length;
    return {
      total_sessions: s.length,
      total_vocab: v.length,
      total_grammar: g.length,
      mastered_vocab: mastered,
      learning_vocab: v.length - mastered,
    };
  }

  // Sessions List / Create
  if (path === '/api/sessions') {
    const list = localDb.get('sessions');
    if (method === 'GET') {
      return list.sort((a, b) => b.session_number - a.session_number);
    }
    if (method === 'POST') {
      const nextId = list.length > 0 ? Math.max(...list.map(i => i.id)) + 1 : 1;
      const nextNum = body.session_number || (list.length > 0 ? Math.max(...list.map(i => i.session_number)) + 1 : 1);
      const newSession = {
        id: nextId,
        session_number: nextNum,
        title: body.title,
        date: body.date || new Date().toISOString().split('T')[0],
        teacher: body.teacher || null,
        goals: body.goals || null,
        notes: body.notes || null,
        created_at: now,
        updated_at: now,
      };
      list.push(newSession);
      localDb.set('sessions', list);
      return newSession;
    }
  }

  // Session Detail / Update / Delete
  const sessionMatch = path.match(/^\/api\/sessions\/(\d+)$/);
  if (sessionMatch) {
    const id = parseInt(sessionMatch[1]);
    const list = localDb.get('sessions');
    const idx = list.findIndex(i => i.id === id);

    if (method === 'GET') {
      if (idx === -1) throw new Error('Session not found');
      const vocab = localDb.get('vocabularies').filter(v => v.session_id === id);
      const grammar = localDb.get('grammars').filter(g => g.session_id === id);
      return {
        session: list[idx],
        vocabularies: vocab,
        grammars: grammar,
      };
    }
    if (method === 'PUT') {
      if (idx === -1) throw new Error('Session not found');
      list[idx] = { ...list[idx], ...body, updated_at: now };
      localDb.set('sessions', list);
      return list[idx];
    }
    if (method === 'DELETE') {
      if (idx !== -1) {
        list.splice(idx, 1);
        localDb.set('sessions', list);
        // Cascade delete vocab & grammar
        const vocab = localDb.get('vocabularies').filter(v => v.session_id !== id);
        localDb.set('vocabularies', vocab);
        const grammar = localDb.get('grammars').filter(g => g.session_id !== id);
        localDb.set('grammars', grammar);
      }
      return { deleted: true, id };
    }
  }

  // Vocabularies Create
  if (path === '/api/vocabularies' && method === 'POST') {
    const list = localDb.get('vocabularies');
    const nextId = list.length > 0 ? Math.max(...list.map(i => i.id)) + 1 : 1;
    const newVocab = {
      id: nextId,
      session_id: body.session_id,
      word: body.word,
      ipa: body.ipa || null,
      pos: body.pos || null,
      meaning: body.meaning,
      example_en: body.example_en || null,
      example_vi: body.example_vi || null,
      status: body.status || 'learning',
      notes: body.notes || null,
      created_at: now,
    };
    list.push(newVocab);
    localDb.set('vocabularies', list);
    return newVocab;
  }

  // Vocabularies Update / Delete
  const vocabMatch = path.match(/^\/api\/vocabularies\/(\d+)$/);
  if (vocabMatch) {
    const id = parseInt(vocabMatch[1]);
    const list = localDb.get('vocabularies');
    const idx = list.findIndex(i => i.id === id);

    if (method === 'PUT') {
      if (idx === -1) throw new Error('Vocabulary not found');
      list[idx] = { ...list[idx], ...body };
      localDb.set('vocabularies', list);
      return list[idx];
    }
    if (method === 'DELETE') {
      if (idx !== -1) {
        list.splice(idx, 1);
        localDb.set('vocabularies', list);
      }
      return { deleted: true, id };
    }
  }

  // Grammars Create
  if (path === '/api/grammars' && method === 'POST') {
    const list = localDb.get('grammars');
    const nextId = list.length > 0 ? Math.max(...list.map(i => i.id)) + 1 : 1;
    const newGrammar = {
      id: nextId,
      session_id: body.session_id,
      title: body.title,
      structure: body.structure,
      explanation: body.explanation,
      examples: body.examples,
      tips: body.tips || null,
      created_at: now,
    };
    list.push(newGrammar);
    localDb.set('grammars', list);
    return newGrammar;
  }

  // Grammars Update / Delete
  const grammarMatch = path.match(/^\/api\/grammars\/(\d+)$/);
  if (grammarMatch) {
    const id = parseInt(grammarMatch[1]);
    const list = localDb.get('grammars');
    const idx = list.findIndex(i => i.id === id);

    if (method === 'PUT') {
      if (idx === -1) throw new Error('Grammar not found');
      list[idx] = { ...list[idx], ...body };
      localDb.set('grammars', list);
      return list[idx];
    }
    if (method === 'DELETE') {
      if (idx !== -1) {
        list.splice(idx, 1);
        localDb.set('grammars', list);
      }
      return { deleted: true, id };
    }
  }

  // Search
  if (path.startsWith('/api/search')) {
    const query = new URLSearchParams(path.split('?')[1] || '').get('q') || '';
    const q = query.toLowerCase();
    const sessions = localDb.get('sessions').filter(s =>
      (s.title && s.title.toLowerCase().includes(q)) ||
      (s.teacher && s.teacher.toLowerCase().includes(q)) ||
      (s.goals && s.goals.toLowerCase().includes(q)) ||
      (s.notes && s.notes.toLowerCase().includes(q))
    );
    const vocabularies = localDb.get('vocabularies').filter(v =>
      (v.word && v.word.toLowerCase().includes(q)) ||
      (v.meaning && v.meaning.toLowerCase().includes(q)) ||
      (v.example_en && v.example_en.toLowerCase().includes(q)) ||
      (v.example_vi && v.example_vi.toLowerCase().includes(q))
    );
    const grammars = localDb.get('grammars').filter(g =>
      (g.title && g.title.toLowerCase().includes(q)) ||
      (g.structure && g.structure.toLowerCase().includes(q)) ||
      (g.explanation && g.explanation.toLowerCase().includes(q)) ||
      (g.examples && g.examples.toLowerCase().includes(q))
    );
    return { sessions, vocabularies, grammars };
  }

  // Backup Import
  if (path === '/api/backup/import' && method === 'POST') {
    if (body.sessions && body.vocabularies && body.grammars) {
      localDb.set('sessions', body.sessions);
      localDb.set('vocabularies', body.vocabularies);
      localDb.set('grammars', body.grammars);
      return { success: true };
    }
    throw new Error('Dữ liệu sao lưu không đúng định dạng');
  }

  throw new Error(`Endpoint not supported in client mode: ${path}`);
}

// DOM Elements
const el = {
  // Sidebar & Stats
  searchInput: document.getElementById('searchInput'),
  sessionList: document.getElementById('sessionList'),
  sessionListCount: document.getElementById('sessionListCount'),
  btnNewSession: document.getElementById('btnNewSession'),
  btnWelcomeNewSession: document.getElementById('btnWelcomeNewSession'),
  statSessions: document.getElementById('statSessions'),
  statVocab: document.getElementById('statVocab'),
  statGrammar: document.getElementById('statGrammar'),
  statMastered: document.getElementById('statMastered'),
  btnExport: document.getElementById('btnExport'),
  btnImport: document.getElementById('btnImport'),
  importFileInput: document.getElementById('importFileInput'),

  // Views
  sessionView: document.getElementById('sessionView'),
  searchView: document.getElementById('searchView'),
  emptyWelcomeView: document.getElementById('emptyWelcomeView'),
  searchResultsList: document.getElementById('searchResultsList'),
  searchQueryLabel: document.getElementById('searchQueryLabel'),

  // Header
  viewSessionBadge: document.getElementById('viewSessionBadge'),
  viewSessionTitle: document.getElementById('viewSessionTitle'),
  viewSessionDate: document.getElementById('viewSessionDate'),
  viewSessionTeacher: document.getElementById('viewSessionTeacher'),
  viewSessionGoalsBox: document.getElementById('viewSessionGoalsBox'),
  viewSessionGoals: document.getElementById('viewSessionGoals'),
  btnEditSession: document.getElementById('btnEditSession'),
  btnDeleteSession: document.getElementById('btnDeleteSession'),

  // Tabs
  tabBtns: document.querySelectorAll('.tab-btn'),
  tabContentVocab: document.getElementById('tabContentVocab'),
  tabContentGrammar: document.getElementById('tabContentGrammar'),
  tabContentNotes: document.getElementById('tabContentNotes'),
  tabContentPractice: document.getElementById('tabContentPractice'),
  tabVocabCount: document.getElementById('tabVocabCount'),
  tabGrammarCount: document.getElementById('tabGrammarCount'),
  countVocabBadge: document.getElementById('countVocabBadge'),
  countGrammarBadge: document.getElementById('countGrammarBadge'),

  // Lists
  vocabList: document.getElementById('vocabList'),
  grammarList: document.getElementById('grammarList'),
  sessionNotesText: document.getElementById('sessionNotesText'),
  btnSaveNotes: document.getElementById('btnSaveNotes'),
  flashcardWrapper: document.getElementById('flashcardWrapper'),

  // Buttons for adding items
  btnAddVocab: document.getElementById('btnAddVocab'),
  btnAddGrammar: document.getElementById('btnAddGrammar'),

  // Modals
  modalSession: document.getElementById('modalSession'),
  modalVocab: document.getElementById('modalVocab'),
  modalGrammar: document.getElementById('modalGrammar'),

  // Forms
  formSession: document.getElementById('formSession'),
  btnSubmitSession: document.getElementById('btnSubmitSession'),
  formVocab: document.getElementById('formVocab'),
  btnSubmitVocab: document.getElementById('btnSubmitVocab'),
  formGrammar: document.getElementById('formGrammar'),
  btnSubmitGrammar: document.getElementById('btnSubmitGrammar'),
};

// API Helpers with automatic fallback
async function api(path, options = {}) {
  if (useLocalStore) {
    return handleLocalApi(path, options);
  }

  try {
    const res = await fetch(path, {
      headers: {
        'Content-Type': 'application/json',
        ...(options.headers || {}),
      },
      ...options,
    });
    if (!res.ok) {
      // If serverless 404 / 500 on Vercel, switch to localStore automatically
      if (res.status === 404 || res.status === 405 || res.status >= 500) {
        console.warn('Backend unavailable, switching to local storage mode');
        useLocalStore = true;
        return handleLocalApi(path, options);
      }
      const err = await res.text();
      throw new Error(err || `Lỗi HTTP ${res.status}`);
    }
    return await res.json();
  } catch (e) {
    console.warn('Network error, activating client storage mode:', e);
    useLocalStore = true;
    return handleLocalApi(path, options);
  }
}

// Audio Pronunciation using native Web Speech API
function speakWord(word) {
  if (!('speechSynthesis' in window)) {
    alert('Trình duyệt không hỗ trợ phát âm trực tiếp.');
    return;
  }
  window.speechSynthesis.cancel();
  const utter = new SpeechSynthesisUtterance(word);
  utter.lang = 'en-US';
  utter.rate = 0.9;
  window.speechSynthesis.speak(utter);
}

// Load App Stats
async function loadStats() {
  try {
    const stats = await api('/api/stats');
    el.statSessions.textContent = stats.total_sessions;
    el.statVocab.textContent = stats.total_vocab;
    el.statGrammar.textContent = stats.total_grammar;
    const pct = stats.total_vocab > 0 
      ? Math.round((stats.mastered_vocab / stats.total_vocab) * 100) 
      : 0;
    el.statMastered.textContent = `${pct}%`;
  } catch (e) {
    console.error('Không thể tải thống kê', e);
  }
}

// Load Session List
async function loadSessions(selectId = null) {
  try {
    const sessions = await api('/api/sessions');
    state.sessions = sessions;
    el.sessionListCount.textContent = sessions.length;

    renderSessionList();
    await loadStats();

    if (sessions.length > 0) {
      const targetId = selectId || state.currentSessionId || sessions[0].id;
      const found = sessions.find(s => s.id === targetId);
      if (found) {
        selectSession(targetId);
      } else {
        selectSession(sessions[0].id);
      }
    } else {
      showEmptyState();
    }
  } catch (e) {
    alert('Lỗi tải danh sách buổi học: ' + e.message);
  }
}

function renderSessionList() {
  el.sessionList.innerHTML = '';
  state.sessions.forEach(s => {
    const li = document.createElement('li');
    li.className = `session-item ${s.id === state.currentSessionId ? 'active' : ''}`;
    li.onclick = () => selectSession(s.id);

    li.innerHTML = `
      <div class="session-item-top">
        <span class="session-badge">#${s.session_number}</span>
        <span class="session-date">${s.date}</span>
      </div>
      <div class="session-item-title" title="${escapeHtml(s.title)}">${escapeHtml(s.title)}</div>
      <div class="session-item-meta">
        <span>${escapeHtml(s.teacher || 'Tự học')}</span>
      </div>
    `;
    el.sessionList.appendChild(li);
  });
}

// Select a Session
async function selectSession(sessionId) {
  state.currentSessionId = sessionId;
  renderSessionList();

  try {
    const detail = await api(`/api/sessions/${sessionId}`);
    state.currentDetail = detail;
    renderSessionDetail(detail);

    el.emptyWelcomeView.style.display = 'none';
    el.searchView.style.display = 'none';
    el.sessionView.style.display = 'block';
  } catch (e) {
    alert('Không thể tải chi tiết buổi học: ' + e.message);
  }
}

function renderSessionDetail(detail) {
  const { session, vocabularies, grammars } = detail;

  el.viewSessionBadge.textContent = `BUỔI #${session.session_number}`;
  el.viewSessionTitle.textContent = session.title;
  el.viewSessionDate.textContent = session.date;
  el.viewSessionTeacher.textContent = session.teacher || 'Chưa cập nhật';

  if (session.goals && session.goals.trim()) {
    el.viewSessionGoals.textContent = session.goals;
    el.viewSessionGoalsBox.style.display = 'block';
  } else {
    el.viewSessionGoalsBox.style.display = 'none';
  }

  el.sessionNotesText.value = session.notes || '';

  // Tab counts
  el.tabVocabCount.textContent = vocabularies.length;
  el.tabGrammarCount.textContent = grammars.length;
  el.countVocabBadge.textContent = `${vocabularies.length} từ`;
  el.countGrammarBadge.textContent = `${grammars.length} điểm`;

  renderVocabList(vocabularies);
  renderGrammarList(grammars);
  renderFlashcardView(vocabularies);
}

// Render Vocab List
function renderVocabList(vocabularies) {
  el.vocabList.innerHTML = '';
  if (vocabularies.length === 0) {
    el.vocabList.innerHTML = `
      <div class="empty-state" style="padding: 32px 0;">
        <p>Chưa có từ vựng nào trong buổi học này.</p>
        <button class="btn btn-secondary btn-sm" onclick="openVocabModal()">+ Thêm từ vựng đầu tiên</button>
      </div>
    `;
    return;
  }

  vocabularies.forEach(v => {
    const card = document.createElement('div');
    card.className = 'vocab-card';

    const statusLabels = {
      learning: 'Đang học',
      reviewing: 'Cần ôn tập',
      mastered: 'Đã thuộc',
    };

    card.innerHTML = `
      <div class="vocab-header">
        <div class="vocab-word-group">
          <span class="vocab-word">${escapeHtml(v.word)}</span>
          ${v.ipa ? `<span class="vocab-ipa">${escapeHtml(v.ipa)}</span>` : ''}
          ${v.pos ? `<span class="vocab-pos">${escapeHtml(v.pos)}</span>` : ''}
          <button class="audio-btn" title="Nghe phát âm" onclick="speakWord('${escapeJsString(v.word)}')">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
              <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
              <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
            </svg>
          </button>
        </div>
        <div style="display: flex; gap: 8px; align-items: center;">
          <span class="status-pill status-${v.status}" onclick="cycleVocabStatus(${v.id}, '${v.status}')" title="Nhấp để chuyển trạng thái">
            ${statusLabels[v.status] || v.status}
          </span>
          <button class="btn btn-secondary btn-sm teacher-only" onclick="openVocabModal(${v.id})">Sửa</button>
          <button class="btn btn-secondary btn-sm btn-danger-ghost teacher-only" onclick="deleteVocab(${v.id})">Xóa</button>
        </div>
      </div>
      <div class="vocab-meaning">${escapeHtml(v.meaning)}</div>
      ${v.example_en ? `
        <div class="vocab-example-box">
          <div class="example-en"><strong>Ví dụ: </strong>${escapeHtml(v.example_en)}</div>
          ${v.example_vi ? `<div class="example-vi">${escapeHtml(v.example_vi)}</div>` : ''}
        </div>
      ` : ''}
      ${v.notes ? `
        <div class="vocab-footer">
          <span class="vocab-notes">Lưu ý: ${escapeHtml(v.notes)}</span>
        </div>
      ` : ''}
    `;
    el.vocabList.appendChild(card);
  });
}

// Render Grammar List
function renderGrammarList(grammars) {
  el.grammarList.innerHTML = '';
  if (grammars.length === 0) {
    el.grammarList.innerHTML = `
      <div class="empty-state" style="padding: 32px 0;">
        <p>Chưa có điểm ngữ pháp nào trong buổi học này.</p>
        <button class="btn btn-secondary btn-sm" onclick="openGrammarModal()">+ Thêm ngữ pháp</button>
      </div>
    `;
    return;
  }

  grammars.forEach(g => {
    const card = document.createElement('div');
    card.className = 'grammar-card';

    card.innerHTML = `
      <div style="display: flex; justify-content: space-between; align-items: flex-start;">
        <h3 class="grammar-title">${escapeHtml(g.title)}</h3>
        <div style="display: flex; gap: 6px;">
          <button class="btn btn-secondary btn-sm teacher-only" onclick="openGrammarModal(${g.id})">Sửa</button>
          <button class="btn btn-secondary btn-sm btn-danger-ghost teacher-only" onclick="deleteGrammar(${g.id})">Xóa</button>
        </div>
      </div>
      <div class="grammar-structure-box">${escapeHtml(g.structure)}</div>
      <div class="grammar-explanation">${escapeHtml(g.explanation)}</div>
      <div class="grammar-examples-box">${escapeHtml(g.examples)}</div>
      ${g.tips ? `<div class="grammar-tips-box"><strong>Mẹo / Lưu ý:</strong> ${escapeHtml(g.tips)}</div>` : ''}
    `;
    el.grammarList.appendChild(card);
  });
}

// Render Flashcard View
function renderFlashcardView(vocabularies) {
  if (vocabularies.length === 0) {
    el.flashcardWrapper.innerHTML = `
      <div class="empty-state">
        <p>Thêm từ vựng vào buổi học để bắt đầu ôn tập flashcard.</p>
      </div>
    `;
    return;
  }

  if (state.flashcardIdx >= vocabularies.length) {
    state.flashcardIdx = 0;
  }

  const v = vocabularies[state.flashcardIdx];

  el.flashcardWrapper.innerHTML = `
    <div class="flashcard-progress">
      Thẻ ${state.flashcardIdx + 1} / ${vocabularies.length} &bull; Trạng thái: <strong>${v.status}</strong>
    </div>

    <div class="flashcard" id="activeFlashcard" onclick="toggleFlashcard()">
      <div class="fc-word">${escapeHtml(v.word)}</div>
      ${v.ipa ? `<div class="fc-ipa">${escapeHtml(v.ipa)}</div>` : ''}
      <button class="audio-btn" style="margin-bottom: 12px;" onclick="event.stopPropagation(); speakWord('${escapeJsString(v.word)}')">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
        </svg>
      </button>

      <div class="fc-details" style="display: ${state.flashcardFlipped ? 'block' : 'none'};">
        <div class="fc-meaning">${escapeHtml(v.meaning)}</div>
        ${v.example_en ? `<div class="fc-example">"${escapeHtml(v.example_en)}"</div>` : ''}
      </div>

      <div class="card-hint">
        ${state.flashcardFlipped ? 'Nhấp để ẩn nghĩa' : 'Nhấp vào thẻ hoặc nhấn Space để lật mở nghĩa'}
      </div>
    </div>

    <div class="flashcard-controls">
      <button class="btn btn-secondary" onclick="prevFlashcard()">Thẻ trước</button>
      <button class="btn btn-secondary" onclick="markCardStatus('learning')">Chưa thuộc</button>
      <button class="btn btn-primary" onclick="markCardStatus('mastered')">Đã thuộc &#10003;</button>
      <button class="btn btn-secondary" onclick="nextFlashcard()">Thẻ tiếp</button>
    </div>
  `;
}

function toggleFlashcard() {
  state.flashcardFlipped = !state.flashcardFlipped;
  if (state.currentDetail) {
    renderFlashcardView(state.currentDetail.vocabularies);
  }
}

function nextFlashcard() {
  state.flashcardFlipped = false;
  if (state.currentDetail && state.currentDetail.vocabularies.length > 0) {
    state.flashcardIdx = (state.flashcardIdx + 1) % state.currentDetail.vocabularies.length;
    renderFlashcardView(state.currentDetail.vocabularies);
  }
}

function prevFlashcard() {
  state.flashcardFlipped = false;
  if (state.currentDetail && state.currentDetail.vocabularies.length > 0) {
    state.flashcardIdx = (state.flashcardIdx - 1 + state.currentDetail.vocabularies.length) % state.currentDetail.vocabularies.length;
    renderFlashcardView(state.currentDetail.vocabularies);
  }
}

async function markCardStatus(newStatus) {
  if (!state.currentDetail || state.currentDetail.vocabularies.length === 0) return;
  const v = state.currentDetail.vocabularies[state.flashcardIdx];
  try {
    await api(`/api/vocabularies/${v.id}`, {
      method: 'PUT',
      body: JSON.stringify({ status: newStatus }),
    });
    v.status = newStatus;
    broadcastRealtimeUpdate('STATUS_CHANGED');
    await loadStats();
    nextFlashcard();
  } catch (e) {
    alert('Lỗi cập nhật trạng thái: ' + e.message);
  }
}

async function cycleVocabStatus(id, currentStatus) {
  const nextMap = {
    learning: 'reviewing',
    reviewing: 'mastered',
    mastered: 'learning',
  };
  const newStatus = nextMap[currentStatus] || 'learning';

  try {
    await api(`/api/vocabularies/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ status: newStatus }),
    });
    broadcastRealtimeUpdate('STATUS_CHANGED');
    await selectSession(state.currentSessionId);
  } catch (e) {
    alert('Lỗi cập nhật: ' + e.message);
  }
}

function showEmptyState() {
  el.emptyWelcomeView.style.display = 'block';
  el.sessionView.style.display = 'none';
  el.searchView.style.display = 'none';
}

// Tab Switching
el.tabBtns.forEach(btn => {
  btn.addEventListener('click', () => {
    el.tabBtns.forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    const tab = btn.dataset.tab;
    state.activeTab = tab;

    el.tabContentVocab.style.display = tab === 'vocab' ? 'block' : 'none';
    el.tabContentGrammar.style.display = tab === 'grammar' ? 'block' : 'none';
    el.tabContentNotes.style.display = tab === 'notes' ? 'block' : 'none';
    el.tabContentPractice.style.display = tab === 'practice' ? 'block' : 'none';
  });
});

// Modals Handling
function openModal(modal) {
  modal.style.display = 'flex';
  modal.classList.add('active');
}

function closeModal(modal) {
  modal.style.display = 'none';
  modal.classList.remove('active');
}

document.querySelectorAll('[data-close-modal]').forEach(btn => {
  btn.addEventListener('click', () => {
    closeModal(btn.closest('.modal-overlay'));
  });
});

// Session CRUD Modals
function openSessionModal(editSession = null) {
  const today = new Date().toISOString().split('T')[0];
  if (editSession) {
    document.getElementById('modalSessionTitle').textContent = 'Chỉnh sửa buổi học';
    document.getElementById('sessionFormId').value = editSession.id;
    document.getElementById('sessionFormNumber').value = editSession.session_number;
    document.getElementById('sessionFormDate').value = editSession.date;
    document.getElementById('sessionFormTitle').value = editSession.title;
    document.getElementById('sessionFormTeacher').value = editSession.teacher || '';
    document.getElementById('sessionFormGoals').value = editSession.goals || '';
    document.getElementById('sessionFormNotes').value = editSession.notes || '';
  } else {
    document.getElementById('modalSessionTitle').textContent = 'Tạo buổi học mới';
    document.getElementById('sessionFormId').value = '';
    const nextNum = state.sessions.length > 0 
      ? Math.max(...state.sessions.map(s => s.session_number)) + 1 
      : 1;
    document.getElementById('sessionFormNumber').value = nextNum;
    document.getElementById('sessionFormDate').value = today;
    document.getElementById('sessionFormTitle').value = `Buổi ${nextNum}: `;
    document.getElementById('sessionFormTeacher').value = '';
    document.getElementById('sessionFormGoals').value = '';
    document.getElementById('sessionFormNotes').value = '';
  }
  openModal(el.modalSession);
}

el.btnNewSession.onclick = () => openSessionModal();
el.btnWelcomeNewSession.onclick = () => openSessionModal();
el.btnEditSession.onclick = () => {
  if (state.currentDetail) openSessionModal(state.currentDetail.session);
};

el.btnDeleteSession.onclick = async () => {
  if (!state.currentSessionId) return;
  if (confirm('Bạn có chắc chắn muốn xóa buổi học này và toàn bộ từ vựng, ngữ pháp liên quan?')) {
    try {
      await api(`/api/sessions/${state.currentSessionId}`, { method: 'DELETE' });
      broadcastRealtimeUpdate('SESSION_DELETED');
      state.currentSessionId = null;
      await loadSessions();
    } catch (e) {
      alert('Lỗi xóa buổi học: ' + e.message);
    }
  }
};

el.btnSubmitSession.onclick = async (e) => {
  e.preventDefault();
  const id = document.getElementById('sessionFormId').value;
  const num = parseInt(document.getElementById('sessionFormNumber').value) || null;
  const title = document.getElementById('sessionFormTitle').value.trim();
  const date = document.getElementById('sessionFormDate').value;
  const teacher = document.getElementById('sessionFormTeacher').value.trim() || null;
  const goals = document.getElementById('sessionFormGoals').value.trim() || null;
  const notes = document.getElementById('sessionFormNotes').value.trim() || null;

  if (!title) {
    alert('Vui lòng nhập tiêu đề buổi học.');
    return;
  }

  const payload = { session_number: num, title, date, teacher, goals, notes };

  try {
    let saved;
    if (id) {
      saved = await api(`/api/sessions/${id}`, {
        method: 'PUT',
        body: JSON.stringify(payload),
      });
    } else {
      saved = await api('/api/sessions', {
        method: 'POST',
        body: JSON.stringify(payload),
      });
    }
    closeModal(el.modalSession);
    broadcastRealtimeUpdate('SESSION_SAVED', { id: saved.id });
    await loadSessions(saved.id);
  } catch (err) {
    alert('Lỗi lưu buổi học: ' + err.message);
  }
};

// Vocab Modals
function openVocabModal(id = null) {
  const statusEl = document.getElementById('lookupStatus');
  if (statusEl) statusEl.style.display = 'none';

  if (id && state.currentDetail) {
    const v = state.currentDetail.vocabularies.find(item => item.id === id);
    if (!v) return;
    document.getElementById('modalVocabTitle').textContent = 'Chỉnh sửa từ vựng';
    document.getElementById('vocabFormId').value = v.id;
    document.getElementById('vocabFormWord').value = v.word;
    document.getElementById('vocabFormIpa').value = v.ipa || '';
    document.getElementById('vocabFormPos').value = v.pos || '';
    document.getElementById('vocabFormMeaning').value = v.meaning;
    document.getElementById('vocabFormExampleEn').value = v.example_en || '';
    document.getElementById('vocabFormExampleVi').value = v.example_vi || '';
    document.getElementById('vocabFormStatus').value = v.status || 'learning';
    document.getElementById('vocabFormNotes').value = v.notes || '';
  } else {
    document.getElementById('modalVocabTitle').textContent = 'Thêm từ vựng mới';
    document.getElementById('vocabFormId').value = '';
    document.getElementById('vocabFormWord').value = '';
    document.getElementById('vocabFormIpa').value = '';
    document.getElementById('vocabFormPos').value = '';
    document.getElementById('vocabFormMeaning').value = '';
    document.getElementById('vocabFormExampleEn').value = '';
    document.getElementById('vocabFormExampleVi').value = '';
    document.getElementById('vocabFormStatus').value = 'learning';
    document.getElementById('vocabFormNotes').value = '';
  }
  openModal(el.modalVocab);
}

// Quick Dictionary & Translation API Auto-Fill
async function lookupWordApi(word) {
  const statusEl = document.getElementById('lookupStatus');
  if (!word || !word.trim()) {
    alert('Vui lòng nhập từ tiếng Anh cần tra cứu.');
    return;
  }
  const cleanWord = word.trim().toLowerCase();

  if (statusEl) {
    statusEl.style.display = 'block';
    statusEl.style.color = 'var(--text-secondary)';
    statusEl.textContent = '⏳ Đang tra cứu từ điển & dịch nghĩa...';
  }

  try {
    const dictPromise = fetch(`https://api.dictionaryapi.dev/api/v2/entries/en/${encodeURIComponent(cleanWord)}`)
      .then(res => res.ok ? res.json() : null)
      .catch(() => null);

    const transPromise = fetch(`https://api.mymemory.translated.net/get?q=${encodeURIComponent(cleanWord)}&langpair=en|vi`)
      .then(res => res.ok ? res.json() : null)
      .catch(() => null);

    const [dictData, transData] = await Promise.all([dictPromise, transPromise]);

    let found = false;

    if (dictData && Array.isArray(dictData) && dictData.length > 0) {
      const entry = dictData[0];

      // IPA Phonetic
      let ipa = entry.phonetic || '';
      if (!ipa && entry.phonetics && entry.phonetics.length > 0) {
        const pWithText = entry.phonetics.find(p => p.text);
        if (pWithText) ipa = pWithText.text;
      }
      if (ipa) {
        document.getElementById('vocabFormIpa').value = ipa;
        found = true;
      }

      // Part of speech & definitions & examples
      if (entry.meanings && entry.meanings.length > 0) {
        const m = entry.meanings[0];
        if (m.partOfSpeech) {
          document.getElementById('vocabFormPos').value = m.partOfSpeech;
        }
        if (m.definitions && m.definitions.length > 0) {
          const d = m.definitions[0];
          if (d.example) {
            document.getElementById('vocabFormExampleEn').value = d.example;
          }
          if (!document.getElementById('vocabFormMeaning').value && d.definition) {
            document.getElementById('vocabFormMeaning').value = d.definition;
          }
        }
      }
    }

    // Translation to Vietnamese
    if (transData && transData.responseData && transData.responseData.translatedText) {
      let viMeaning = transData.responseData.translatedText.trim().toLowerCase();
      if (viMeaning && !viMeaning.includes('mymemory') && !viMeaning.includes('warning:')) {
        document.getElementById('vocabFormMeaning').value = viMeaning;
        found = true;
      }
    }

    if (statusEl) {
      if (found) {
        statusEl.style.color = '#059669';
        statusEl.textContent = '✓ Đã tự động điền IPA, từ loại, ví dụ & nghĩa tiếng Việt!';
      } else {
        statusEl.style.color = '#D97706';
        statusEl.textContent = 'Không tìm thấy dữ liệu tự động, bạn vui lòng nhập thủ công.';
      }
    }
  } catch (err) {
    if (statusEl) {
      statusEl.style.color = '#DC2626';
      statusEl.textContent = 'Lỗi tra cứu: ' + err.message;
    }
  }
}

document.getElementById('btnQuickLookup')?.addEventListener('click', () => {
  const word = document.getElementById('vocabFormWord').value;
  lookupWordApi(word);
});

document.getElementById('vocabFormWord')?.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    e.preventDefault();
    lookupWordApi(e.target.value);
  }
});

el.btnAddVocab.onclick = () => openVocabModal();

async function deleteVocab(id) {
  if (confirm('Xóa từ vựng này?')) {
    try {
      await api(`/api/vocabularies/${id}`, { method: 'DELETE' });
      broadcastRealtimeUpdate('VOCAB_DELETED');
      await selectSession(state.currentSessionId);
    } catch (e) {
      alert('Lỗi xóa từ vựng: ' + e.message);
    }
  }
}

el.btnSubmitVocab.onclick = async (e) => {
  e.preventDefault();
  const id = document.getElementById('vocabFormId').value;
  const word = document.getElementById('vocabFormWord').value.trim();
  const meaning = document.getElementById('vocabFormMeaning').value.trim();

  if (!word || !meaning) {
    alert('Vui lòng nhập từ và nghĩa tiếng Việt.');
    return;
  }

  const payload = {
    session_id: state.currentSessionId,
    word,
    ipa: document.getElementById('vocabFormIpa').value.trim() || null,
    pos: document.getElementById('vocabFormPos').value.trim() || null,
    meaning,
    example_en: document.getElementById('vocabFormExampleEn').value.trim() || null,
    example_vi: document.getElementById('vocabFormExampleVi').value.trim() || null,
    status: document.getElementById('vocabFormStatus').value,
    notes: document.getElementById('vocabFormNotes').value.trim() || null,
  };

  try {
    if (id) {
      await api(`/api/vocabularies/${id}`, {
        method: 'PUT',
        body: JSON.stringify(payload),
      });
    } else {
      await api('/api/vocabularies', {
        method: 'POST',
        body: JSON.stringify(payload),
      });
    }
    closeModal(el.modalVocab);
    broadcastRealtimeUpdate('VOCAB_SAVED');
    await selectSession(state.currentSessionId);
  } catch (err) {
    alert('Lỗi lưu từ vựng: ' + err.message);
  }
};

// Grammar Modals
function openGrammarModal(id = null) {
  if (id && state.currentDetail) {
    const g = state.currentDetail.grammars.find(item => item.id === id);
    if (!g) return;
    document.getElementById('modalGrammarTitle').textContent = 'Chỉnh sửa ngữ pháp';
    document.getElementById('grammarFormId').value = g.id;
    document.getElementById('grammarFormTitle').value = g.title;
    document.getElementById('grammarFormStructure').value = g.structure;
    document.getElementById('grammarFormExplanation').value = g.explanation;
    document.getElementById('grammarFormExamples').value = g.examples;
    document.getElementById('grammarFormTips').value = g.tips || '';
  } else {
    document.getElementById('modalGrammarTitle').textContent = 'Thêm ngữ pháp mới';
    document.getElementById('grammarFormId').value = '';
    document.getElementById('grammarFormTitle').value = '';
    document.getElementById('grammarFormStructure').value = '';
    document.getElementById('grammarFormExplanation').value = '';
    document.getElementById('grammarFormExamples').value = '';
    document.getElementById('grammarFormTips').value = '';
  }
  openModal(el.modalGrammar);
}

el.btnAddGrammar.onclick = () => openGrammarModal();

async function deleteGrammar(id) {
  if (confirm('Xóa điểm ngữ pháp này?')) {
    try {
      await api(`/api/grammars/${id}`, { method: 'DELETE' });
      broadcastRealtimeUpdate('GRAMMAR_DELETED');
      await selectSession(state.currentSessionId);
    } catch (e) {
      alert('Lỗi xóa ngữ pháp: ' + e.message);
    }
  }
}

el.btnSubmitGrammar.onclick = async (e) => {
  e.preventDefault();
  const id = document.getElementById('grammarFormId').value;
  const title = document.getElementById('grammarFormTitle').value.trim();
  const structure = document.getElementById('grammarFormStructure').value.trim();
  const explanation = document.getElementById('grammarFormExplanation').value.trim();
  const examples = document.getElementById('grammarFormExamples').value.trim();

  if (!title || !structure || !explanation || !examples) {
    alert('Vui lòng điền đủ các trường bắt buộc của cấu trúc ngữ pháp.');
    return;
  }

  const payload = {
    session_id: state.currentSessionId,
    title,
    structure,
    explanation,
    examples,
    tips: document.getElementById('grammarFormTips').value.trim() || null,
  };

  try {
    if (id) {
      await api(`/api/grammars/${id}`, {
        method: 'PUT',
        body: JSON.stringify(payload),
      });
    } else {
      await api('/api/grammars', {
        method: 'POST',
        body: JSON.stringify(payload),
      });
    }
    closeModal(el.modalGrammar);
    broadcastRealtimeUpdate('GRAMMAR_SAVED');
    await selectSession(state.currentSessionId);
  } catch (err) {
    alert('Lỗi lưu ngữ pháp: ' + err.message);
  }
};

// Save Session Notes
el.btnSaveNotes.onclick = async () => {
  if (!state.currentSessionId) return;
  const notes = el.sessionNotesText.value;
  try {
    await api(`/api/sessions/${state.currentSessionId}`, {
      method: 'PUT',
      body: JSON.stringify({ notes }),
    });
    alert('Đã lưu ghi chú thành công!');
  } catch (e) {
    alert('Lỗi lưu ghi chú: ' + e.message);
  }
};

// Search handling with debounce
let searchTimer = null;
el.searchInput.addEventListener('input', (e) => {
  clearTimeout(searchTimer);
  const q = e.target.value.trim();
  if (!q) {
    el.searchView.style.display = 'none';
    if (state.currentSessionId) {
      el.sessionView.style.display = 'block';
    } else {
      el.emptyWelcomeView.style.display = 'block';
    }
    return;
  }

  searchTimer = setTimeout(() => doSearch(q), 250);
});

async function doSearch(q) {
  try {
    const res = await api(`/api/search?q=${encodeURIComponent(q)}`);
    el.sessionView.style.display = 'none';
    el.emptyWelcomeView.style.display = 'none';
    el.searchView.style.display = 'block';
    el.searchQueryLabel.textContent = q;

    const { sessions, vocabularies, grammars } = res;
    const total = sessions.length + vocabularies.length + grammars.length;

    if (total === 0) {
      el.searchResultsList.innerHTML = `
        <div class="empty-state">
          <p>Không tìm thấy kết quả nào phù hợp.</p>
        </div>
      `;
      return;
    }

    let html = '';

    if (vocabularies.length > 0) {
      html += `<h3 style="font-size: 14px; margin: 16px 0 8px; color: var(--text-secondary);">Từ vựng (${vocabularies.length})</h3>`;
      html += '<div class="vocab-list">';
      vocabularies.forEach(v => {
        html += `
          <div class="vocab-card" onclick="selectSession(${v.session_id})" style="cursor: pointer;">
            <div class="vocab-word-group">
              <span class="vocab-word">${escapeHtml(v.word)}</span>
              ${v.ipa ? `<span class="vocab-ipa">${escapeHtml(v.ipa)}</span>` : ''}
              <span class="vocab-pos">${escapeHtml(v.meaning)}</span>
            </div>
            ${v.example_en ? `<div class="example-en" style="margin-top: 4px; font-size: 12px;">${escapeHtml(v.example_en)}</div>` : ''}
          </div>
        `;
      });
      html += '</div>';
    }

    if (grammars.length > 0) {
      html += `<h3 style="font-size: 14px; margin: 20px 0 8px; color: var(--text-secondary);">Ngữ pháp (${grammars.length})</h3>`;
      html += '<div class="grammar-list">';
      grammars.forEach(g => {
        html += `
          <div class="grammar-card" onclick="selectSession(${g.session_id})" style="cursor: pointer;">
            <div class="grammar-title">${escapeHtml(g.title)}</div>
            <div class="grammar-structure-box" style="margin-bottom: 4px;">${escapeHtml(g.structure)}</div>
            <div style="font-size: 12px; color: var(--text-secondary);">${escapeHtml(g.explanation)}</div>
          </div>
        `;
      });
      html += '</div>';
    }

    if (sessions.length > 0) {
      html += `<h3 style="font-size: 14px; margin: 20px 0 8px; color: var(--text-secondary);">Buổi học (${sessions.length})</h3>`;
      html += '<div style="display: flex; flex-direction: column; gap: 8px;">';
      sessions.forEach(s => {
        html += `
          <div class="vocab-card" onclick="selectSession(${s.id})" style="cursor: pointer;">
            <div style="font-weight: 700;">#${s.session_number}: ${escapeHtml(s.title)}</div>
            <div style="font-size: 12px; color: var(--text-muted);">${s.date} - ${escapeHtml(s.teacher || 'Tự học')}</div>
          </div>
        `;
      });
      html += '</div>';
    }

    el.searchResultsList.innerHTML = html;
  } catch (e) {
    alert('Lỗi tìm kiếm: ' + e.message);
  }
}

// Backup Export & Import (Supports both Server & Vercel client-side)
el.btnExport.onclick = () => {
  if (useLocalStore) {
    const data = {
      sessions: localDb.get('sessions'),
      vocabularies: localDb.get('vocabularies'),
      grammars: localDb.get('grammars'),
      exported_at: new Date().toISOString()
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'enotes_backup.json';
    a.click();
    URL.revokeObjectURL(url);
  } else {
    window.location.href = '/api/backup/export';
  }
};

el.btnImport.onclick = () => {
  el.importFileInput.click();
};

el.importFileInput.onchange = async (e) => {
  const file = e.target.files[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = async (ev) => {
    try {
      const data = JSON.parse(ev.target.result);
      if (!data.sessions || !data.vocabularies) {
        throw new Error('Định dạng file sao lưu không hợp lệ.');
      }
      if (confirm('Nhập file này sẽ ghi đè dữ liệu hiện tại. Bạn có chắc muốn tiếp tục?')) {
        await api('/api/backup/import', {
          method: 'POST',
          body: JSON.stringify(data),
        });
        broadcastRealtimeUpdate('BACKUP_IMPORTED');
        alert('Khôi phục dữ liệu thành công!');
        await loadSessions();
      }
    } catch (err) {
      alert('Lỗi đọc file JSON: ' + err.message);
    }
    el.importFileInput.value = '';
  };
  reader.readAsText(file);
};

// Keyboard navigation (Space for flashcard flip, Arrow keys)
window.addEventListener('keydown', (e) => {
  if (state.activeTab === 'practice' && el.modalSession.className.indexOf('active') === -1) {
    if (e.code === 'Space' && e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA') {
      e.preventDefault();
      toggleFlashcard();
    } else if (e.code === 'ArrowRight') {
      nextFlashcard();
    } else if (e.code === 'ArrowLeft') {
      prevFlashcard();
    }
  }
});

// Utilities
function escapeHtml(str) {
  if (!str) return '';
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

function escapeJsString(str) {
  if (!str) return '';
  return String(str).replace(/'/g, "\\'").replace(/"/g, '\\"');
}

// Cross-Tab & Cross-Client Realtime Synchronizer
const syncChannel = 'BroadcastChannel' in window ? new BroadcastChannel('enotes_realtime_sync') : null;

function broadcastRealtimeUpdate(action, payload = {}) {
  if (syncChannel) {
    syncChannel.postMessage({ action, payload, time: Date.now() });
  }
}

if (syncChannel) {
  syncChannel.onmessage = (event) => {
    // If another tab or window updated data, refresh current view seamlessly
    console.log('[Realtime Sync] Received broadcast:', event.data);
    loadSessions(state.currentSessionId);
  };
}

// Fallback storage event for older browsers
window.addEventListener('storage', (e) => {
  if (e.key && e.key.startsWith('enotes_')) {
    loadSessions(state.currentSessionId);
  }
});

// Periodic background sync (every 5 seconds) to keep data in sync if running backend server
setInterval(() => {
  if (!useLocalStore && document.visibilityState === 'visible') {
    loadStats();
  }
}, 5000);

// Role Management (Giáo viên / Học viên)
let currentRole = localStorage.getItem('enotes_current_role') || 'teacher';

function setRole(role) {
  currentRole = role;
  localStorage.setItem('enotes_current_role', role);

  const btnTeacher = document.getElementById('roleBtnTeacher');
  const btnStudent = document.getElementById('roleBtnStudent');

  if (role === 'student') {
    document.body.classList.add('role-student');
    document.body.classList.remove('role-teacher');
    if (btnStudent) btnStudent.classList.add('active');
    if (btnTeacher) btnTeacher.classList.remove('active');
    if (el.sessionNotesText) el.sessionNotesText.readOnly = true;
  } else {
    document.body.classList.add('role-teacher');
    document.body.classList.remove('role-student');
    if (btnTeacher) btnTeacher.classList.add('active');
    if (btnStudent) btnStudent.classList.remove('active');
    if (el.sessionNotesText) el.sessionNotesText.readOnly = false;
  }
}

document.getElementById('roleBtnTeacher')?.addEventListener('click', () => setRole('teacher'));
document.getElementById('roleBtnStudent')?.addEventListener('click', () => setRole('student'));

// Initialize Application & Role
setRole(currentRole);
loadSessions();
