# V132 — thực thi kỹ năng trên Android

BIA 1.32.0 nối kỹ năng đã khai báo với năm thao tác Android: mở cài đặt,
mở URL HTTP(S), tìm web, mở ứng dụng bằng tên gói và ghi clipboard.
Lõi vẫn chạy native, không thêm mô hình hay dịch vụ AI bên ngoài.

## Thử một kỹ năng

Gửi lần lượt trong chat:

```text
Kỹ năng lưu ghi chú: sẵn sàng -> đã sao chép
Gắn thao tác lưu ghi chú: Sao chép Xin chào BIA
Chạy kỹ năng lưu ghi chú
```

BIA hiển thị tên và nội dung chính xác trong hộp thoại. Chọn **Thực thi**
để chạy bước đó, hoặc **Dừng** để hủy hàng đợi. Clipboard được đọc lại
để kiểm tra nội dung sau khi ghi. Dừng không bị học thành kỹ năng thất bại.

## Chạy chuỗi

```text
Kỹ năng mở cài đặt: đã sao chép -> đã mở cài đặt
Gắn thao tác mở cài đặt: Mở cài đặt
Chạy kế hoạch: sẵn sàng -> đã mở cài đặt
```

BIA tìm chuỗi tối đa bốn bước trong các kỹ năng có thao tác đã gắn và chưa
bị chặn bởi thất bại. Mỗi bước cần duyệt riêng; lỗi dừng phần còn lại.
Các điều kiện và kết quả trong khai báo là mô tả của bạn, không phải cảm biến
xác nhận trạng thái thực tế. Bạn cần kiểm tra chúng khi duyệt bước.

Các kiểu thao tác khác:

```text
Gắn thao tác tra cứu: Tìm web thời tiết Hà Nội
Gắn thao tác đọc trang: Mở https://example.com
Gắn thao tác mở ứng dụng: Mở ứng dụng com.example.app
```

Phải khai báo kỹ năng tương ứng trước khi gắn. Mở ứng dụng phụ thuộc ứng dụng
đã cài và khả năng hiển thị gói của Android.

## Theo dõi và khôi phục

- `Trạng thái thực thi`: số bước chờ và biên nhận, hoặc thông báo kết quả chưa rõ.
- `Hủy thực thi`: bỏ hàng đợi, giải phóng lượt chưa rõ sau khi bạn kiểm tra thiết bị.
  Không hoàn tác tác động đã xảy ra.
- `Kết quả tên kỹ năng: thành công`: phản hồi do bạn xác nhận sau khi kiểm tra,
  có thể mở lại kỹ năng bị chặn nếu nhật ký còn chỗ.
- `Lập kế hoạch: ...` chỉ mô phỏng; `Chạy kế hoạch: ...` mới chuẩn bị thao tác.

Trước mỗi tác động, Android lưu và đọc kiểm tra trạng thái đã nhận bước.
Nếu app bị ngắt trước khi lưu kết quả, bước đó giữ trạng thái chưa rõ khi mở lại;
không tự chạy lại. Đây là cách tránh phát lại tự động, không phải bảo đảm
“đúng một lần” trong mọi sự cố thiết bị. Hủy rồi chạy mới là yêu cầu thực thi mới.

## Giới hạn

Mở URL, tìm web, mở cài đặt và ứng dụng chỉ xác nhận Android tiếp nhận yêu cầu;
không chứng minh tác vụ trong ứng dụng đã xong. BIA không tự bấm giao diện ứng dụng
khác, đăng nhập, thanh toán hoặc chạy mã tùy ý. Không chạy nền không cần duyệt.

Giữ tối đa 32 biên nhận, 32 kỹ năng và 128 thay đổi nhật ký người dùng. Lỗi adapter
có vùng lưu riêng theo kỹ năng để vẫn chặn chạy lại khi nhật ký đã đầy.
Nội dung tài liệu/Share không được diễn giải thành lệnh thực thi kỹ năng.
