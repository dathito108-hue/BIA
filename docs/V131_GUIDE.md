# BIA V131 — Bản tích hợp năm hướng trí tuệ

Bản này nối ngôn ngữ, quản lý nguồn, phản hồi kết quả, lập kế hoạch và suy rộng
vào chat bằng các lệnh tiếng Việt có cấu trúc. Đây là một bộ điều phối trên
SemanticReasoner, AutonomousCognitiveLoop và DeliberativePlanner hiện có của BIA;
không thêm mô hình hoặc dịch vụ AI bên ngoài.

## 1. Ngôn ngữ và nguồn bằng chứng

Gửi lần lượt:

```text
Quan sát pin: pin yếu dẫn tới máy chậm.
Nguồn nhiệt: máy chậm kéo theo phản hồi trễ.
Hỏi: pin yếu có dẫn tới phản hồi trễ không?
Đính chính nhiệt: máy chậm ngăn phản hồi trễ.
Hỏi: pin yếu có dẫn tới phản hồi trễ không?
Rút nguồn nhiệt
Hỏi: pin yếu có dẫn tới phản hồi trễ không?
```

Các câu trả lời sẽ lần lượt thể hiện chuỗi ủng hộ, nhánh phản đối và thiếu bằng
chứng. Nội dung nguồn cũ không được trộn vào nguồn đã đính chính. Rút một nguồn
không xóa các nguồn khác. `Giả thuyết tên: ...` lưu một giả thuyết nhưng chưa dùng
làm bằng chứng. `Quan sát` cũng là nhãn do người dùng cung cấp, không có nghĩa
BIA đã tự kiểm chứng bằng cảm biến.

`Hỏi:` tra cứu riêng các nguồn được quản lý bằng nhóm lệnh này. Chat và tài liệu
đã học bằng luồng cũ vẫn giữ nguyên đường xử lý của chúng; đính chính ở đây không
hứa xóa mọi quan hệ đã học trong bộ nhớ cũ. Tên nguồn được chuẩn hóa không dấu.

Parser có thêm “dẫn tới”, “kéo theo”, “là nguyên nhân của”, “B là do A”. Các
mẫu phủ định/chưa chắc được hỗ trợ sẽ không bị ghi nhầm thành quan hệ khẳng định;
đây chưa phải bộ logic phủ định ngôn ngữ tổng quát. Trong `Hỏi:`, “nó” và “điều đó”
chỉ được thay bằng chủ thể gần nhất khi ngữ cảnh xác định duy nhất. Nếu không rõ,
BIA yêu cầu nêu tên, không đoán.

## 2. Lập kế hoạch và học từ phản hồi

```text
Kỹ năng nhanh: sẵn sàng -> hoàn tất
Kỹ năng chuẩn bị: sẵn sàng -> đã chuẩn bị
Kỹ năng dự phòng: đã chuẩn bị -> hoàn tất
Lập kế hoạch: sẵn sàng -> hoàn tất
Kết quả nhanh: thất bại
```

BIA ban đầu đề xuất `nhanh`, sau phản hồi thất bại sẽ mô phỏng đường
`chuẩn bị → dự phòng`. `Kết quả nhanh: thành công` cho phép xem xét lại kỹ năng;
đây là phản hồi do người dùng xác nhận, không phải biên nhận thực thi trên thiết bị.
Lặp cùng trạng thái phản hồi không tăng điểm. `Lập lại kế hoạch:` kiểm tra lại
mục tiêu gần nhất trong phiên.

Nhiều điều kiện/kết quả phân cách bằng dấu phẩy. Ràng buộc tránh dùng dạng:

```text
Kỹ năng nguy hiểm: sẵn sàng -> hoàn tất, hỏng
Lập kế hoạch: sẵn sàng -> hoàn tất; tránh hỏng
```

Kỹ năng ở đây mô tả điều kiện và kết quả trong một mô hình đơn giản. BIA mô phỏng
và đề xuất, **không tự thực thi**. Hàng đợi thao tác và phê duyệt Android vẫn tách
riêng. Các kỹ năng chỉ thêm điều kiện trong mô hình; chưa mô tả hiệu ứng xóa điều
kiện hay hành động ngoài đời. Tìm kiếm tối đa bốn bước, tám nhánh và beam 12,
không phải bộ giải kế hoạch đầy đủ.

## 3. Suy rộng có phản ví dụ

```text
Ví dụ làm mát: quạt a -> giảm nhiệt
Ví dụ làm mát: quạt b -> giảm nhiệt
Suy rộng: làm mát cho quạt c
Phản ví dụ làm mát: quạt d -> giảm nhiệt
Suy rộng: làm mát cho quạt c
```

Hai trường hợp khác tên có cùng kết quả cho phép đề xuất một **giả thuyết** cho
trường hợp mới. Một ví dụ lặp không được tính hai lần. Kết quả khác nhau hoặc phản
ví dụ sẽ chặn suy rộng. Tên trường hợp khác nhau không chứng minh độc lập thống kê
hay quan hệ nhân quả; kết quả không tự trở thành sự thật hoặc kỹ năng thực thi.

## Lưu/khôi phục và giới hạn

Nguồn, đính chính, kỹ năng, phản hồi và ví dụ được lưu cùng continuity của ứng dụng.
Khôi phục phát lại nhật ký nhận thức, không phát lại thao tác thiết bị. Mục tiêu
mô phỏng gần nhất và tham chiếu hội thoại là ngữ cảnh phiên; sau mở lại hãy gửi
lại câu hỏi/lệnh lập kế hoạch. Nhật ký sai định dạng bị từ chối nguyên khối.

Giới hạn: 128 thay đổi trong nhật ký, 32 nguồn, 32 kỹ năng, 64 ví dụ, 1.024 ký tự
mỗi lệnh, tám điều kiện/kết quả mỗi kỹ năng và tám mệnh đề được xét mỗi nguồn.
Khi nhật ký đầy, BIA từ chối ghi mới thay vì xóa lịch sử khiến nguồn đã rút sống lại.

`/integrated` chạy bộ kiểm tra native: 32 phiên với diễn đạt và chuỗi kế hoạch khác
nhau, kiểm tra cả năm nhóm cùng lưu/khôi phục. PASS chỉ chứng minh các tình huống
này; không chứng minh hiểu tiếng Việt tự do, tự học mọi kỹ năng hay AGI.
