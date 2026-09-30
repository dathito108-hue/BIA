# V143 — kiểm kê và kết nối năng lực

Một `OfflineMobileBia` trong `RUNTIME` là runtime điều phối duy nhất. Bộ điều khiển game trước đây nằm trong `GAME_AGENT` riêng; nay là trường `game` của runtime. Không thêm mô hình nền tảng hoặc lõi suy luận khác.

| Nhóm | Điểm thực thi / quyền sở hữu | Giới hạn |
|---|---|---|
| Nhận thức, hội thoại, tri thức, suy luận, mục tiêu | Các trường hiện có trong OfflineMobileBia | Không chứng minh trí tuệ tổng quát |
| Thao tác thiết bị | SkillExecution + ExecutionAuthority hiện có | Quyền Android và phê duyệt vẫn bắt buộc |
| Chơi game | GameAgent thuộc OfflineMobileBia; capture/accessibility là adapter OS | Phản xạ màu/ROI đã hiệu chỉnh, chưa chứng minh chơi FPS/MOBA tổng quát |
| Tạo sản phẩm | ProductBundle.compile → CoreSkills → bộ biên dịch Rust → kiểm tra gói | Bộ sinh mẫu có phạm vi xác định, chưa có doanh thu được kiểm chứng |
| Tạo ảnh thủ tục | CreativeEngine.render → CoreSkills | Không phải ảnh chân thực từ mô tả bất kỳ |
| Dựng cảnh | SceneEngine.render → CoreSkills | CPU rasterizer và primitive giới hạn |
| Điêu khắc | Sculpt.apply → CoreSkills | Cọ/subdivision có giới hạn |
| Hoạt hình 3D | GlbWriter.write → CoreSkills | TRS keyframe, chưa rig/skeleton |
| Học ảnh và sinh ảnh học | ImageLearning.train / Model.generate → CoreSkills | Atlas nhân tố Java 64×64, checkpoint tài nguyên riêng; chưa chuyển toán học/trọng số vào Rust |
| Thị trường | TradingNative.analyze/quality → CoreSkills → Rust | Kết quả hàm có thể là chặn/chờ dữ liệu, không có nghĩa tín hiệu đạt chất lượng |
| EVM DEX | DexFeed.rpc, DexTransaction.prepare → CoreSkills | RPC danh sách cho phép, draft unsigned, không gửi tiền |
| Solana | SolanaOrder.fetch → CoreSkills | Quote/unsigned preparation; ký, kiểm tra ví, hạn mức, broadcast và đối soát giữ quy trình hiện hữu |
| Giọng nói, tài liệu, chia sẻ, tệp | Adapter Android đi vào hội thoại/ingest/hàng đợi hiện hữu | Nhận giọng nói/TTS do dịch vụ OS; chọn/xuất tệp do người dùng |

## Ý nghĩa kết nối

Các executor có ticket được cấp trước khi chạy và trả trạng thái trong `finally`. Core từ chối tên ngoài danh sách và tối đa 32 tác vụ đồng thời. Không giữ mutex lõi trong lúc render hoặc gọi mạng. Lưu tối đa 32 biên nhận, không ghi API key, địa chỉ ví, prompt hoặc payload vào biên nhận. Thành công nghĩa hàm đã trả về; không chứng minh chất lượng ảnh, tín hiệu giao dịch, giao dịch thành công hay việc bán sản phẩm.

Biên nhận và counter được nối vào continuity hiện có; tác vụ đang chạy khi lưu sẽ khôi phục thành gián đoạn, không tự chạy lại. Import bị chặn khi có executor đang chạy. Nhập snapshot cũ trong cùng runtime không lùi counter. Game reset khi khôi phục vì frame cũ không còn hợp lệ. Continuity dùng lịch lưu hiện hữu của MainActivity, không phải journal crash-durable cho từng tác vụ.

Gõ `kiểm tra năng lực` hoặc `bia skills` trong hội thoại để xem registry và kết quả gần đây. Đây là cơ chế admission/feedback của công cụ; chưa có bộ lập kế hoạch tự chọn toàn bộ công cụ theo ngôn ngữ tự do. Việc mở một activity không được tính là thực thi thành công.

## Quyền tài chính

Registry không có `solana.sign`, `dex.broadcast` hoặc quyền mở khóa ví. Ticket không thay thế phê duyệt ví, hạn mức, kiểm tra freshness, mainnet, preflight hoặc đối soát. Mã ký/broadcast không được thay đổi trong bản này.

## Xác minh

Thêm Rust tests cho ticket, từ chối quyền ngoài danh sách, completion trùng, giới hạn đồng thời, restore bị gián đoạn. Android integration tests chạy renderer thật rồi kiểm tra receipt trong runtime và kiểm tra quyền ví không được cấp. Bộ test hiện hữu tiếp tục chạy. Các kết quả CI phải được xác nhận trước phát hành; không suy ra đạt test chỉ từ tài liệu này.
