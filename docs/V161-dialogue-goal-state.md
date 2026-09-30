# V161 — Dialogue State & Implicit Goal Tracking

V161 cho BIA giữ **mục tiêu hội thoại đang theo đuổi** qua nhiều lượt ngắn mà không biến mục tiêu đó thành tri thức hay hành động.

## Goal state bounded

Mỗi quan hệ hội thoại đang hoạt động có một state:

- **Explore** — đang khám phá/giải thích;
- **Verify** — người dùng đang kiểm tra độ chắc;
- **Challenge** — người dùng muốn tìm phản chứng;
- **Conclude** — người dùng muốn chốt kết luận hiện tại.

Trail mục tiêu được giới hạn tối đa 8 chuyển trạng thái. Khi một semantic question mới thay đổi quan hệ, trail được tái neo từ Explore.

## Theo dõi xuyên nhiều lượt

Ví dụ:

1. `Mưa có gây ra đường trơn không?`
2. `Có chắc không?`
3. `Có phản chứng không?`
4. thêm nguồn mới;
5. `Vậy kết luận thế nào?`

BIA giữ cùng quan hệ làm mục tiêu. Bước kết luận được tính lại từ toàn bộ nguồn hiện tại, không dùng lại verdict cũ.

## Phản chứng grounded

`Có phản chứng không?` chạy tìm kiếm bounded trên chính đồ thị nhân–duyên của quan hệ hiện tại.

BIA chỉ báo nguồn phản chứng khi nguồn đó tạo một cạnh **Inhibits** nằm trên một causal path phản đối từ subject tới object. Nguồn không liên quan không được đưa vào.

Nếu có nhánh phản đối nhưng provenance ức chế không thể tách đơn nhất, BIA nói rõ giới hạn thay vì đoán nguồn.

## Re-anchoring

- semantic question mới → reset goal trail cho quan hệ mới;
- topic shift rõ ràng → xóa active goal;
- acknowledgement, source inspection và output filtering không tạo semantic turn giả;
- nguồn được thêm/đính chính không tự đổi active relation, nên lượt `chốt lại` sau đó sẽ tái suy luận với dữ liệu mới.

Goal state là state hội thoại trong phiên, không phải factual memory và không được dùng để cấp authority.

V161 tiếp tục một lõi BIA duy nhất; không LLM, Transformer, SSM, Mamba hay backend AI thứ hai.
