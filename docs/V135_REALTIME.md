# BIA V135 — trading realtime, dữ liệu thật

Theo yêu cầu chuyển sang realtime, V135 không có môi trường mô phỏng, backtest,
khớp lệnh giả hoặc dữ liệu demo trong sản phẩm. Đây là bước **thu nhận dữ liệu thật
và phân tích kỹ thuật native**, chưa phải hệ thống đặt lệnh tiền thật hay bằng chứng
khả năng kiếm tiền. Giữ nguyên kiến trúc BIA; không thêm model hoặc dịch vụ AI.

## Cách dùng

BIA → **Trading: dữ liệu realtime** → chọn nguồn → nhập 1–3 mã → **Kết nối dữ liệu thật**.

- **Binance:** crypto spot, ví dụ `BTCUSDT,ETHUSDT`. Không cần API key. Sử dụng
  `data-api.binance.vision` và `data-stream.binance.vision`, chỉ dữ liệu công khai.
- **Twelve Data:** Forex, cổ phiếu, ETF, crypto, ví dụ `EUR/USD,AAPL,BTC/USD`.
  Cần API key DỮ LIỆU của bạn và quyền WebSocket/REST cho từng mã/thị trường.
  Gói dữ liệu, sàn, phiên giao dịch có thể giới hạn hoặc làm trễ giá. Không nhập
  API key sàn giao dịch, broker hay key có quyền đặt lệnh. Chưa tích hợp MT5,
  Exness/XAUUSDc, futures, options hoặc xác nhận bao phủ mọi sàn.

Key chỉ giữ trong phiên, không lưu vào file, backup, log hoặc trạng thái Android;
trường key bị xóa khi rời màn hình. Mã chứng khoán và key Twelve Data được gửi
đến đúng nhà cung cấp dữ liệu đó qua TLS. Không chuyển đến dịch vụ AI.
Không nhận endpoint tùy ý; không theo HTTP redirect. Lỗi không hiển thị URL chứa key.

**Dừng kết nối** đóng WebSocket/REST đang chạy và vô hiệu hóa phân tích. Rời màn hình,
khóa máy hoặc chuyển app cũng dừng phiên. Chưa chạy nền 24/7, chưa tự kết nối lại;
bấm Kết nối để mở phiên mới và lấy lại nến. Tối đa 3 mã, một WebSocket cho nguồn.

## Phân tích và tính mới của dữ liệu

- Giá WebSocket có timestamp từ nguồn; UI cập nhật mỗi giây, không phải cam kết
  độ trễ 1 giây. Hiển thị tuổi giá và giờ UTC nguồn. Không thay timestamp cũ bằng
  giờ nhận mới để giả vờ realtime.
- Binance: aggregate trades + nến `1m` đã đóng từ luồng. REST seed tối đa 60 nến.
- Twelve Data: luồng giá + REST nến 1 phút khoảng một lần/phút. Không tạo OHLC giả
  từ luồng tick thưa; nến nguồn phải có UTC và đã đóng.
- Native BIA tính EMA5/EMA20, ATR14, khoảng lệch giá/ATR; xuất xu hướng tăng/giảm/
  trung tính. Đây là quy tắc phân tích kỹ thuật, không phải trí tuệ trading đã huấn
  luyện hoặc khuyến nghị mua/bán.
- Chặn khi mất kết nối, giá cũ hơn 15 giây, timestamp tương lai >5 giây, nến cuối
  cũ hơn 90 giây, thiếu 21 nến liên tục, số/OHLC sai hoặc giá lệch hơn 3 ATR.
  Bộ đếm monotonic cũng chặn khi không nhận giá mới trong 15 giây.
- Thị trường ít giao dịch/đóng cửa có thể hiển thị giá cuối và CHẶN, không tự tạo giá.
- Chỉ đọc: không có code gọi endpoint account/order, không giữ tiền, không mở/đóng
  vị thế, không tính doanh thu từ dữ liệu hay tín hiệu.

## Kiểm chứng

- 9 native tests cho freshness, timestamp, OHLC, gap, nến đã đóng, xu hướng và jump.
- Android `TradingLiveTest`: một test bắt buộc đọc **REST + WebSocket công khai thật**
  cho BTCUSDT, kiểm tra JNI và nút Dừng; lỗi mạng làm test thất bại, không fallback.
- Test bộ giải mã Binance/Twelve Data dùng fixture chỉ trong mã kiểm thử, không
  được đóng gói vào APK chính; không phải kiểm chứng kết nối Twelve Data có key.
- Bài kiểm thử game V134 vẫn chạy để kiểm tra hồi quy. Một emulator cho cả bộ test.
- Chưa đo S21 FE/Android 16; chưa xác nhận API key Twelve Data hoặc lợi nhuận.

## Tài liệu giao thức

https://github.com/binance/binance-spot-api-docs/blob/master/faqs/market_data_only.md
https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams
https://support.twelvedata.com/en/articles/5620516-how-to-stream-the-data
https://support.twelvedata.com/en/articles/5745849-timezones
