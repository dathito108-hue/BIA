# V159 — Conversational Implicature & Response Economy

V159 giúp BIA phản hồi theo **ý định hội thoại ngầm** thay vì coi mọi lượt ngắn là một câu hỏi semantic mới.

## Conversation acts

BIA nhận biết bounded các act:

- **Acknowledge** — “hiểu rồi”;
- **Confirm** — “đúng chứ?”, “vậy là đúng?”;
- **Doubt** — “có chắc không?”, “thật không?”;
- **Expand** — “nói thêm”, “đào sâu hơn”;
- **NewOnly** — “có gì mới?”, “chỉ nói phần mới”.

Các act này không tạo factual edge, không tạo DeviceAction và không tự sinh semantic turn.

## Response economy

- Acknowledge không lặp reasoning đã trình bày.
- Confirm trả bản rút gọn của kết luận hiện tại.
- Doubt kiểm tra lại ở mức sâu nhưng không tăng confidence chỉ vì người dùng hỏi lại.
- Expand mở rộng trên cùng mạch bằng chứng.
- NewOnly so sánh snapshot bounded của **các nguồn liên quan tới đúng quan hệ hiện tại**.

Nguồn không liên quan không được báo là “mới” chỉ vì vừa được thêm vào hệ thống.

## Delta bằng chứng

Với “có gì mới?”, BIA phân biệt:

- nguồn thêm;
- nguồn thay đổi;
- nguồn đã rút.

Nếu không có delta liên quan, BIA nói ngắn rằng chưa có bằng chứng mới và không lặp lại phần cũ.

Snapshot chỉ chứa tên nguồn + chữ ký ổn định 64-bit của nội dung/kind, tối đa theo giới hạn nguồn hiện có; không sinh thêm mô hình hay memory backend.

## Sửa continuity

“Dựa vào đâu?” refresh đường bằng chứng bằng re-render read-only, không còn tạo semantic turn giả. Do đó “ý trước” vẫn trỏ đúng lượt người dùng thực sự đã hỏi.

V159 tiếp tục giữ một lõi BIA duy nhất; không LLM, Transformer, SSM, Mamba hay AI backend thứ hai.
