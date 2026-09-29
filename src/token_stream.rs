use crate::inference_matrix::{f32_to_q15, MatrixDecision, MatrixSignal, TamThienMatrix};
use crate::types::CognitiveMoment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstantToken {
    pub text: String,
    pub ordinal: u16,
    pub final_token: bool,
}

#[derive(Clone, Debug, Default)]
pub struct InstantTokenEmitter {
    matrix: TamThienMatrix,
}

impl InstantTokenEmitter {
    pub fn signals_from_text(&self, input: &str) -> Vec<MatrixSignal> {
        let mut lanes = [0i32; 16];
        for (index, b) in input.bytes().enumerate().take(256) {
            let lane = ((b as usize).wrapping_add(index * 7)) & 15;
            let weight = 4096 + ((b as i32 & 31) * 256);
            lanes[lane] = (lanes[lane] + weight).min(i16::MAX as i32);
        }
        lanes
            .iter()
            .enumerate()
            .filter(|(_, v)| **v > 0)
            .map(|(lane, value)| MatrixSignal {
                lane: lane as u8,
                value_q15: *value as i16,
            })
            .collect()
    }

    pub fn decide(&self, input: &str, moment: Option<&CognitiveMoment>) -> MatrixDecision {
        let mut signals = self.signals_from_text(input);
        if let Some(m) = moment {
            // Reserve semantic lanes for uncertainty, feeling and hypothesis support.
            signals.push(MatrixSignal {
                lane: 13,
                value_q15: f32_to_q15((1.0 - m.uncertainty).clamp(0.0, 1.0)),
            });
            signals.push(MatrixSignal {
                lane: 14,
                value_q15: f32_to_q15(m.feeling.max(0.0)),
            });
            signals.push(MatrixSignal {
                lane: 15,
                value_q15: f32_to_q15((m.hypotheses.len() as f32 / 8.0).min(1.0)),
            });
        }
        self.matrix.infer(&signals)
    }

    /// Emits a useful first word without waiting for full contemplation when
    /// TieuThien has a decisive margin. This is a word-token stream, not an LLM.
    pub fn emit_immediate(&self, input: &str) -> Vec<InstantToken> {
        let signals = self.signals_from_text(input);
        if let Some((lane, _)) = self.matrix.infer_early(&signals) {
            return vec![InstantToken {
                text: early_word(lane, input).to_string(),
                ordinal: 0,
                final_token: false,
            }];
        }
        Vec::new()
    }

    pub fn emit_response(
        &self,
        input: &str,
        moment: &CognitiveMoment,
        response: &str,
    ) -> Vec<InstantToken> {
        let decision = self.decide(input, Some(moment));
        let mut words: Vec<String> = response
            .split_whitespace()
            .take(96)
            .map(ToString::to_string)
            .collect();

        // Put the highest-value discourse marker first when it adds information.
        let lead = lead_word(decision.winner, moment.uncertainty);
        if words.first().map(String::as_str) != Some(lead) {
            words.insert(0, lead.to_string());
        }

        let last = words.len().saturating_sub(1);
        words
            .into_iter()
            .enumerate()
            .map(|(i, text)| InstantToken {
                text,
                ordinal: i.min(u16::MAX as usize) as u16,
                final_token: i == last,
            })
            .collect()
    }
}

fn early_word(lane: u8, input: &str) -> &'static str {
    let lower = input.to_lowercase();
    if lower.contains("không") || lower.contains("khong") {
        return "Không";
    }
    if lower.contains("tại sao") || lower.contains("tai sao") {
        return "Vì";
    }
    if lower.contains("làm") || lower.contains("lam") {
        return "Được";
    }
    match lane & 3 {
        0 => "Tôi",
        1 => "Được",
        2 => "Hiện",
        _ => "Theo",
    }
}

fn lead_word(winner: u8, uncertainty: f32) -> &'static str {
    if uncertainty > 0.55 {
        return "Có";
    }
    match winner & 3 {
        0 => "Tôi",
        1 => "Được",
        2 => "Hiện",
        _ => "Theo",
    }
}
