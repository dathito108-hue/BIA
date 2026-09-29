const MAX_CALIBRATION_EVENTS: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct CalibrationEvent {
    pub predicted_confidence: f32,
    pub correct: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SelfCalibration {
    events: Vec<CalibrationEvent>,
}

impl SelfCalibration {
    pub fn observe(&mut self, predicted_confidence: f32, correct: bool) {
        if self.events.len() >= MAX_CALIBRATION_EVENTS {
            self.events.remove(0);
        }
        self.events.push(CalibrationEvent {
            predicted_confidence: predicted_confidence.clamp(0.0, 1.0),
            correct,
        });
    }

    pub fn bias(&self) -> f32 {
        if self.events.is_empty() {
            return 0.0;
        }
        self.events
            .iter()
            .map(|e| e.predicted_confidence - if e.correct { 1.0 } else { 0.0 })
            .sum::<f32>()
            / self.events.len() as f32
    }

    pub fn adjusted(&self, raw: f32) -> f32 {
        (raw - self.bias()).clamp(0.0, 1.0)
    }

    pub fn brier_score(&self) -> f32 {
        if self.events.is_empty() {
            return 0.0;
        }
        self.events
            .iter()
            .map(|e| {
                let y = if e.correct { 1.0 } else { 0.0 };
                let d = e.predicted_confidence - y;
                d * d
            })
            .sum::<f32>()
            / self.events.len() as f32
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
