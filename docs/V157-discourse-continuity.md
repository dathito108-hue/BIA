# V157 — Discourse Continuity & Anti-Repetition

V157 phát triển bộ sinh giao tiếp của BIA theo hướng hội thoại dài và tự nhiên hơn mà không thêm mô hình ngôn ngữ hay lõi AI thứ hai.

## Trục chính

`semantic turn -> continuity classification -> grounded reasoning -> contextual surface realization`

BIA phân biệt năm trạng thái quan hệ hội thoại:

- **New** — quan hệ mới;
- **Repeat** — hỏi lại đúng quan hệ vừa xét;
- **SameSubject** — giữ nguyên nguyên nhân/chủ thể và chuyển sang kết quả khác;
- **SameObject** — giữ nguyên kết quả và chuyển sang nguyên nhân/chủ thể khác;
- **Return** — quay lại một quan hệ đã xuất hiện trước đó.

Lịch sử semantic turn tăng từ 6 lên tối đa 12 lượt.

## Không làm bẩn lịch sử khi diễn đạt lại

Các thao tác như:

- nói ngắn gọn;
- giải thích sâu;
- sinh phần giải thích / so sánh / tóm tắt bên trong một yêu cầu ghép;

chỉ **render lại** reasoning hiện có. Chúng không còn bị ghi thành câu hỏi semantic mới.

Điều này giữ cho “ý trước”, “trường hợp trước” và tham chiếu thứ tự phản ánh đúng hội thoại của người dùng.

## Chống lặp

Khi người dùng hỏi lại đúng quan hệ và không yêu cầu giải thích sâu:

- BIA dùng mở câu kiểu `Vẫn ở quan hệ này, ...`;
- không lặp lại toàn bộ `Đường suy luận` và danh sách nguồn;
- thay bằng chỉ dấu ngắn rằng mạch bằng chứng không đổi.

Khi chuyển trong cùng mạch, lớp bề mặt dùng các chuyển tiếp như:

- `Tiếp theo mạch về ...`;
- `Vẫn với kết quả ...`;
- `Quay lại quan hệ giữa ...`.

Mọi verdict, confidence, Duyên, provenance và quyền thực thi vẫn do các lớp lõi hiện có quyết định. V157 không tạo factual edge hay DeviceAction.
