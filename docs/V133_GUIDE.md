# BIA V133 — chuỗi liên tục và tự duyệt có phạm vi

BIA 1.33.0 cho phép duyệt cả chuỗi một lần, hoặc cấp trước quyền tự duyệt
đúng thao tác đã xem. Bạn bật quyền trên điện thoại trong hộp thoại; mặc định
vẫn chỉ duyệt một bước.

## Ba chế độ

| Chế độ | Phạm vi |
| --- | --- |
| Chỉ bước đầu | Một ID thao tác hiện tại |
| Cả chuỗi hiện tại | Tối đa 12 ID thao tác trong hàng đợi đã xem |
| Tự duyệt thao tác đã liệt kê | Đúng loại thao tác và nội dung đã xem, kể cả yêu cầu mới; tối đa 100 lần bắt đầu trong 30 phút |

Quyền tự duyệt so khớp nguyên văn nội dung clipboard, URL, câu tìm kiếm hoặc tên
gói ứng dụng. Đổi nội dung hoặc loại thao tác thì cần duyệt lại. Không cấp quyền
theo tên miền, mẫu ký tự hay mọi ứng dụng. Hàng đợi thay đổi trong lúc xem hộp thoại
thì thao tác cấp quyền bị từ chối để bạn xem lại. Cấp quyền mới thay thế quyền cũ.

## Thử ngay

Gửi lần lượt:

```text
Kỹ năng ghi a: sẵn sàng -> đã ghi a
Gắn thao tác ghi a: Sao chép Nội dung A
Kỹ năng ghi b: đã ghi a -> đã ghi b
Gắn thao tác ghi b: Sao chép Nội dung B
Chạy chuỗi: ghi a -> ghi b -> ghi a
```

Trong hộp thoại, chọn **Cả chuỗi hiện tại** rồi **Cấp quyền và chạy**.
Ba lần ghi clipboard chạy liên tiếp; mỗi lần đọc lại để kiểm tra nội dung.
Chỉ nội dung cuối được giữ trong clipboard.

Chọn chế độ **Tự duyệt thao tác đã liệt kê** nếu muốn các lệnh tiếp theo sử dụng
chính những thao tác đó không hỏi lại trong hạn mức. Đây là quyền cho phép chạy,
không tự tạo công việc vô hạn. Ví dụ gửi tiếp:

```text
Lặp kỹ năng ghi a: 8
```

`Chạy chuỗi: tên 1 -> tên 2 -> ...` và `Lặp kỹ năng tên: số lần` có giới hạn
1–12 bước mỗi yêu cầu. Toàn bộ kỹ năng phải tồn tại, có thao tác đã gắn và chưa
bị chặn. Nếu một tên sai thì không xếp một phần chuỗi.

`Chạy kế hoạch: trạng thái đầu -> mục tiêu` vẫn dùng bộ lập kế hoạch giới hạn
4 bước. Chuỗi chỉ định là lệnh trực tiếp của bạn; điều kiện của kỹ năng chưa
được tự quan sát trên thiết bị.

## Dừng và tiếp tục

- Nút **Dừng chuỗi / Tắt tự duyệt** dừng các bước chưa chạy và thu hồi quyền.
- `Dừng tự động` hoặc `Tắt tự duyệt` có cùng tác dụng.
- `Trạng thái thực thi` cho biết hàng đợi, biên nhận và quyền còn lại.
- `Hủy thực thi` dùng sau khi bạn đã kiểm tra bước có kết quả chưa rõ;
  không hoàn tác tác động đã xảy ra.

BIA nhường luồng giao diện giữa các bước. Chạy clipboard liên tiếp khi BIA ở
foreground. Khi mở URL, tìm web, cài đặt hoặc ứng dụng khác, chuỗi tạm chờ;
quay lại BIA thì tiếp tục nếu quyền vẫn còn. Rời app dừng lịch chạy đến khi quay lại.
Không tự điều khiển giao diện bên trong ứng dụng khác.

Lỗi adapter dừng chuỗi, thu hồi quyền và chặn kỹ năng lỗi. Lỗi lưu trạng thái
cũng tắt tự duyệt. Bước đã nhận nhưng chưa rõ kết quả sau gián đoạn không được
chạy lại tự động. Quyền duyệt không được lưu qua việc tạo lại Activity hoặc
khởi động lại tiến trình; hàng đợi và biên nhận vẫn được lưu.

## Phạm vi năng lực

Hỗ trợ năm adapter: clipboard, cài đặt, URL HTTP(S), tìm web, mở tên gói ứng dụng.
Mở ứng dụng chỉ xác nhận Android tiếp nhận, không xác nhận tác vụ bên trong đã xong.
Đây chưa phải điều khiển chạm toàn thiết bị, đa chạm chơi game hay chạy nền 24/7.
Lõi BIA native được giữ nguyên, không thêm mô hình AI khác.
