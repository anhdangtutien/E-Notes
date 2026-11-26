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

// Seed data for standalone/Vercel mode
const SEED_DATA = {
  sessions: [
    {
      id: 2,
      session_number: 2,
      title: "Buổi 2: Conditional Sentences & Hypothetical Scenarios",
      date: new Date().toISOString().split('T')[0],
      teacher: "Ms. Sarah Jenkins",
      goals: "Thành thạo câu điều kiện hỗn hợp (Mixed Conditionals) và cấu trúc Wish/If only.",
      notes: "Luyện tập phân biệt điều kiện loại 2 (hiện tại) và loại 3 (quá khứ).",
      created_at: "2026-11-25 10:00:00",
      updated_at: "2026-11-25 10:00:00"
    },
    {
      id: 1,
      session_number: 1,
      title: "Buổi 1: Business English & Negotiations",
      date: new Date().toISOString().split('T')[0],
      teacher: "Mr. David Miller",
      goals: "Nắm vững kỹ năng thuyết trình dự án và đàm phán hợp đồng thương mại.",
      notes: "Cần chú ý cách diễn đạt gián tiếp (diplomatic language) khi từ chối đối tác lịch sự.",
      created_at: "2026-11-25 09:00:00",
      updated_at: "2026-11-25 09:00:00"
    }
  ],
  vocabularies: [
    {
      id: 1,
      session_id: 1,
      word: "articulate",
      ipa: "/ɑːrˈtɪkjuleɪt/",
      pos: "verb / adjective",
      meaning: "diễn đạt gãy gọn, lưu loát / ăn nói rõ ràng",
      example_en: "She gave a witty, entertaining, and articulate presentation.",
      example_vi: "Cô ấy đã có một bài thuyết trình hóm hỉnh, cuốn hút và diễn đạt rất lưu loát.",
      status: "mastered",
      notes: "Thường đi với 'articulate an idea / opinion'",
      created_at: "2026-11-25 09:10:00"
    },
    {
      id: 2,
      session_id: 1,
      word: "compromise",
      ipa: "/ˈkɑːmprəmaɪz/",
      pos: "noun / verb",
      meaning: "sự thỏa hiệp, nhượng bộ lẫn nhau",
      example_en: "We reached a compromise after two hours of tough debate.",
      example_vi: "Chúng tôi đã đạt được thỏa hiệp sau hai giờ tranh luận căng thẳng.",
      status: "learning",
      notes: "Collocation: reach/arrive at a compromise",
      created_at: "2026-11-25 09:15:00"
    },
    {
      id: 3,
      session_id: 1,
      word: "concession",
      ipa: "/kənˈseʃn/",
      pos: "noun",
      meaning: "sự nhượng bộ trong đàm phán",
      example_en: "Both parties made major concessions to secure the deal.",
      example_vi: "Cả hai bên đều đã có những nhượng bộ lớn để ký kết được hợp đồng.",
      status: "learning",
      notes: "Make a concession: nhượng bộ điều gì đó",
      created_at: "2026-11-25 09:20:00"
    },
    {
      id: 4,
      session_id: 2,
      word: "contingency",
      ipa: "/kənˈtɪndʒənsi/",
      pos: "noun",
      meaning: "kế hoạch dự phòng, trường hợp khẩn cấp có thể xảy ra",
      example_en: "We need a contingency plan in case the supplier delays shipments.",
      example_vi: "Chúng ta cần một kế hoạch dự phòng đề phòng trường hợp nhà cung ứng chậm giao hàng.",
      status: "reviewing",
      notes: "Contingency plan = Plan B",
      created_at: "2026-11-25 10:10:00"
    },
    {
      id: 5,
      session_id: 2,
      word: "hindsight",
      ipa: "/ˈhaɪndsaɪt/",
      pos: "noun",
      meaning: "sự suy xét lại sau khi sự việc đã xảy ra",
      example_en: "With hindsight, we should have invested in automated testing much sooner.",
      example_vi: "Nhìn lại quá khứ, đáng lẽ chúng ta nên đầu tư vào kiểm thử tự động sớm hơn.",
      status: "learning",
      notes: "Idiom: 'With hindsight' hoặc 'In hindsight'",
      created_at: "2026-11-25 10:15:00"
    }
  ],
  grammars: [
    {
      id: 1,
      session_id: 1,
      title: "Diplomatic Language (Ngôn ngữ ngoại giao / Giảm nhẹ)",
      structure: "Subject + seem / appear / tend + to V | I was wondering if... | Wouldn't it be better if...",
      explanation: "Sử dụng để giảm bớt sự gay gắt khi nêu ý kiến trái chiều hoặc chỉ trích đối tác trong môi trường công sở quốc tế.",
      examples: "Direct: 'Your price is too high.'\nDiplomatic: 'Your pricing seems to be slightly above our current budget constraints.'\nDirect: 'Send me the report now.'\nDiplomatic: 'I was wondering if you could possibly send over the report by 3 PM.'",
      tips: "Tránh dùng 'You must' hoặc 'You are wrong' trong đàm phán chuyên nghiệp.",
      created_at: "2026-11-25 09:30:00"
    },
    {
      id: 2,
      session_id: 2,
      title: "Mixed Conditional: Past Action with Present Result",
      structure: "If + S + had + V3/ed (Quá khứ), S + would + V-inf (Hiện tại)",
      explanation: "Diễn tả một giả định trái ngược với quá khứ, dẫn đến một kết quả ảnh hưởng trực tiếp đến hiện tại.",
      examples: "+ If I had taken that training course last year, I would speak fluent English today.\n- If he hadn't missed the flight yesterday, he wouldn't be stuck at the airport now.\n? Would you have this high position today if you hadn't worked so hard?",
      tips: "Chú ý trạng từ thời gian: vế If có dấu hiệu quá khứ (yesterday, last year), vế chính có dấu hiệu hiện tại (now, today).",
      created_at: "2026-11-25 10:30:00"
    }
  ]
};

// LocalStorage Controller
const localDb = {
  get(key) {
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
          <button class="btn btn-secondary btn-sm" onclick="openVocabModal(${v.id})">Sửa</button>
          <button class="btn btn-secondary btn-sm btn-danger-ghost" onclick="deleteVocab(${v.id})">Xóa</button>
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
          <button class="btn btn-secondary btn-sm" onclick="openGrammarModal(${g.id})">Sửa</button>
          <button class="btn btn-secondary btn-sm btn-danger-ghost" onclick="deleteGrammar(${g.id})">Xóa</button>
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
    await loadSessions(saved.id);
  } catch (err) {
    alert('Lỗi lưu buổi học: ' + err.message);
  }
};

// Vocab Modals
function openVocabModal(id = null) {
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

el.btnAddVocab.onclick = () => openVocabModal();

async function deleteVocab(id) {
  if (confirm('Xóa từ vựng này?')) {
    try {
      await api(`/api/vocabularies/${id}`, { method: 'DELETE' });
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

// Initialize Application
loadSessions();
