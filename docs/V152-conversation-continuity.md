# V152 — Conversational Continuity Generator

V152 thêm bộ nhớ diễn ngôn hữu hạn vào cùng lõi BIA.

## Khả năng

- nhớ tối đa 6 lượt câu hỏi gần nhất;
- tham chiếu lại `ý vừa rồi`, `trường hợp trước`;
- nhận diện `cái thứ nhất` / `cái thứ hai` theo subject/object của câu hỏi gần nhất;
- nếu tham chiếu xác định được đối tượng nhưng chưa xác định vai nhân/quả, BIA hỏi lại thay vì đoán.

## Giới hạn an toàn

Continuity chỉ giữ cấu trúc hội thoại. Nó không:
- tạo tri thức mới;
- sửa nguồn;
- tạo DeviceAction;
- suy đoán vai semantic khi có nhiều khả năng.

Luồng:

`conversation input -> bounded continuity -> semantic role check -> existing reasoning/generation`
