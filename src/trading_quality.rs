//! Sequential, non-overlapping forward observation scoring. No simulated orders or P&L.
#[derive(Debug)]
pub struct Quality {
    pub direction: i8,
    pub samples: usize,
    pub successes: usize,
    pub lower_bound: f64,
    pub volatility_bps: f64,
}
pub fn evaluate(points: &[(u64, f64)], now: u64, cost_bps: f64) -> Result<Quality, &'static str> {
    if points.len() < 21
        || points.len() > 240
        || !cost_bps.is_finite()
        || !(0.0..=10000.0).contains(&cost_bps)
    {
        return Err("Cần 21–240 báo giá thật và ngưỡng chi phí hợp lệ");
    }
    for (i, &(t, p)) in points.iter().enumerate() {
        if t == 0 || t > now.saturating_add(5000) || !p.is_finite() || p <= 0.0 {
            return Err("Dữ liệu không hợp lệ");
        }
        if i > 0 {
            let delta = t
                .checked_sub(points[i - 1].0)
                .ok_or("Timestamp đảo thứ tự")?;
            if !(5000..=60000).contains(&delta) {
                return Err("Chuỗi quan sát thiếu/trùng; cần chuỗi liên tục mới");
            }
        }
    }
    if now.saturating_sub(points.last().unwrap().0) > 45000 {
        return Err("Báo giá đã cũ");
    }
    let classify = |i: usize| -> (i8, f64) {
        let w = &points[i - 5..=i];
        let moves: Vec<f64> = w
            .windows(2)
            .map(|x| (x[1].1 / x[0].1 - 1.0) * 10000.0)
            .collect();
        let mean = moves.iter().sum::<f64>() / moves.len() as f64;
        let vol =
            (moves.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / moves.len() as f64).sqrt();
        let momentum = (w[5].1 / w[0].1 - 1.0) * 10000.0;
        let hurdle = cost_bps + 2.0 * vol;
        let d = if momentum > hurdle {
            1
        } else if momentum < -hurdle {
            -1
        } else {
            0
        };
        (d, vol)
    };
    let mut n = 0;
    let mut wins = 0;
    // Every score uses an earlier signal and a later observed quote; horizons do not overlap.
    for i in (5..points.len() - 5).step_by(5) {
        let (d, _) = classify(i);
        if d != 0 {
            n += 1;
            let forward = (points[i + 5].1 / points[i].1 - 1.0) * 10000.0 * f64::from(d);
            if forward > cost_bps {
                wins += 1;
            }
        }
    }
    let lower = if n == 0 {
        0.0
    } else {
        let count = n as f64;
        let p = wins as f64 / count;
        let z2 = 1.96_f64.powi(2);
        (p + z2 / (2.0 * count) - 1.96 * ((p * (1.0 - p) + z2 / (4.0 * count)) / count).sqrt())
            / (1.0 + z2 / count)
    };
    let (direction, volatility_bps) = classify(points.len() - 1);
    Ok(Quality {
        direction,
        samples: n,
        successes: wins,
        lower_bound: lower,
        volatility_bps,
    })
}
pub fn text(points: &[(u64, f64)], now: u64, cost_bps: f64) -> String {
    match evaluate(points, now, cost_bps) {
        Err(e) => format!("CHƯA ĐỦ BẰNG CHỨNG: {e}"),
        Ok(q) => format!("Quan sát giá thật: {} | biến động {:.2} bps\nTín hiệu đã đủ thời gian đối chiếu: {} • vượt ngưỡng chi phí: {}\nCận dưới Wilson 95%: {:.1}% • ngưỡng chi phí {:.1} bps\nĐây là đo chuyển động giá, không phải khớp lệnh/P&L. Chưa chứng minh lợi thế hoặc cấp quyền giao dịch.", match q.direction { 1 => "TĂNG", -1 => "GIẢM", _ => "CHỜ" },q.volatility_bps,q.samples,q.successes,q.lower_bound*100.0,cost_bps),
    }
}
