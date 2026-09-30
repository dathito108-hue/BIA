# V147 — giao tiếp tự nhiên giữ nguyên vai nghĩa

V147 mở rộng trực tiếp ngôn ngữ của lõi BIA, không thêm LLM, Transformer, SSM, Mamba hay một lõi AI thứ hai.

## Nâng cấp

- Hiểu thêm cách hỏi đời thường: `A có phải là nguyên nhân của B không?`.
- Hiểu dạng đảo vai ngôn ngữ nhưng giữ đúng nghĩa: `B có phải là do A không?` được chuẩn hóa thành quan hệ nguyên nhân `A -> B`.
- Chấp nhận thêm tiền tố lịch sự như `cho mình hỏi`, `mình muốn hỏi`, `theo bạn`.
- Xử lý một số hậu tố hội thoại như `nhỉ`, `nhé`, `đúng không`, `phải không`.
- Mở rộng yêu cầu nối lượt: `thế còn nguyên nhân X thì sao` và `thế còn kết quả Y thì sao`.
- Các câu kiểu `thế còn X thì sao` vẫn phải làm rõ vai, BIA không tự đoán.

## Nguyên tắc kiến trúc

Ngôn ngữ chỉ tạo meaning frame hoặc semantic query cho IntegratedCognition. Nó không tạo DeviceAction và không thay authority gate. Mọi câu được chuẩn hóa theo vai nguyên nhân/kết quả trước khi reasoning; câu mơ hồ không được tự suy diễn thành quan hệ mới.

V147 vì vậy tăng độ tự nhiên của giao tiếp nhưng vẫn là BIA-native structured meaning + causal reasoning, không giả lập hội thoại tự do bằng mô hình ngôn ngữ bên ngoài.
