# V156 — Natural Surface Realization

V156 tiếp tục bộ sinh giao tiếp của BIA theo kiến trúc riêng, không thêm LLM/Transformer/SSM/Mamba hay lõi AI thứ hai.

## Mục tiêu

Tách **nội dung cần nói** khỏi **cách nói**:

`reasoning + DuyenWeave + discourse intent -> grounded thought -> NaturalSurfaceRealizer -> lời đáp`

Lớp hiện thực hóa bề mặt chỉ dùng tín hiệu đã có trong BIA:

- subject / object của câu hỏi hiện tại;
- mức diễn đạt Brief / Standard / Deep;
- hình học Duyên: số nhánh, hội tụ, chồng lấp, phản chiều;
- kế hoạch hội thoại đã được V154/V155 xác định.

Nó không tạo factual edge, không tự thêm nguồn, không thay verdict và không tạo DeviceAction.

## Thay đổi chính

- bỏ tiền tố cố định `Về quan hệ giữa ...` ở đường trả lời tích hợp;
- mở câu theo hình học Duyên và độ sâu diễn đạt;
- ghép explain / compare / summary / previous-reference thành văn xuôi có chuyển ý;
- giữ câu trả lời Brief trực tiếp, không thêm lớp dẫn nhập;
- Android instrumentation được tách trạng thái hội thoại giữa các test để tránh nhiễu nguồn từ test trước.

## Giới hạn còn lại

V156 vẫn là bộ hiện thực hóa có ràng buộc và có thể kiểm chứng. Nó làm lời nói tự nhiên hơn nhưng chưa đồng nghĩa với khả năng sinh ngôn ngữ tự do không giới hạn.
