# V158 — Contextual Pragmatics

V158 phát triển BIA từ continuity bề mặt sang **suy ngữ dụng có kiểm soát**.

## Mục tiêu

Cho phép người dùng nói tự nhiên hơn mà không phải lặp đầy đủ hai vế của quan hệ ở mọi lượt.

Ví dụ, sau khi đã hỏi về một quan hệ của `mưa`, người dùng có thể nói:

- `Còn bùn?`
- `Còn gió thì sao?`

BIA chỉ tự điền phần bị lược khi vai trò của đối tượng được xác nhận duy nhất từ lịch sử semantic turn hoặc các nguồn đã ghi.

## Quy tắc vai trò

BIA tổng hợp bằng chứng vai trò từ:

- các subject/object trong tối đa 12 semantic turn gần nhất;
- subject/object trong các nguồn Report/Observation hiện có.

Nếu đối tượng chỉ xuất hiện ở vai subject, nó có thể thay nguyên nhân/chủ thể hiện tại.
Nếu chỉ xuất hiện ở vai object, nó có thể thay kết quả hiện tại.

Nếu đối tượng xuất hiện ở cả hai vai, hoặc chưa có bằng chứng vai trò, BIA hỏi lại thay vì đoán.

Ví dụ:

`mưa -> bùn -> đường trơn`

thì `bùn` vừa là kết quả vừa là nguyên nhân. Câu `Còn bùn thì sao?` phải được làm rõ.

## Chuyển chủ đề

Các câu rõ ràng như:

- `chuyển chủ đề sang X`
- `đổi chủ đề sang X`
- `giờ chuyển sang X`

sẽ xóa **quan hệ đang hoạt động** nhưng không xóa nguồn tri thức hay lịch sử bounded. Vì vậy đại từ hoặc câu `Tại sao?` ngay sau khi chuyển chủ đề không vô tình bám vào quan hệ cũ.

## Ranh giới

V158 không tạo factual edge, không sửa nguồn, không tạo DeviceAction và không cấp quyền thực thi.
Nó không dùng LLM/Transformer/SSM/Mamba hay lõi AI thứ hai.
