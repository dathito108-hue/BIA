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
  API key sàn giao dịch, broker hay key có quyền đặt lệnh. Mục DEX riêng hỗ trợ V2 trên Ethereum/BNB Chain. Chưa tích hợp MT5,
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


## DEX on-chain và chuẩn bị giao dịch

BIA → **DEX: pool thật và chuẩn bị swap**. Chọn Ethereum/Uniswap V2 hoặc BNB
Chain/PancakeSwap V2; nhập ĐỊA CHỈ POOL V2. Các kiểu V3/V4, Solana, stable-swap,
bridge, derivatives và router khác chưa được hỗ trợ. Địa chỉ mặc định là pool
USDC/WETH trên Ethereum; không dùng địa chỉ đó trên BNB Chain.

Adapter chỉ gọi RPC đọc qua PublicNode. Cứ 15 giây sau lần đọc trước nó lấy chainId,
block, factory, token0/1, getPair, decimals, reserves và balanceOf ở CÙNG block,
đọc lại block hash để phát hiện thay đổi trong lần thu nhận. Đây là cập nhật
on-chain bằng polling, không phải tick-level/mempool feed. Block head chưa final;
RPC là nguồn phải tin cậy, không phải bằng chứng light-client độc lập.

Báo giá một pool dùng số nguyên BigInteger theo công thức V2, phí LP 30 bps
(Uniswap) hoặc 25 bps (PancakeSwap). Tính lượng ra và minOut theo slippage 0–100 bps.
Chặn nếu snapshot >60 giây, reserves không cập nhật 5 phút, số dư khác reserves,
input >1% reserve đầu vào, output bằng 0, chain/factory/getPair sai. Theo dõi biến
động spot qua 6 block quan sát; giảm >20% thanh khoản hình học giữa hai snapshot
sẽ khóa phiên đến khi người dùng dừng/kiểm tra/mở lại. Chưa định giá liquidity USD,
chưa dự báo lợi nhuận, chưa có chiến lược DEX tự chủ hoặc định tuyến nhiều pool.

**Kiểm tra & tạo swap CHƯA KÝ** cần địa chỉ ví CÔNG KHAI. Adapter kiểm tra balance,
allowance rồi dùng eth_call và eth_estimateGas để kiểm tra giao dịch đọc-only;
không approve và không broadcast. Đây là preflight EVM của giao dịch dự kiến,
không phải môi trường giao dịch mô phỏng. Nếu ví chưa đủ allowance, dừng và báo lý do.

Bản JSON chứa chainId, router, recipient bằng chính địa chỉ from, amount, minOut,
deadline 120 giây, gas ước tính +20% và calldata swapExactTokensForTokens. Snapshot
cho bước tạo phải mới trong 30 giây. Bản nháp vô hiệu trong app khi dừng/rời màn hình;
copy từ chối nếu form thay đổi, hết deadline hoặc báo giá đang bị chặn. Bản đã copy
ra ngoài vẫn cần ví kiểm tra và chịu deadline trong calldata.

CHƯA tích hợp kết nối/ký bằng ví, gửi giao dịch, theo dõi receipt hay xác nhận khớp.
JSON chưa ký không tự giao dịch được; không coi việc tạo nó là đã đặt lệnh.
Không có seed/private key, eth_sendTransaction, eth_sendRawTransaction hoặc approve.
Không xác minh được honeypot, token tax, blacklist/owner upgrade, MEV hay khả năng
bán trong tương lai. Preflight thành công không bảo đảm giao dịch thực tế thành công.

Kiểm thử thêm: số nguyên/decimals/minOut/calldata, từ chối địa chỉ ví sai, đọc
pool thật trên Ethereum và BNB Chain. Không kiểm thử ký/giao dịch tiền thật.

Nguồn giao thức/factory:
https://developers.uniswap.org/docs/protocols/v2/deployments
https://developer.pancakeswap.finance/contracts/v2/addresses
https://docs.pancakeswap.finance/earn/pancakeswap-pools
https://ethereum.publicnode.com/
