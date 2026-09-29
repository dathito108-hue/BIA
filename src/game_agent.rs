//! Calibrated visual reflex controller, not a general game-playing model.
//! Pixels and coordinates stay local. No OS calls, rewards or win claims here.
#[derive(Clone, Debug)]
pub struct GameProfile {
    pub roi: [f32; 4],
    pub rgb: [u8; 3],
    pub tolerance: u8,
    pub moba: bool,
}
impl GameProfile {
    pub fn valid(&self) -> bool {
        self.roi
            .iter()
            .all(|x| x.is_finite() && (0.0..=1.0).contains(x))
            && self.roi[2] - self.roi[0] >= 0.05
            && self.roi[3] - self.roi[1] >= 0.05
            && (5..=80).contains(&self.tolerance)
    }
}
#[derive(Clone, Debug, Default)]
pub struct GameDecision {
    pub target: Option<[f32; 2]>,
    pub movement: [f32; 2],
    pub aim: [f32; 2],
    pub fire: bool,
    pub skill: bool,
    pub confirmed: bool,
}
#[derive(Default)]
pub struct GameAgent {
    previous: Option<[f32; 2]>,
    last_frame: Option<u64>,
    stable: u8,
    last_skill: Option<u64>,
}
impl GameAgent {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn observe(
        &mut self,
        pixels: &[u32],
        width: usize,
        height: usize,
        profile: &GameProfile,
        now: u64,
    ) -> GameDecision {
        if width < 8
            || height < 8
            || width > 512
            || height > 512
            || pixels.len() != width * height
            || !profile.valid()
            || self.last_frame.is_some_and(|t| now <= t)
        {
            self.reset();
            return GameDecision::default();
        }
        if self.last_frame.is_some_and(|t| now - t > 600) {
            self.previous = None;
            self.stable = 0;
        }
        self.last_frame = Some(now);
        let x0 = (profile.roi[0] * width as f32) as usize;
        let x1 = ((profile.roi[2] * width as f32) as usize).min(width);
        let y0 = (profile.roi[1] * height as f32) as usize;
        let y1 = ((profile.roi[3] * height as f32) as usize).min(height);
        let mut marked = vec![false; pixels.len()];
        let matches = |p: u32| {
            [
                ((p >> 16) & 255) as u8,
                ((p >> 8) & 255) as u8,
                (p & 255) as u8,
            ]
            .iter()
            .zip(profile.rgb)
            .all(|(a, b)| a.abs_diff(b) <= profile.tolerance)
        };
        let mut best = None;
        let mut best_score = f32::NEG_INFINITY;
        for y in y0..y1 {
            for x in x0..x1 {
                let index = y * width + x;
                if marked[index] || !matches(pixels[index]) {
                    continue;
                }
                let mut stack = vec![(x, y)];
                marked[index] = true;
                let (mut count, mut sx, mut sy) = (0usize, 0usize, 0usize);
                while let Some((cx, cy)) = stack.pop() {
                    count += 1;
                    sx += cx;
                    sy += cy;
                    for (nx, ny) in [
                        (cx.wrapping_sub(1), cy),
                        (cx + 1, cy),
                        (cx, cy.wrapping_sub(1)),
                        (cx, cy + 1),
                    ] {
                        if nx < x0 || nx >= x1 || ny < y0 || ny >= y1 {
                            continue;
                        }
                        let n = ny * width + nx;
                        if !marked[n] && matches(pixels[n]) {
                            marked[n] = true;
                            stack.push((nx, ny));
                        }
                    }
                }
                if count < 4 || count > ((x1 - x0) * (y1 - y0)) / 5 {
                    continue;
                }
                let point = [
                    sx as f32 / count as f32 / width as f32,
                    sy as f32 / count as f32 / height as f32,
                ];
                let anchor = self.previous.unwrap_or([0.5, 0.5]);
                let distance = (point[0] - anchor[0]).hypot(point[1] - anchor[1]);
                let score = (count as f32).sqrt() / width as f32 - distance;
                if score > best_score {
                    best_score = score;
                    best = Some(point);
                }
            }
        }
        let Some(point) = best else {
            self.previous = None;
            self.stable = 0;
            return GameDecision::default();
        };
        if self
            .previous
            .is_some_and(|p| (point[0] - p[0]).hypot(point[1] - p[1]) < 0.12)
        {
            self.stable = self.stable.saturating_add(1);
        } else {
            self.stable = 1;
        }
        self.previous = Some(point);
        let mut out = GameDecision {
            target: Some(point),
            confirmed: self.stable >= 2,
            ..GameDecision::default()
        };
        if !out.confirmed {
            return out;
        }
        let dx = point[0] - 0.5;
        let dy = point[1] - 0.5;
        let distance = dx.hypot(dy);
        if profile.moba {
            if distance > 0.15 {
                out.movement = [dx / distance, dy / distance];
            }
            out.fire = distance <= 0.20;
            if out.fire
                && self
                    .last_skill
                    .is_none_or(|t| now.saturating_sub(t) >= 1500)
            {
                out.skill = true;
                self.last_skill = Some(now);
            }
        } else {
            out.aim = [
                (dx * 0.45).clamp(-0.08, 0.08),
                (dy * 0.45).clamp(-0.08, 0.08),
            ];
            out.fire = distance <= 0.055;
        }
        out
    }
}
