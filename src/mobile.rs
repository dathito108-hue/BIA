use crate::action::{ActionDecision, ActionProposal, CuTranPolicy};
use crate::action_queue::ActionQueue;
use crate::budget::DeviceState;
use crate::capability::{infer_device_action, DeviceAction, DeviceActionKind};
use crate::continuity::{decode_continuity, encode_continuity, ContinuityState};
use crate::core::BiaDca;
use crate::dialogue::{DialogueContext, DialogueTurn, Speaker};
use crate::duyen_token::DuyenTokenDecoder;
use crate::goals::{GoalStack, GoalStatus};
use crate::knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
use crate::language::VietnameseGate;
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::token_stream::{InstantToken, InstantTokenEmitter};
use crate::planner::decompose_goal;
use crate::retrieval::SemanticRetriever;
use crate::types::{CognitiveMoment, Phenomenon, SenseGate, WorldLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct MobileReply {
    pub text: String,
    pub moment: CognitiveMoment,
    pub pending_action: Option<DeviceAction>,
}

pub struct OfflineMobileBia {
    pub bia: BiaDca,
    pub language: VietnameseGate,
    pub policy: CuTranPolicy,
    pub dialogue: DialogueContext,
    pub goals: GoalStack,
    pub knowledge: KnowledgeLedger,
    pub tokens: InstantTokenEmitter,
    pub decoder: DuyenTokenDecoder,
    pub semantic: SemanticReasoner,
    pub retriever: SemanticRetriever,
    queue: ActionQueue,
}

impl OfflineMobileBia {
    pub fn new(bia: BiaDca) -> Self {
        Self {
            bia,
            language: VietnameseGate,
            policy: CuTranPolicy::default(),
            dialogue: DialogueContext::new(24),
            goals: GoalStack::new(16),
            knowledge: KnowledgeLedger::new(96),
            tokens: InstantTokenEmitter::default(),
            decoder: DuyenTokenDecoder::default(),
            semantic: SemanticReasoner::default(),
            retriever: SemanticRetriever,
            queue: ActionQueue::new(12),
        }
    }

    pub fn converse(
        &mut self,
        text: &str,
        timestamp: u64,
        device: DeviceState,
    ) -> Option<MobileReply> {
        self.dialogue.push(DialogueTurn {
            speaker: Speaker::User,
            text: text.to_string(),
            timestamp,
        });

        let semantic_scene = self
            .semantic
            .ingest(&mut self.bia.world, text, timestamp);
        let mut semantic_answer = self
            .semantic
            .answer_scene(&self.bia.world, &semantic_scene);
        if semantic_scene.query.is_some() && matches!(semantic_answer, OpenAnswer::Unknown) {
            let hits = self.retriever.recall(&self.knowledge, text, 3);
            for hit in hits {
                let _ = self
                    .semantic
                    .ingest(&mut self.bia.world, &hit.record.excerpt, timestamp);
            }
            semantic_answer = self
                .semantic
                .answer_scene(&self.bia.world, &semantic_scene);
        }

        if let Some(goal) = extract_goal(text) {
            let id = stable_id(goal, timestamp);
            self.goals.set(id, goal.to_string(), timestamp);
            if self.queue.is_empty() {
                let plan = decompose_goal(goal, id);
                for step in plan.steps {
                    self.queue.push(step);
                }
            }
        }

        let teaching = extract_teaching(text);
        let mut ps = self.language.perceive(teaching.unwrap_or(text), timestamp);
        let focus = ps.pop()?;

        for p in &ps {
            let meaning = self.bia.observe(p.clone());
            if teaching.is_some() {
                self.bia.experience(p, meaning, 0.85, 0.0);
            }
        }

        let moment = self.bia.contemplate(focus.clone(), device, 0.7);

        if let Some(lesson) = teaching {
            self.ingest_content(
                "user",
                ProvenanceKind::User,
                lesson,
                timestamp,
                0.95,
            );
        }

        let direct = infer_device_action(text, stable_id(text, timestamp));
        if let Some(action) = direct {
            self.queue.push(action);
        } else if is_continue(text) && self.queue.is_empty() {
            if let Some(goal) = self.goals.active() {
                let plan = decompose_goal(
                    &goal.description,
                    stable_id(&goal.description, timestamp),
                );
                for step in plan.steps {
                    self.queue.push(step);
                }
            }
        }

        let pending = self.queue.front().cloned();

        let mut reply = if semantic_scene.query.is_some()
            && !matches!(semantic_answer, OpenAnswer::Unknown)
        {
            open_answer_text(&semantic_answer)
        } else if let Some(lesson) = teaching {
            format!(
                "Tôi đã ghi nhận “{}” cùng nguồn gốc và huân tập nó vào kinh nghiệm cục bộ.",
                lesson.trim()
            )
        } else if let Some(goal) = self.goals.active() {
            if is_goal_command(text) {
                format!(
                    "Tôi đã nhận mục tiêu: “{}”. Planner đã phân rã được {} bước có thể thực thi ngay.",
                    goal.description,
                    self.queue.len()
                )
            } else {
                self.language.respond(text, &moment)
            }
        } else {
            self.language.respond(text, &moment)
        };

        if let Some(a) = &pending {
            reply.push_str(&format!(
                " Bước kế tiếp là “{}”; mỗi tác động ra ngoài vẫn cần xác nhận.",
                a.label
            ));
        }

        self.dialogue.push(DialogueTurn {
            speaker: Speaker::Bia,
            text: reply.clone(),
            timestamp: timestamp.saturating_add(1),
        });

        Some(MobileReply {
            text: reply,
            moment,
            pending_action: pending,
        })
    }

    pub fn ingest_content(
        &mut self,
        source: &str,
        kind: ProvenanceKind,
        content: &str,
        timestamp: u64,
        confidence: f32,
    ) -> usize {
        let clean = content.trim();
        if clean.is_empty() {
            return 0;
        }

        let excerpt: String = clean.chars().take(4096).collect();
        let id = stable_id(&format!("{source}:{excerpt}"), timestamp);
        self.knowledge.add(KnowledgeRecord {
            id,
            source: source.to_string(),
            kind,
            excerpt: excerpt.clone(),
            timestamp,
            confidence: confidence.clamp(0.0, 1.0),
        });
        let _ = self.decoder.learn_text(&excerpt);
        let _ = self.semantic.ingest(&mut self.bia.world, &excerpt, timestamp);

        let phenomena = self.language.perceive(&excerpt, timestamp);
        let mut count = 0usize;
        for p in phenomena.into_iter().take(24) {
            let meaning = self.bia.observe(p.clone());
            self.bia.experience(&p, meaning, confidence.clamp(0.0, 1.0), 0.0);
            count += 1;
        }
        count
    }

    pub fn immediate_tokens(&mut self, input: &str) -> Vec<InstantToken> {
        let text = self.decoder.first_token(input);
        vec![InstantToken {
            text,
            ordinal: 0,
            final_token: false,
        }]
    }

    pub fn duyen_generate(&mut self, input: &str, max_tokens: usize) -> String {
        self.decoder.generate(input, max_tokens).tokens.join(" ")
    }

    pub fn learned_vocab_len(&self) -> usize {
        self.decoder.learned_vocab_len()
    }

    pub fn response_tokens(
        &mut self,
        input: &str,
        moment: &CognitiveMoment,
        response: &str,
    ) -> Vec<InstantToken> {
        self.tokens.emit_response(input, moment, response)
    }

    pub fn pending_action(&self) -> Option<&DeviceAction> {
        self.queue.front()
    }

    pub fn resolve_pending_action(&mut self, success: bool, timestamp: u64) {
        let Some(action) = self.queue.pop_front() else {
            return;
        };

        let kind = action_kind_code(action.kind);
        let event = Phenomenon::new(
            action.id,
            WorldLevel::TrungThien,
            SenseGate::System,
            kind,
            action_features(&action),
            if success { 0.95 } else { 0.8 },
            0.85,
            timestamp,
        );
        self.bia.observe(event.clone());
        self.bia.experience(
            &event,
            kind,
            if success { 0.95 } else { 0.0 },
            if success { 0.0 } else { 0.85 },
        );

        if let Some(goal) = self.goals.active() {
            if success {
                let next = (goal.progress + 0.25).min(1.0);
                let status = if next >= 1.0 {
                    GoalStatus::Completed
                } else {
                    GoalStatus::Active
                };
                self.goals.update_active(next, status);
            } else {
                self.goals.update_active(goal.progress, GoalStatus::Blocked);
                self.queue.clear();
            }
        }
    }

    pub fn continuity_export(&self) -> String {
        encode_continuity(&ContinuityState {
            active_goal: self.goals.active().cloned(),
            queued_actions: self.queue.items().cloned().collect(),
            knowledge: self.knowledge.records().cloned().collect(),
        })
    }

    pub fn continuity_import(&mut self, text: &str) -> bool {
        let Some(state) = decode_continuity(text) else {
            return false;
        };
        if let Some(goal) = state.active_goal {
            self.goals.restore_active(goal);
        }
        self.queue.clear();
        for action in state.queued_actions {
            self.queue.push(action);
        }
        for record in state.knowledge {
            self.knowledge.add(record);
        }
        true
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn authorize(&self, proposal: ActionProposal) -> ActionDecision {
        self.policy.evaluate(proposal)
    }
}

fn extract_teaching(input: &str) -> Option<&str> {
    let lower = input.to_lowercase();
    for prefix in ["nhớ rằng ", "ghi nhớ rằng ", "ghi nhớ ", "nho rang ", "ghi nho "] {
        if lower.starts_with(prefix) {
            let n = prefix.chars().count();
            return input
                .char_indices()
                .nth(n)
                .map(|(i, _)| &input[i..])
                .or(Some(""));
        }
    }
    None
}

fn extract_goal(input: &str) -> Option<&str> {
    let lower = input.to_lowercase();
    for prefix in ["mục tiêu: ", "mục tiêu ", "muc tieu: ", "muc tieu "] {
        if lower.starts_with(prefix) {
            let n = prefix.chars().count();
            return input
                .char_indices()
                .nth(n)
                .map(|(i, _)| &input[i..])
                .or(Some(""));
        }
    }
    None
}

fn is_goal_command(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower.starts_with("mục tiêu") || lower.starts_with("muc tieu")
}

fn is_continue(input: &str) -> bool {
    matches!(
        input.trim().to_lowercase().as_str(),
        "tiếp tục" | "tiep tuc" | "làm tiếp" | "lam tiep" | "kế tiếp" | "ke tiep"
    )
}

fn stable_id(text: &str, timestamp: u64) -> u64 {
    let mut h = 0xcbf29ce484222325_u64 ^ timestamp;
    for b in text.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn action_kind_code(kind: DeviceActionKind) -> u32 {
    match kind {
        DeviceActionKind::OpenUrl => 0xA001,
        DeviceActionKind::SearchWeb => 0xA002,
        DeviceActionKind::OpenSettings => 0xA003,
        DeviceActionKind::LaunchPackage => 0xA004,
        DeviceActionKind::ClipboardWrite => 0xA005,
    }
}

fn action_features(action: &DeviceAction) -> Vec<f32> {
    let mut out = vec![0.0; 8];
    for (i, b) in action.payload.bytes().enumerate() {
        out[i % 8] += f32::from(b) / 255.0;
    }
    out[0] += action.confidence;
    out
}


fn open_answer_text(answer: &OpenAnswer) -> String {
    match answer {
        OpenAnswer::Supported { confidence, path } => format!(
            "Có cơ sở nhân–quả để ủng hộ kết luận này. Độ tin cậy {:.0}%, chuỗi Duyên có {} mắt xích.",
            confidence * 100.0,
            path.len().saturating_sub(1)
        ),
        OpenAnswer::Opposed { confidence, path } => format!(
            "Bằng chứng hiện tại nghiêng về phía phủ định. Độ tin cậy {:.0}%, chuỗi ức chế có {} mắt xích.",
            confidence * 100.0,
            path.len().saturating_sub(1)
        ),
        OpenAnswer::Contradicted { support, opposition } => format!(
            "Tôi thấy mâu thuẫn thật trong các Duyên: ủng hộ {:.0}% và phản đối {:.0}%. Chưa nên kết luận một chiều.",
            support * 100.0,
            opposition * 100.0
        ),
        OpenAnswer::Counterfactual {
            support_delta,
            factual_support,
            counterfactual_support,
        } => format!(
            "Nếu bỏ điều kiện đó, mức ủng hộ thay đổi {:.0} điểm phần trăm: từ {:.0}% xuống {:.0}%.",
            support_delta * 100.0,
            factual_support * 100.0,
            counterfactual_support * 100.0
        ),
        OpenAnswer::Unknown => {
            "Tôi chưa dựng được chuỗi Duyên đủ chắc từ dữ kiện hiện có.".to_string()
        }
    }
}
