# V154 — Compositional Dialogue Generation

V154 cho phép BIA xử lý nhiều mục tiêu giao tiếp trong một lượt, nhưng vẫn dùng cùng một lõi reasoning.

Ví dụ yêu cầu:
- giải thích hiện tại;
- so sánh với trường hợp trước;
- tóm tắt ngắn gọn.

BIA xây `CompositeDialoguePlan` hữu hạn rồi thực thi từng goal bằng các API sinh hiện có. Mỗi phần được suy luận lại từ nguồn hiện tại; phần ghép không tự tạo thêm factual claim.

## Invariants

- tối đa một tập goal nhỏ đã biết;
- không biến văn bản ghép thành tri thức;
- không sinh DeviceAction;
- không bypass authority;
- thiếu trường hợp trước thì hỏi rõ thay vì giả lập dữ liệu.

Luồng:

`input -> CompositeDialoguePlan -> continuity lookup -> grounded re-answer -> section composition`
