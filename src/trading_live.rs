//! Native analysis of timestamped live feed data. No simulated fills or order authority.
#[derive(Debug, Clone, Copy)]
pub struct Candle {
    pub close_ms: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
#[derive(Debug, PartialEq)]
pub enum State {
    Up,
    Down,
    Neutral,
    Blocked(&'static str),
}
#[derive(Debug)]
pub struct Analysis {
    pub state: State,
    pub fast: f64,
    pub slow: f64,
    pub atr: f64,
    pub distance_atr: f64,
}
fn blocked(reason: &'static str) -> Analysis {
    Analysis {
        state: State::Blocked(reason),
        fast: 0.0,
        slow: 0.0,
        atr: 0.0,
        distance_atr: 0.0,
    }
}
pub fn analyze(
    bars: &[Candle],
    price: f64,
    event_ms: u64,
    now_ms: u64,
    connected: bool,
) -> Analysis {
    if !connected {
        return blocked("Mất kết nối — tín hiệu vô hiệu");
    }
    if !price.is_finite() || price <= 0.0 || event_ms == 0 {
        return blocked("Chưa có giá hợp lệ từ nguồn");
    }
    if event_ms > now_ms.saturating_add(5000) {
        return blocked("Sai lệch đồng hồ / timestamp tương lai");
    }
    if now_ms.saturating_sub(event_ms) > 15000 {
        return blocked("Giá quá 15 giây — có thể trễ hoặc thị trường nghỉ");
    }
    if bars.len() < 21 || bars.len() > 120 {
        return blocked("Chưa đủ 21 nến 1 phút đã đóng");
    }
    for (i, b) in bars.iter().enumerate() {
        if b.close_ms == 0
            || b.close_ms > now_ms
            || [b.open, b.high, b.low, b.close]
                .iter()
                .any(|x| !x.is_finite() || *x <= 0.0)
            || b.low > b.open.min(b.close)
            || b.high < b.open.max(b.close)
        {
            return blocked("Nến không hợp lệ / chưa đóng");
        }
        if i > 0 && b.close_ms.checked_sub(bars[i - 1].close_ms) != Some(60000) {
            return blocked("Dữ liệu nến bị thiếu / trùng / đảo thứ tự");
        }
    }
    let last = bars.last().unwrap();
    if now_ms.saturating_sub(last.close_ms) > 90000 {
        return blocked("Nến quá cũ — chưa đủ dữ liệu mới");
    }
    let mut fast = bars[0].close;
    let mut slow = fast;
    for b in &bars[1..] {
        fast += (b.close - fast) * 2.0 / 6.0;
        slow += (b.close - slow) * 2.0 / 21.0;
    }
    let tail = &bars[bars.len() - 15..];
    let atr = tail
        .windows(2)
        .map(|w| {
            (w[1].high - w[1].low)
                .max((w[1].high - w[0].close).abs())
                .max((w[1].low - w[0].close).abs())
        })
        .sum::<f64>()
        / 14.0;
    if !atr.is_finite() || atr <= 0.0 || !fast.is_finite() || !slow.is_finite() {
        return blocked("Không đủ biến động hợp lệ");
    }
    let distance_atr = (price - last.close).abs() / atr;
    if distance_atr > 3.0 {
        return blocked("Giá lệch quá 3 ATR — cần chờ nến mới");
    }
    let state = if fast - slow > atr * 0.15 && price > slow {
        State::Up
    } else if slow - fast > atr * 0.15 && price < slow {
        State::Down
    } else {
        State::Neutral
    };
    Analysis {
        state,
        fast,
        slow,
        atr,
        distance_atr,
    }
}
impl Analysis {
    pub fn text(&self) -> String {
        let label = match self.state {
            State::Up => "XU HƯỚNG TĂNG",
            State::Down => "XU HƯỚNG GIẢM",
            State::Neutral => "TRUNG TÍNH",
            State::Blocked(reason) => return format!("CHẶN PHÂN TÍCH: {reason}"),
        };
        format!("{}\nEMA5 {:.8} | EMA20 {:.8}\nATR14 {:.8} | lệch giá {:.2} ATR\nQuy tắc kỹ thuật; chưa kiểm chứng lợi thế, không phải lệnh mua/bán.",label,self.fast,self.slow,self.atr,self.distance_atr)
    }
}
