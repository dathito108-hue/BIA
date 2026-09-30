# V151 — Context-Adaptive Expression

V151 cho phép BIA tự chọn độ sâu diễn đạt từ chính trạng thái suy luận.

## Quy tắc chọn style

- **Deep** khi có xung đột, phản duyên, bất định cao, nhiều nhánh chồng lấp hoặc đường suy luận sâu.
- **Brief** khi cấu trúc rất đơn giản và mức bất định thấp.
- **Standard** cho các trường hợp ở giữa.

Bộ chọn chỉ đọc `OpenAnswer + DuyenWeave + uncertainty`. Nó không thay đổi verdict hay bằng chứng.

Yêu cầu rõ của người dùng như `Nói ngắn gọn` hoặc `Giải thích kỹ hơn` luôn ghi đè lựa chọn tự động.

Luồng mặc định:

`Reasoning -> DuyenWeave -> Metacognition -> AdaptiveExpressionSelector -> GenerativeCognition`

V151 không thêm lõi AI, không dùng mô hình ngôn ngữ bên ngoài và không tạo thêm quyền thực thi.
