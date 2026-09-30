# V146 — học cách diễn đạt và nối lượt theo vai nghĩa

Nâng trực tiếp IntegratedCognition, không thêm lõi AI hoặc mô hình bên ngoài.

## Thử trên điện thoại

1. `Cách nói làm phát sinh: gây ra`
2. `Ghi nhớ rằng gió mạnh làm phát sinh sóng lớn.`
3. `Ghi nhớ rằng sóng lớn gây ra thuyền lắc.`
4. `Gió mạnh có làm phát sinh sóng lớn không?`
5. `Còn kết quả thuyền lắc thì sao?`
6. `Dựa vào đâu?`

Cách nói được người dùng chỉ định nghĩa một cách tường minh. BIA lưu tối đa 16 cụm từ, mỗi cụm 2–6 từ, áp dụng trong vị trí quan hệ của câu hỏi/câu dạy và lưu qua journal hiện có. Đây là học từ vựng có giám sát, chưa tự khám phá ngữ pháp từ hội thoại tự do. `Rút cách nói làm phát sinh` bỏ ánh xạ; các nguồn tri thức đã ghi vẫn giữ nguyên. Chưa hỗ trợ ánh xạ quan hệ khác ngoài “gây ra”. Phủ định, giả định, dấu phân cách lệnh và ánh xạ chồng nhau bị từ chối hoặc không mở rộng.

Theo dõi lượt hỏi bằng vai nguyên nhân/kết quả: `Còn nguyên nhân X thì sao?` thay chủ thể, giữ kết quả; `Còn kết quả Y thì sao?` giữ nguyên nhân, thay kết quả. `Còn X thì sao?` yêu cầu làm rõ vai, không tự chọn. Ngữ cảnh nối lượt vẫn chỉ tồn tại trong phiên.

Trả lời dựng từ phán quyết và đường quan hệ hiện có. “Dựa vào đâu?” tính lại câu hỏi theo dữ liệu hiện tại rồi chọn các nguồn có cạnh nằm trên đường đã chọn; loại nguồn không nằm trên đường. Trong trường hợp chưa có đường đơn nhất (thiếu bằng chứng/xung đột/phản thực), trả lời rõ chưa xác định được, không giả danh nguồn liên quan. Nguồn vẫn là thông tin được báo lại, chưa xác minh độc lập.

## Kiểm chứng và giới hạn

Các test so sánh trước/sau khi học cụm từ, đổi thực thể ở câu kiểm tra, khôi phục checkpoint, rút cách nói, chặn phủ định và thao tác ngoài, đổi vai qua nhiều lượt, chọn đúng nguồn. Android kiểm tra bằng chat JNI thật. Đây là chuyển giao từ vựng trong ngữ pháp giới hạn, không phải đánh giá độc lập về hội thoại tự do hoặc trí tuệ tổng quát. Không tuyên bố đạt chất lượng AI lớn. Không dùng thêm GPU, trọng số ngoài hay dịch vụ AI.
