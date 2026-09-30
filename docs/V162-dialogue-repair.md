# V162 — Cooperative Grounding & Dialogue Repair

V162 bổ sung lớp sửa hiểu hội thoại bounded trên cùng lõi BIA.

- nhận biết tín hiệu người dùng nói BIA hiểu sai;
- nhận phần làm rõ/thay thế mà không tự biến nó thành factual edge;
- giữ trạng thái sửa tạm thời cho tới khi có semantic question mới;
- semantic question mới tái neo DialogueGoalState;
- topic shift xóa trạng thái repair;
- không tạo DeviceAction, execution authority hoặc semantic turn giả;
- giới hạn input 1.024 ký tự.

Mục tiêu là để hội thoại tự nhiên hơn: người dùng có thể sửa BIA giữa dòng mà không phải reset phiên.
