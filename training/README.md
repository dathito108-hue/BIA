# BIA V144 — đợt học đa nhiệm CPU đầu tiên

Lệnh trong APK: **huấn luyện tổng quát**. Lệnh CLI: `cargo run --release --example train_v144 -- training-output`.

Đợt này dùng cơ chế học ký hiệu hiện có của IntegratedCognition, không thêm lõi AI, không thay bằng LLM/SSM/Transformer. 36 bản ghi gồm nguồn, mô tả kỹ năng và ví dụ; sáu nhóm nhãn tệp, sản phẩm, hình ảnh, dữ liệu, thiết bị, công việc. Đây là sáu kịch bản tổng hợp cùng cấu trúc, chưa phải dữ liệu thực phong phú của sáu lĩnh vực. Tên `hoc144...` dành riêng cho giáo trình, không được hiểu là thông tin về thế giới thật.

Đánh giá 36 trường hợp: kết hợp chuỗi nhân–quả hai bước, kế hoạch hai bước, suy rộng sang tên trường hợp mới, thiếu bằng chứng, đính chính nguồn và phản ví dụ. Câu hỏi giữ riêng không được nạp vào checkpoint. Các probe thay đổi tri thức chạy trên bản sao, không ghi vào ứng viên. Đây là phép đo trong cùng phân phối và cùng dạng bài, không phải benchmark độc lập; người viết giáo trình cũng viết phép đánh giá. Không suy ra hiểu tiếng Việt tự do, chơi game, khả năng tạo ảnh hay lợi nhuận trading từ điểm số này.

Pipeline so sánh trạng thái trước/sau, kiểm tra nạp lại checkpoint, chỉ thay trạng thái ứng dụng khi đủ 36 bản ghi được nhận, đạt tất cả probe và có cải thiện. Không ghi đè dữ liệu người dùng để lấy chỗ: khi giới hạn bộ nhớ ngăn học đủ, ứng viên bị loại. Chạy lại không nhân đôi dữ liệu hoặc báo tăng trí tuệ giả. Không gắn thao tác thiết bị và không thực thi tài chính. Bộ nhớ tích hợp được lưu qua continuity hiện có sau lượt hội thoại.

Gói xuất gồm training.txt, checkpoint.bia-cognition và report.txt. Checkpoint là nhật ký tri thức học được, không phải trọng số mô hình nền tảng. File dành cho API restore của IntegratedCognition; APK không có trình nhập trực tiếp file này. Trên điện thoại dùng lệnh huấn luyện để tái tạo cùng giáo trình và giữ bộ nhớ hiện hữu.

Để tiến tới huấn luyện tổng quát thực sự cần dữ liệu nhiều dạng, bộ đánh giá độc lập chưa dùng để thiết kế, đo khả năng chuyển giao và quên kiến thức trên thiết bị. V144 chỉ tạo bước học có thể lặp lại và kiểm chứng được.
