use crate::integrated_cognition::IntegratedCognition;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct IntegratedReport {
    pub cases: usize,
    pub language: usize,
    pub revision: usize,
    pub feedback: usize,
    pub planning: usize,
    pub transfer: usize,
    pub continuity: usize,
    pub elapsed: Duration,
}
impl IntegratedReport {
    pub fn passed(&self) -> bool {
        [
            self.language,
            self.revision,
            self.feedback,
            self.planning,
            self.transfer,
            self.continuity,
        ]
        .iter()
        .all(|n| *n == self.cases)
    }
}
pub fn run_integrated_evaluation() -> IntegratedReport {
    let start = Instant::now();
    let mut r = IntegratedReport {
        cases: 32,
        language: 0,
        revision: 0,
        feedback: 0,
        planning: 0,
        transfer: 0,
        continuity: 0,
        elapsed: Duration::ZERO,
    };
    let say = |c: &mut IntegratedCognition, s: &str| c.handle(s).unwrap_or_default();
    let verbs = ["dẫn tới", "kéo theo", "là nguyên nhân của", "gây ra"];
    for i in 0..r.cases {
        let mut c = IntegratedCognition::default();
        let (a, b, d) = (format!("nguon{i}"), format!("giua{i}"), format!("dich{i}"));
        say(&mut c, &format!("Quan sát s1: {a} {} {b}.", verbs[i % 4]));
        if say(&mut c, &format!("Hỏi: Nó có dẫn tới {b} không?")).contains("ủng hộ") {
            r.language += 1;
        }
        say(&mut c, &format!("Nguồn s2: {b} gây ra {d}."));
        let before = say(&mut c, &format!("Hỏi: {a} có gây ra {d} không?"));
        say(&mut c, &format!("Đính chính s2: {b} ngăn {d}."));
        let after = say(&mut c, &format!("Hỏi: {a} có gây ra {d} không?"));
        if before.contains("2 mắt xích") && after.contains("phản đối") {
            r.revision += 1;
        }
        say(&mut c, &format!("Kỹ năng nhanh: {a} -> {d}"));
        let depth = 2 + i % 3;
        let mut prev = a.clone();
        for step in 0..depth {
            let next = if step + 1 == depth {
                d.clone()
            } else {
                format!("buoc{i}x{step}")
            };
            say(&mut c, &format!("Kỹ năng buoc{step}: {prev} -> {next}"));
            prev = next;
        }
        let query = format!("Lập kế hoạch: {a} -> {d}");
        if say(&mut c, &query).contains("nhanh") {
            r.planning += 1;
        }
        let changed = say(&mut c, "Kết quả nhanh: thất bại");
        if changed.contains("buoc0")
            && changed.contains("chưa thực thi")
            && !changed.contains("nhanh")
        {
            r.feedback += 1;
        }
        say(&mut c, &format!("Ví dụ lop{i}: mau1 -> ketqua{i}"));
        let weak = say(&mut c, &format!("Suy rộng: lop{i} cho moi{i}"));
        say(&mut c, &format!("Ví dụ lop{i}: mau2 -> ketqua{i}"));
        let predicted = say(&mut c, &format!("Suy rộng: lop{i} cho moi{i}"));
        say(&mut c, &format!("Phản ví dụ lop{i}: mau3 -> ketqua{i}"));
        let held = say(&mut c, &format!("Suy rộng: lop{i} cho moi{i}"));
        if weak.contains("Chưa đủ") && predicted.contains("Giả thuyết") && held.contains("tạm giữ")
        {
            r.transfer += 1;
        }
        let mut restored = IntegratedCognition::default();
        if restored.restore(&c.export())
            && say(&mut restored, &format!("Hỏi: {a} có gây ra {d} không?")).contains("phản đối")
            && say(&mut restored, &query).contains("buoc0")
            && say(&mut restored, &format!("Suy rộng: lop{i} cho moi{i}")).contains("tạm giữ")
        {
            r.continuity += 1;
        }
    }
    r.elapsed = start.elapsed();
    r
}
