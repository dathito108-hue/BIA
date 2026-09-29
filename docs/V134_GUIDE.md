# BIA V134 — vòng lặp game trên Android

Bản này nối ảnh màn hình thật → nhận dấu mục tiêu bằng màu → bộ điều khiển native
→ cử chỉ chạm/đa chạm Android. Có thể cấu hình cho màn hình game ngang, không cần
root hay dịch vụ AI. Không được hiểu là BIA đã biết chơi tốt mọi FPS/MOBA.

## Năng lực có trong bản này

- MediaProjection đọc màn hình sau khi bạn đồng ý; xử lý trong RAM, không lưu/gửi ảnh.
- Vùng quan sát và màu mục tiêu do bạn hiệu chỉnh trên ảnh đóng băng.
- Hai ảnh liên tiếp xác nhận dấu mục tiêu trước khi phát hành động.
- FPS: kéo vùng ngắm về dấu mục tiêu, bắn khi dấu nằm gần tâm màn hình.
- MOBA: kéo cần di chuyển theo hướng dấu mục tiêu; đánh và tung một chiêu theo
  khoảng cách trên màn hình, với khoảng nghỉ chiêu 1,5 giây.
- Một cử chỉ có thể chứa nhiều điểm chạm đồng thời: di chuyển + đánh + chiêu.
- Thanh nổi Hiệu chỉnh / Bật / Tạm dừng / DỪNG. Phiên tự chạy tối đa 5 phút hoặc
  1.200 cử chỉ; hết hạn cần bạn bật lại.
- Sân tập tích hợp đếm điểm chạm thật và lần trúng để kiểm tra toàn bộ đường thực thi.

## Dùng trên điện thoại

1. Mở BIA, chọn **Game: quan sát và đa chạm**.
2. Chọn **BIA — sân tập đa chạm**, chế độ **MOBA** để thử lần đầu.
3. Bấm **Bật BIA Game trong Trợ năng**, bật đúng dịch vụ được ghi tên.
4. Trở lại BIA, bấm **Đồng ý đọc màn hình và mở game** và xác nhận hộp thoại Android.
5. Sân tập có sẵn cấu hình; trên thanh nổi bấm **Bật 5 phút**. Quan sát số trúng và
   số chạm đồng thời tăng. Bấm **DỪNG** để kết thúc cả phiên đọc màn hình.
6. Sau khi sân tập hoạt động, chọn game đã cài trong danh sách. Mở chế độ luyện tập
   của game trước khi cấp thao tác. BIA không tự vào trận hoặc điều hướng menu.

Android có thể yêu cầu bạn cho phép cài đặt hạn chế đối với dịch vụ Trợ năng của
APK cài ngoài. Đây là quyền hệ điều hành do bạn quyết định; BIA không tự bật quyền.
Một số game không chấp nhận cử chỉ Trợ năng hoặc chặn ghi hình. BIA không vượt các
cơ chế này; màn hình không có dấu mục tiêu hợp lệ sẽ không tạo hành động.

## Hiệu chỉnh game ngoài

Sau khi vào màn hình luyện tập, trên thanh nổi chọn **Hiệu chỉnh** và chạm bảy điểm:

1. Góc trên trái vùng quan sát mục tiêu.
2. Góc dưới phải vùng đó, tránh HUD/nút và thanh nổi.
3. Dấu màu nổi bật của mục tiêu, ví dụ viền hoặc dấu mục tiêu, tránh màu nền phổ biến.
4. Tâm cần di chuyển.
5. Nút bắn/đánh.
6. Nút chiêu.
7. Vùng trống dùng kéo ngắm.

Ảnh được đóng băng trong bước hiệu chỉnh để lấy đúng màu/pixel. Góc phải của thanh
hướng dẫn là vùng **HỦY**. Hoàn tất chỉ lưu cấu hình; chưa tự chạy. Bấm **Bật** mới
cấp phiên điều khiển cho đúng gói game. Cấu hình gắn với game, FPS/MOBA và kích thước
màn hình; quyền thực thi không được lưu qua khởi động lại.

## Điều kiện dừng

Đổi cửa sổ, rời game, khóa máy, ảnh quá cũ, quá nhiệt, cử chỉ bị hủy/từ chối hoặc
không có kết quả đúng hạn sẽ tạm dừng. Mất quyền đọc màn hình, xoay/đổi kích thước
màn hình hoặc DỪNG kết thúc phiên. Cử chỉ đã bắt đầu có thể hoàn tất trong tối đa
80 ms; không có vòng lặp tự phát lại khi kết quả chưa rõ. Nếu chưa rõ kết quả,
DỪNG rồi mở phiên mới sau khi kiểm tra thiết bị.

## Giới hạn thực tế

Đây là bộ điều khiển phản xạ theo dấu màu đã hiệu chỉnh. Chưa phân biệt mọi địch/đồng
đội, đọc minimap, suy luận chiến thuật, mua đồ, chọn nhiều kỹ năng theo trạng thái,
hiểu menu, né đạn hay chơi trọn trận tự chủ. Tâm quan sát giả định ở giữa màn hình;
điều khiển cần và kéo ngắm là các cử chỉ ngắn, chưa giữ pointer xuyên nhiều cử chỉ.

Giới hạn lấy ảnh danh định là 10 lần/giây, cử chỉ tối đa 80 ms. Đây là cấu hình,
không phải tốc độ đo được trên S21 FE hoặc bảo đảm hiệu năng FPS cạnh tranh.
Nên thử sân tập rồi chế độ luyện tập từng game, và chỉ giữ cấu hình khi hành vi
quan sát được đúng ý bạn. Bằng chứng của sân tập không phải bằng chứng thắng game thương mại.
