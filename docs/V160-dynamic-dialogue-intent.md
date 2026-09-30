# V160 — Dynamic Dialogue Intent Composition

V160 nâng bộ giao tiếp BIA từ xử lý một conversation act mỗi lượt sang **lập kế hoạch nhiều ý định trong cùng một câu**.

## Ví dụ

`Tôi hiểu rồi, nhưng có chắc không, nếu có gì mới thì chỉ nói phần mới thôi.`

được phân tích thành:

1. Acknowledge
2. Doubt
3. NewOnly

BIA không chạy ba câu trả lời độc lập. Nó tạo một kế hoạch bounded rồi thực hiện theo semantics chung.

## Thứ tự semantics

- **Doubt / Expand** là thao tác reasoning: kiểm tra lại hoặc mở rộng trên cùng quan hệ.
- **NewOnly** là bộ lọc đầu ra và được áp dụng sau reasoning.
- **Acknowledge** chỉ thêm lời xác nhận khi không bị NewOnly yêu cầu loại phần cũ.
- **Doubt** ưu tiên hơn Confirm nếu cả hai xuất hiện, tránh vừa xác nhận vừa nghi ngờ cùng một kết luận bằng hai câu trùng nhau.
- Expand + Confirm tạo một phần mở rộng sâu; verdict trong reasoning đã bao hàm trạng thái xác nhận hiện tại.

Điều này tránh lỗi tuần tự: nếu Doubt cập nhật snapshot trước NewOnly, delta bằng chứng sẽ bị mất. V160 giữ snapshot cũ cho tới khi bộ lọc NewOnly tính xong phần thay đổi.

## Giới hạn

Kế hoạch chỉ nhận các intent bounded đã định nghĩa và chỉ kích hoạt khi có ít nhất hai intent. Lượt một intent tiếp tục dùng đường V159 đã kiểm thử.

V160 không tạo factual edge, không tạo semantic turn giả, không tạo DeviceAction và không cấp authority. Kiến trúc vẫn là một lõi BIA duy nhất, không LLM/Transformer/SSM/Mamba/backend AI thứ hai.
