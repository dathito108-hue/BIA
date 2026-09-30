# V155 — Intent Fusion & Natural Response Planning

V155 hợp nhất nhiều yêu cầu giao tiếp thành một kế hoạch trả lời duy nhất.

## Hỗ trợ

- giải thích;
- so sánh với trường hợp trước;
- tóm tắt;
- kết luận tham chiếu theo trường hợp trước;
- tập trung vào một subject/object của câu hỏi hiện tại;
- loại bỏ một section cụ thể như phần so sánh / giải thích / tóm tắt.

## Nguyên tắc

Trọng tâm chỉ hợp lệ nếu khớp subject hoặc object của semantic query hiện tại. Nếu không khớp, BIA hỏi lại thay vì gán ghép.

Các section được sinh lại từ reasoning hiện có. Intent fusion không tạo factual edge, không sửa nguồn và không tạo DeviceAction.

Luồng:

`input -> IntentFusionPlan -> semantic focus validation -> grounded section generation -> fused response`
