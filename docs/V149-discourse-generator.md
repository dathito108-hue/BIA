# V149 — Grounded discourse generator

V149 tiếp tục bộ sinh giao tiếp BIA sau DuyenWeave.

## Mục tiêu

Thay vì tạo một câu phản hồi đơn, BIA xây một kế hoạch diễn đạt hữu hạn gồm tối đa bốn bước:

1. verdict — kết luận hiện tại;
2. convergence — các nhánh Duyên hội tụ;
3. opposition — phản duyên hoặc nhánh ngược chiều;
4. limitation — giới hạn chắc chắn / nhu cầu thêm bằng chứng.

Thứ tự này được tạo từ `OpenAnswer + DuyenWeave`, không từ mô hình ngôn ngữ ngoài.

## Thuộc tính

- deterministic và bounded;
- giữ tương thích API cũ của `OpenAnswer` và `ReasoningVerdict`;
- không tạo tri thức mới trong lúc diễn đạt;
- không tạo DeviceAction;
- không biến một nhánh mạnh thành “nguyên nhân duy nhất” khi weave còn nhiều nhánh hoặc phản duyên.

V149 là tầng tổ chức diễn ngôn của cùng lõi BIA.
