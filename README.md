# E-Notes — Sổ tay học tiếng Anh (Rust Core & Unslop Design)

Ứng dụng web tinh gọn, hiệu năng cao phục vụ việc **ghi chép buổi học tiếng Anh**, quản lý **từ vựng (Vocabulary)**, **ngữ pháp (Grammar)** theo từng buổi, hỗ trợ **phát âm chuẩn (Audio TTS)**, **ôn tập thẻ nhớ (Flashcard)** và **sao lưu dữ liệu (JSON Export/Import)**.

Được phát triển với tiêu chuẩn khắt khe:
- **Core 100% bằng Rust**: Toàn bộ HTTP REST server, routing, xử lý logic, truy vấn SQLite và bảo mật đều được viết thuần bằng Rust (không phụ thuộc vào runtime cồng kềnh).
- **Hạn chế TypeScript / JS tối đa**: Frontend sử dụng Vanilla JS siêu nhẹ (~400 dòng), không cần `node_modules`, không tốn tài nguyên build frontend.
- **Thiết kế theo triết lý Unslop ([heydamir/unslop](https://github.com/heydamir/unslop.git))**: Loại bỏ hoàn toàn các lỗi "AI slop" (gradient tím bão hòa, viền phát sáng lòe loẹt, emoji bừa bãi). Thay vào đó là giao diện chuẩn mực, tối giản (editorial slate theme), khoảng cách 4px có kỷ luật, tập trung 100% vào trải nghiệm đọc và học tập.

---

## 🌟 Các tính năng chính

### 1. Quản lý buổi học (Lesson Sessions)
- Lưu trữ theo số thứ tự buổi (`#01`, `#02`...), ngày học, tên giảng viên / nguồn học.
- Ghi chú mục tiêu bài học (Goals) và tóm tắt / dặn dò sau buổi học.
- Đếm tự động số lượng từ vựng và điểm ngữ pháp của từng buổi.

### 2. Sổ tay từ vựng chi tiết (Vocabulary per Session)
- **Từ & Phiên âm IPA**: Ví dụ `articulate` — `/ɑːrˈtɪkjuleɪt/`.
- **Từ loại**: `noun`, `verb`, `adj`, `idiom`, `phrasal verb`...
- **Nghĩa tiếng Việt** & **Ghi chú Collocations/Bẫy từ vựng**.
- **Câu ví dụ tiếng Anh + Bản dịch nghĩa** trực quan.
- **Phát âm âm thanh bản ngữ (Audio TTS)**: Nhấn biểu tượng loa để nghe giọng đọc tiếng Anh chuẩn thông qua Web Speech API.
- **Trạng thái ghi nhớ**: Phân loại theo 3 mức `Đang học (learning)`, `Cần ôn tập (reviewing)`, `Đã thuộc (mastered)`.

### 3. Sổ tay cấu trúc ngữ pháp (Grammar Points)
- Tên chủ điểm ngữ pháp (ví dụ: *Mixed Conditionals*, *Inversion*, *Diplomatic Language*...).
- Khung hiển thị công thức / cấu trúc rõ ràng với font monospaced.
- Giải thích chi tiết cách dùng và sắc thái ngữ cảnh.
- Bộ câu ví dụ cấu trúc (+ Khẳng định, - Phủ định, ? Nghi vấn).
- Khung lưu ý / mẹo làm bài tập tránh bẫy.

### 4. Chế độ luyện tập thẻ ghi nhớ (Flashcard Practice)
- Lật mở thẻ để kiểm tra trí nhớ từ vựng (hỗ trợ phím tắt `Space` để lật thẻ, phím mũi tên `←` / `→`).
- Đánh dấu nhanh "Đã thuộc" hoặc "Cần ôn" để cập nhật trực tiếp tiến độ học.

### 5. Tìm kiếm toàn cục tức thì (Instant Search)
- Tìm kiếm từ khóa theo thời gian thực (realtime) xuyên suốt tất cả các buổi học, từ vựng, ngữ pháp.
- Nhấp vào kết quả để chuyển thẳng tới bài học tương ứng.

### 6. Sao lưu & Phục hồi an toàn (Backup & Restore)
- Xuất toàn bộ dữ liệu ra file `.json` chỉ với 1 click.
- Nhập lại file sao lưu bất kỳ lúc nào để không bao giờ sợ mất dữ liệu.

---

## 🏗️ Kiến trúc dự án

```
E-Notes/
├── Cargo.toml          # Cấu hình dependency Rust (rusqlite, serde, chrono)
├── src/
│   ├── main.rs         # Điểm khởi động server, nạp biến môi trường, định tuyến
│   ├── models.rs       # Cấu trúc dữ liệu Rust (Session, Vocabulary, Grammar, Stats)
│   ├── db.rs           # Quản lý SQLite database, migrations và seed mẫu ban đầu
│   ├── http.rs         # HTTP 1.1 server thuần, route matcher, request/response builder
│   └── handlers.rs     # Xử lý các REST API endpoints và phục vụ file tĩnh
├── static/
│   ├── index.html      # Giao diện web ngữ nghĩa, chuẩn accessibility
│   ├── style.css       # Hệ thống design tokens Unslop (lưới 4px, slate palette)
│   └── app.js          # Client logic Vanilla JS cực nhẹ, không cần node_modules
├── Dockerfile          # Multi-stage build cho triển khai web 1-click
├── render.yaml         # Cấu hình deploy tự động lên Render.com
├── fly.toml            # Cấu hình deploy tự động lên Fly.io
├── run.bat             # File chạy nhanh trên Windows
├── run.sh              # File chạy nhanh trên Linux/macOS
└── README.md
```

---

## 🚀 Hướng dẫn chạy tại máy (Local Development)

### Cách 1: Sử dụng Cargo (Yêu cầu đã cài Rust)

1. Mở terminal tại thư mục dự án:
```bash
cargo run --release
```
2. Mở trình duyệt và truy cập:
👉 **[http://localhost:8080](http://localhost:8080)** (hoặc `http://localhost:3000`)

### Cách 2: Chạy file nhấp đúp (Windows)
- Nhấp đúp vào file `run.bat`.

---

## ☁️ Hướng dẫn Deploy lên Web tự động (Auto Simple Deploy)

Dự án được tối ưu hóa đặc biệt với `Dockerfile` đa tầng (multi-stage), dung lượng image nhỏ gọn và tự động lưu cơ sở dữ liệu SQLite:

### 1. Deploy lên Render (Miễn phí & Tự động 100%)
1. Đẩy mã nguồn lên GitHub của bạn.
2. Đăng nhập [Render.com](https://render.com) > Chọn **New +** > **Blueprint**.
3. Chọn repo GitHub của bạn. Render sẽ tự động đọc file `render.yaml` và triển khai web service chỉ sau vài phút.

### 2. Deploy lên Fly.io
```bash
fly launch
fly deploy
```

### 3. Deploy với Docker trên bất kỳ VPS nào
```bash
# Build Docker image
docker build -t e-notes .

# Chạy container với mount ổ đĩa lưu dữ liệu
docker run -d -p 3000:3000 -v enotes_data:/data --name e-notes-app e-notes
```

---

## 🎨 Điểm nhấn thiết kế chuẩn Unslop

- **Màu sắc có kỷ luật**: Sử dụng bảng màu Slate trung tính, nền `#F8FAFC`, bề mặt thẻ trắng `#FFFFFF` với viền tóc `1px solid #E2E8F0`. Không dùng gradient lòe loẹt.
- **Hệ thống khoảng cách 4px**: Mọi lề (padding, margin, gap) tuân thủ nghiêm ngặt bội số của 4px (8, 12, 16, 20, 24, 32px).
- **Trọng tâm là nội dung**: Typography phân cấp rõ ràng (font chữ hệ thống sắc nét, font Monospace cho phiên âm IPA và công thức ngữ pháp).
