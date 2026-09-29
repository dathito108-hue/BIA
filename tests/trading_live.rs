use bia_core::trading_live::*;
fn bars() -> Vec<Candle> {
    (0..40)
        .map(|i| {
            let p = 100.0 + i as f64 * 0.1;
            Candle {
                close_ms: 1_800_000_000_000 + i * 60000,
                open: p,
                high: p + 0.2,
                low: p - 0.2,
                close: p + 0.1,
            }
        })
        .collect()
}
fn result(b: &[Candle], p: f64, t: u64, n: u64, c: bool) -> Analysis {
    analyze(b, p, t, n, c)
}
#[test]
fn trend_uses_closed_bars() {
    let b = bars();
    let n = b.last().unwrap().close_ms + 1000;
    assert_eq!(result(&b, 104.0, n, n, true).state, State::Up);
}
#[test]
fn disconnect_invalidates() {
    let b = bars();
    let n = b.last().unwrap().close_ms + 1000;
    assert!(matches!(
        result(&b, 104.0, n, n, false).state,
        State::Blocked(_)
    ));
}
#[test]
fn stale_and_future_quotes_block() {
    let b = bars();
    let n = b.last().unwrap().close_ms + 20000;
    for t in [n - 15001, n + 5001, 0] {
        assert!(matches!(
            result(&b, 104.0, t, n, true).state,
            State::Blocked(_)
        ));
    }
}
#[test]
fn invalid_numeric_prices_block() {
    let b = bars();
    let n = b.last().unwrap().close_ms + 1000;
    for p in [f64::NAN, f64::INFINITY, 0.0, -1.0] {
        assert!(matches!(result(&b, p, n, n, true).state, State::Blocked(_)));
    }
}
#[test]
fn missing_and_duplicate_minutes_block() {
    for time_delta in [-60000, 60000] {
        let mut b = bars();
        b[15].close_ms = (b[15].close_ms as i64 + time_delta) as u64;
        let n = b.last().unwrap().close_ms + 1000;
        assert!(matches!(
            result(&b, 104.0, n, n, true).state,
            State::Blocked(_)
        ));
    }
}
#[test]
fn future_candles_and_old_seed_block() {
    let b = bars();
    let n = b.last().unwrap().close_ms;
    for now in [n - 1, n + 90001] {
        assert!(matches!(
            result(&b, 104.0, now, now, true).state,
            State::Blocked(_)
        ));
    }
}
#[test]
fn invalid_ohlc_and_warmup_block() {
    let mut b = bars();
    let n = b.last().unwrap().close_ms + 1000;
    b[4].low = b[4].high + 1.0;
    assert!(matches!(
        result(&b, 104.0, n, n, true).state,
        State::Blocked(_)
    ));
    assert!(matches!(
        result(&b[..20], 104.0, n, n, true).state,
        State::Blocked(_)
    ));
}
#[test]
fn jump_blocks_until_new_candles() {
    let b = bars();
    let n = b.last().unwrap().close_ms + 1000;
    assert!(matches!(
        result(&b, 120.0, n, n, true).state,
        State::Blocked(_)
    ));
}
#[test]
fn falling_series_is_down() {
    let mut b = bars();
    for v in &mut b {
        v.open = 210.0 - v.open;
        v.close = 210.0 - v.close;
        let low = 210.0 - v.high;
        v.high = 210.0 - v.low;
        v.low = low;
    }
    let n = b.last().unwrap().close_ms + 1000;
    assert_eq!(result(&b, 106.0, n, n, true).state, State::Down);
}
