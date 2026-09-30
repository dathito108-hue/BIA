//! Bounded integration of source-governed reasoning, feedback and simulation.
//! Only direct chat commands reach this component; retrieved documents never do.
use crate::autonomous_cognitive_loop::AutonomousCognitiveLoop;
use crate::deliberation::{DeliberativePlanner, GoalSpec};
use crate::generative_cognition::GenerativeCognition;
use crate::metacognition::MetacognitiveController;
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::semantic::{concept_id, normalize, VietnameseSemanticParser};
use crate::world::WorldGraph;
use crate::world_model::{SimState, TransitionModel, WorldModel};

const MAX_JOURNAL: usize = 128;
const MAX_INPUT: usize = 1024;
const MAX_SOURCES: usize = 32;
const MAX_SKILLS: usize = 32;
const MAX_EXAMPLES: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceKind {
    Observation,
    Report,
    Hypothesis,
}
#[derive(Clone, Debug)]
struct Source {
    name: String,
    text: String,
    kind: SourceKind,
}
#[derive(Clone, Debug)]
struct Skill {
    name: String,
    requires: Vec<u64>,
    adds: Vec<u64>,
    blocked: bool,
    successes: u8,
    failures: u8,
}
#[derive(Clone, Debug)]
struct Example {
    class: String,
    instance: String,
    outcome: String,
    counter: bool,
}

#[derive(Clone, Debug, Default)]
pub struct IntegratedCognition {
    sources: Vec<Source>,
    skills: Vec<Skill>,
    examples: Vec<Example>,
    journal: Vec<String>,
    topic: Option<String>,
    last_plan: Option<(String, String)>,
    last_question: Option<String>,
    continuity: crate::conversation_continuity::ConversationContinuity,
    learned_language: crate::learned_language::LearnedLanguage,
    last_evidence: Vec<String>,
    last_source_snapshot: Vec<(String, u64)>,
    dialogue_goal: crate::dialogue_goal_state::DialogueGoalState,
    grounding: crate::dialogue_grounding::GroundingState,
    bindings: Vec<(String, crate::capability::DeviceAction)>,
    execution_failures: Vec<String>,
}

impl IntegratedCognition {
    /// Returns None for ordinary chat so the existing BIA pipeline remains canonical.
    pub fn handle(&mut self, input: &str) -> Option<String> {
        // Preserve exact Unicode, case and punctuation in Android payloads.
        if let Some((head, payload)) = input.split_once(':') {
            if let Some(name) = normalize(head).trim().strip_prefix("gan thao tac ") {
                if self.journal.len() >= MAX_JOURNAL || input.chars().count() > MAX_INPUT {
                    return Some("Đã đạt giới hạn; chưa gắn thao tác.".into());
                }
                if !self.skills.iter().any(|skill| skill.name == name) {
                    return Some("Hãy khai báo kỹ năng trước khi gắn thao tác.".into());
                }
                let Some(action) = crate::skill_execution::parse_binding(payload.trim()) else {
                    return Some("Thao tác chưa được hỗ trợ. Chỉ gắn mở cài đặt, URL HTTP(S), tìm web, mở gói ứng dụng hoặc sao chép.".into());
                };
                if let Some(binding) = self.bindings.iter_mut().find(|b| b.0 == name) {
                    binding.1 = action;
                } else {
                    self.bindings.push((name.to_string(), action));
                }
                self.journal.push(input.to_string());
                return Some("Đã gắn thao tác cụ thể. Khi chạy, bạn sẽ thấy nội dung từng bước để phê duyệt.".into());
            }
        }
        let folded = input
            .split("->")
            .map(normalize)
            .collect::<Vec<_>>()
            .join(" -> ");
        let folded = folded.trim();
        let (head, body) = folded.split_once(':').unwrap_or((folded, ""));
        let head = head.trim();
        let body = body.trim();
        let mutation = [
            "cach noi ",
            "rut cach noi ",
            "nguon ",
            "quan sat ",
            "gia thuyet ",
            "dinh chinh ",
            "rut nguon ",
            "ky nang ",
            "ket qua ",
            "vi du ",
            "phan vi du ",
        ]
        .iter()
        .any(|p| head.starts_with(p));
        let recognized = mutation
            || head == "hoi"
            || head == "lap ke hoach"
            || head == "suy rong"
            || head == "lap lai ke hoach";
        if !recognized {
            return self.conversation(input);
        }
        if input.chars().count() > MAX_INPUT {
            return Some("Yêu cầu quá dài; hãy chia nhỏ dưới 1.024 ký tự.".into());
        }
        if mutation && self.journal.len() >= MAX_JOURNAL {
            return Some(
                "Bộ nhớ phiên đã đầy; tôi không ghi đè nguồn hay lịch sử đính chính.".into(),
            );
        }
        let (reply, changed) = self.execute(head, body);
        if changed {
            self.journal.push(input.to_string());
        }
        Some(reply)
    }

    fn conversation(&mut self,input:&str)->Option<String>{
        use crate::conversation_language::{understand,Frame};
        let normalized = normalize(input).trim().trim_end_matches(['?','!','.']).to_string();
        if let Some(goal)=crate::dialogue_goal_state::parse_goal(input) {
            use crate::dialogue_goal_state::ImplicitDialogueGoal;
            match goal {
                ImplicitDialogueGoal::Challenge => {
                    return Some(self.challenge_current_relation());
                }
                ImplicitDialogueGoal::Conclude => {
                    return Some(self.conclude_current_relation());
                }
                ImplicitDialogueGoal::Explore | ImplicitDialogueGoal::Verify => {}
            }
        }
        if let Some(plan) = crate::dynamic_dialogue_intent::DynamicDialoguePlan::parse(input) {
            return Some(self.execute_dynamic_dialogue_plan(plan));
        }
        if let Some(act) = crate::conversational_implicature::parse(input) {
            use crate::conversational_implicature::ConversationAct;
            match act {
                ConversationAct::Acknowledge => {
                    return Some(if self.last_question.is_some() {
                        "Được. Tôi giữ mạch vừa rồi và không lặp lại phần đã rõ.".into()
                    } else {
                        "Được.".into()
                    });
                }
                ConversationAct::Confirm => {
                    let Some(q)=self.last_question.clone() else {
                        return Some("Chưa có kết luận gần đây để xác nhận.".into());
                    };
                    let brief=self.answer_styled(&q,crate::expression_style::ExpressionStyle::Brief);
                    return Some(format!("Nếu bạn đang xác nhận kết luận vừa rồi: {brief}"));
                }
                ConversationAct::Doubt => {
                    let Some(q)=self.last_question.clone() else {
                        return Some("Chưa có kết luận gần đây để kiểm tra lại.".into());
                    };
                    self.dialogue_goal.advance(crate::dialogue_goal_state::ImplicitDialogueGoal::Verify);
                    let deep=self.answer_styled(&q,crate::expression_style::ExpressionStyle::Deep);
                    self.last_source_snapshot=self.relation_source_snapshot(&q);
                    return Some(format!("Tôi kiểm tra lại mà không tăng độ chắc chỉ vì bị hỏi lại. {deep}"));
                }
                ConversationAct::Expand => {
                    let Some(q)=self.last_question.clone() else {
                        return Some("Chưa có câu hỏi gần đây để mở rộng.".into());
                    };
                    self.dialogue_goal.advance(crate::dialogue_goal_state::ImplicitDialogueGoal::Explore);
                    let deep=self.answer_styled(&q,crate::expression_style::ExpressionStyle::Deep);
                    self.last_source_snapshot=self.relation_source_snapshot(&q);
                    return Some(format!("Mở rộng thêm từ cùng mạch bằng chứng: {deep}"));
                }
                ConversationAct::NewOnly => {
                    return Some(self.response_delta());
                }
            }
        }
        if let Some(move_)=crate::dialogue_grounding::parse(input, self.last_question.as_deref()) {
            use crate::dialogue_grounding::GroundingMove;
            match move_ {
                GroundingMove::CorrectRelation { question } => {
                    self.grounding.note_repair();
                    return Some(format!(
                        "Hiểu rồi — tôi sửa mạch theo ý bạn. {}",
                        self.answer(&question)
                    ));
                }
                GroundingMove::ReplaceSubject { rejected, replacement } => {
                    return Some(self.apply_grounding_entity_repair(
                        &rejected,
                        &replacement,
                        true,
                    ));
                }
                GroundingMove::ReplaceObject { rejected, replacement } => {
                    return Some(self.apply_grounding_entity_repair(
                        &rejected,
                        &replacement,
                        false,
                    ));
                }
                GroundingMove::Clarify { prompt } => {
                    self.grounding.note_unresolved();
                    return Some(prompt);
                }
            }
        }
        if let Some(move_) = crate::contextual_pragmatics::parse(input) {
            use crate::contextual_pragmatics::PragmaticMove;
            match move_ {
                PragmaticMove::TopicShift(topic) => {
                    self.last_question = None;
                    self.topic = None;
                    self.dialogue_goal.clear_active();
                    self.grounding.clear_unresolved();
                    return Some(format!(
                        "Được, chuyển sang chủ đề “{topic}”. Mạch quan hệ trước vẫn được giữ trong lịch sử để bạn có thể quay lại khi cần."
                    ));
                }
                PragmaticMove::EllipticEntity(entity) => {
                    let Some(current) = self.last_question.clone() else {
                        return Some(
                            "Chưa có quan hệ hiện tại để điền phần bị lược; hãy nêu câu hỏi đầy đủ một lần."
                                .into(),
                        );
                    };
                    let scene = VietnameseSemanticParser.parse(&current);
                    let Some(query) = scene.query else {
                        return Some(
                            "Ngữ cảnh hiện tại chưa đủ rõ để suy ra phần bị lược; hãy nêu nguyên nhân hoặc kết quả."
                                .into(),
                        );
                    };
                    let entity = normalize(&entity).trim().to_string();
                    let role = if entity == query.subject.text && entity != query.object.text {
                        Some(true)
                    } else if entity == query.object.text && entity != query.subject.text {
                        Some(false)
                    } else {
                        self.pragmatic_entity_role(
                            &entity,
                            &query.subject.text,
                            &query.object.text,
                        )
                    };
                    let Some(as_subject) = role else {
                        return Some(format!(
                            "“{entity}” chưa có một vai duy nhất trong ngữ cảnh. Bạn muốn xét nó ở vai nguyên nhân hay kết quả? Hãy nói rõ “còn nguyên nhân {entity} thì sao” hoặc “còn kết quả {entity} thì sao”."
                        ));
                    };
                    let question = if as_subject {
                        format!("{entity} co gay ra {} khong", query.object.text)
                    } else {
                        format!("{} co gay ra {entity} khong", query.subject.text)
                    };
                    return Some(self.answer(&question));
                }
            }
        }
        if let Some(plan)=crate::intent_fusion::IntentFusionPlan::parse(input) {
            use crate::intent_fusion::ResponseSection;
            let Some(current)=self.last_question.clone() else {
                return Some("Chưa có câu hỏi hiện tại để hợp nhất kế hoạch trả lời.".into());
            };
            let previous=self.continuity.previous().map(|turn|turn.question.clone());
            let current_scene=VietnameseSemanticParser.parse(&current);
            let focus_valid=plan.focus.as_ref().is_none_or(|focus|{
                let Some(query)=current_scene.query.as_ref() else{return false};
                focus==&query.subject.text || focus==&query.object.text
            });
            if !focus_valid {
                return Some("Trọng tâm được yêu cầu chưa khớp rõ với hai đối tượng của câu hỏi hiện tại; hãy nêu lại đối tượng cần tập trung.".into());
            }
            use crate::natural_surface::{NaturalSurfaceRealizer, SurfacePart, SurfaceSection};
            let mut parts=Vec::new();
            if plan.contains(ResponseSection::Explain) {
                parts.push(SurfacePart::new(
                    SurfaceSection::Explain,
                    self.answer_styled(&current,crate::expression_style::ExpressionStyle::Deep),
                ));
            }
            if plan.contains(ResponseSection::ComparePrevious) {
                let Some(previous)=previous.clone() else {
                    return Some("Chưa có trường hợp trước đủ rõ để so sánh.".into());
                };
                let prior=self.answer_styled(&previous,crate::expression_style::ExpressionStyle::Standard);
                let now=self.answer_styled(&current,crate::expression_style::ExpressionStyle::Standard);
                parts.push(SurfacePart::new(
                    SurfaceSection::ComparePrevious,
                    format!("trường hợp trước là: {prior} Còn trường hợp hiện tại là: {now}"),
                ));
            }
            if plan.contains(ResponseSection::Summary) {
                parts.push(SurfacePart::new(
                    SurfaceSection::Summary,
                    self.answer_styled(&current,crate::expression_style::ExpressionStyle::Brief),
                ));
            }
            if plan.contains(ResponseSection::ConclusionFromPrevious) {
                let Some(previous)=previous else {
                    return Some("Chưa có trường hợp trước đủ rõ để dùng làm kết luận tham chiếu.".into());
                };
                parts.push(SurfacePart::new(
                    SurfaceSection::ConclusionFromPrevious,
                    self.answer_styled(&previous,crate::expression_style::ExpressionStyle::Brief),
                ));
            }
            if parts.is_empty() {
                return Some("Kế hoạch trả lời sau khi loại các phần yêu cầu đã rỗng; hãy giữ lại ít nhất một mục.".into());
            }
            return Some(NaturalSurfaceRealizer.compose(plan.focus.as_deref(), &parts));
        }
        if let Some(plan)=crate::compositional_dialogue::CompositeDialoguePlan::parse(input) {
            use crate::compositional_dialogue::DialogueGoal;
            let Some(current)=self.last_question.clone() else {
                return Some("Chưa có câu hỏi hiện tại để ghép nhiều mục tiêu giao tiếp.".into());
            };
            let previous=self.continuity.previous().map(|turn|turn.question.clone());
            use crate::natural_surface::{NaturalSurfaceRealizer, SurfacePart, SurfaceSection};
            let mut parts=Vec::new();
            if plan.contains(DialogueGoal::ExplainCurrent) {
                parts.push(SurfacePart::new(
                    SurfaceSection::Explain,
                    self.answer_styled(&current,crate::expression_style::ExpressionStyle::Deep),
                ));
            }
            if plan.contains(DialogueGoal::ComparePrevious) {
                let Some(previous)=previous else {
                    return Some("Chưa có trường hợp trước đủ rõ để so sánh.".into());
                };
                let prior=self.answer_styled(&previous,crate::expression_style::ExpressionStyle::Standard);
                let current_standard=self.answer_styled(&current,crate::expression_style::ExpressionStyle::Standard);
                parts.push(SurfacePart::new(
                    SurfaceSection::ComparePrevious,
                    format!("trường hợp trước là: {prior} Còn trường hợp hiện tại là: {current_standard}"),
                ));
            }
            if plan.contains(DialogueGoal::Summarize) {
                parts.push(SurfacePart::new(
                    SurfaceSection::Summary,
                    self.answer_styled(&current,crate::expression_style::ExpressionStyle::Brief),
                ));
            }
            return Some(NaturalSurfaceRealizer.compose(None, &parts));
        }
        if let Some(body)=normalized.strip_prefix("y toi la ") {
            if let Some((replacement,rejected))=body.split_once(" chu khong phai ") {
                let Some(q)=self.continuity.correct_last_entity(replacement,rejected) else {
                    return Some("Tôi chưa tìm thấy đối tượng cần sửa trong lượt gần nhất; hãy nêu lại câu hỏi đầy đủ.".into());
                };
                self.last_question=Some(q.clone());
                self.dialogue_goal.bind_relation(&q);
                let scene=VietnameseSemanticParser.parse(&q);
                let corrected=scene
                    .query
                    .as_ref()
                    .map(|query| query.object.text.as_str())
                    .unwrap_or("đối tượng vừa nêu");
                let answer=self.answer_restate(&q);
                return Some(format!("Đã hiểu, bạn đang sửa đối tượng sang “{corrected}”. {answer}"));
            }
        }
        if matches!(normalized.as_str(), "y truoc"|"y vua roi"|"truong hop truoc"|"truong hop vua roi"|"truong hop kia") {
            let Some(q)=self.continuity.resolve_reference(&normalized) else {
                return Some("Chưa có lượt hội thoại trước đủ rõ để tham chiếu.".into());
            };
            return Some(self.answer(&q));
        }
        if normalized.starts_with("bo duyen thu ") || normalized.starts_with("neu bo duyen thu ") {
            return Some("Chưa thể xác định duyên theo số thứ tự một cách an toàn. Hãy nêu tên duyên cần bỏ, ví dụ: nếu bỏ độ ẩm thì đường trơn.".into());
        }
        if normalized.starts_with("cai thu nhat") || normalized.starts_with("doi tuong thu nhat")
            || normalized.starts_with("cai thu hai") || normalized.starts_with("doi tuong thu hai") {
            let key = if normalized.starts_with("cai thu nhat") || normalized.starts_with("doi tuong thu nhat") {"cai thu nhat"} else {"cai thu hai"};
            let Some(entity)=self.continuity.resolve_reference(key) else {
                return Some("Chưa có câu hỏi trước đủ rõ để xác định đối tượng được nhắc tới.".into());
            };
            if normalized.ends_with("thi sao") {
                return Some(format!("Bạn đang nhắc tới “{entity}”. Bạn muốn xét nó ở vai nguyên nhân hay kết quả?"));
            }
            return Some(format!("Đối tượng được nhắc tới là “{entity}”."));
        }
        let expanded=self.learned_language.expand(input);
        let Some(frame)=understand(expanded.as_deref().unwrap_or(input)) else {self.last_question=None;self.topic=None;self.dialogue_goal.clear_active();return None};
        match frame {
            Frame::Alternative{subject,entity}=>{
                let Some(last)=self.last_question.as_ref() else{return Some("Hãy nêu câu hỏi quan hệ trước khi đổi đối tượng.".into())};
                let scene=VietnameseSemanticParser.parse(last);
                let Some(previous)=scene.query else{return Some("Ngữ cảnh chưa đủ rõ; hãy nêu lại hai đối tượng.".into())};
                let (a,b)=if subject{(entity,previous.object.text)}else{(previous.subject.text,entity)};
                Some(self.answer(&format!("{a} co gay ra {b} khong")))
            },
            Frame::Question(q)=>{
                if self.sources.is_empty() && !q.starts_with("no ") && !q.starts_with("dieu do "){return None}
                Some(self.answer(&q))
            },
            Frame::Remember(body)=>{
                let mut number=self.journal.len();
                while self.sources.iter().any(|s|s.name==format!("hoithoai{number}")){number+=1;}
                self.handle(&format!("Nguồn hoithoai{number}: {body}"))
            },
            Frame::Explain|Frame::Brief=>{
                let Some(q)=self.last_question.clone() else{return Some("Bạn muốn tôi giải thích câu hỏi nào? Hãy nêu lại nội dung.".into())};
                let style=if matches!(frame,Frame::Brief){crate::expression_style::ExpressionStyle::Brief}else{crate::expression_style::ExpressionStyle::Deep};
                Some(self.answer_styled(&q,style))
            },
            Frame::Sources=>{
                let Some(q)=self.last_question.clone() else{return Some("Chưa có câu hỏi gần đây để xác định nguồn. Bạn muốn kiểm tra điều gì?".into())};
                self.answer_restate(&q);
                self.last_source_snapshot=self.relation_source_snapshot(&q);
                let mut out=String::from("Nguồn của đường suy luận vừa xét (thông tin được báo lại, chưa xác minh độc lập):");
                for name in &self.last_evidence {if let Some(source)=self.sources.iter().find(|s|&s.name==name){out.push_str(&format!("\n{}: {}",source.name,source.text));}}
                if self.last_evidence.is_empty(){out.push_str(" chưa xác định được đường bằng chứng đơn nhất; hãy xem phần giải thích.");}
                Some(out)
            },
            Frame::Reply(text)=>{if text.starts_with("Tôi chưa") {self.last_question=None;self.topic=None;self.dialogue_goal.clear_active();}Some(text.into())},
            Frame::Clarify=>Some("“Nó/điều đó” chưa rõ chỉ đối tượng nào; hãy nêu tên cụ thể.".into()),
        }
    }

    fn execute(&mut self, head: &str, body: &str) -> (String, bool) {
        if let Some(phrase)=head.strip_prefix("cach noi ") {
            if body!="gay ra" {return ("Hiện chỉ học cách nói tương đương quan hệ gây ra; chưa ghi thay đổi.".into(),false)}
            return if self.learned_language.teach(phrase){("Đã học cách diễn đạt. Tôi sẽ áp dụng cho đối tượng mới trong câu hỏi và câu dạy quan hệ.".into(),true)}else{("Cụm từ trùng, mơ hồ, không hợp lệ hoặc đã hết chỗ; chưa học thêm.".into(),false)};
        }
        if let Some(phrase)=head.strip_prefix("rut cach noi ") {
            return if body.is_empty()&&self.learned_language.forget(phrase){("Đã rút cách nói đã học; các nguồn tri thức trước đó vẫn giữ nguyên.".into(),true)}else{("Không tìm thấy cách nói hoặc cú pháp chưa đúng.".into(),false)};
        }
        for (prefix, kind) in [
            ("nguon ", SourceKind::Report),
            ("quan sat ", SourceKind::Observation),
            ("gia thuyet ", SourceKind::Hypothesis),
        ] {
            if let Some(name) = head.strip_prefix(prefix) {
                if !valid_name(name) || body.is_empty() {
                    return invalid();
                }
                if self.sources.iter().any(|s| s.name == name) {
                    return (
                        "Tên nguồn đã tồn tại; hãy dùng Đính chính để thay thế có chủ đích.".into(),
                        false,
                    );
                }
                if self.sources.len() >= MAX_SOURCES {
                    return full();
                }
                self.update_topic(body);
                self.sources.push(Source {
                    name: name.into(),
                    text: body.into(),
                    kind,
                });
                return (
                    if kind == SourceKind::Hypothesis {
                        "Đã lưu giả thuyết; chưa dùng nó làm bằng chứng."
                    } else {
                        "Đã ghi nguồn riêng biệt để có thể kiểm tra hoặc đính chính."
                    }
                    .into(),
                    true,
                );
            }
        }
        if let Some(name) = head.strip_prefix("dinh chinh ") {
            if body.is_empty() {
                return invalid();
            }
            let Some(source) = self.sources.iter_mut().find(|s| s.name == name) else {
                return ("Không tìm thấy nguồn để đính chính.".into(), false);
            };
            source.text = body.into();
            self.update_topic(body);
            return (
                "Đã thay nội dung nguồn. Những kết luận từ nội dung cũ sẽ được tính lại khi hỏi."
                    .into(),
                true,
            );
        }
        if let Some(name) = head.strip_prefix("rut nguon ") {
            let before = self.sources.len();
            self.sources.retain(|s| s.name != name);
            if self.sources.len() == before {
                return ("Không tìm thấy nguồn.".into(), false);
            }
            self.topic = None;
            return ("Đã rút nguồn; không còn dùng nó để suy luận.".into(), true);
        }
        if head == "hoi" {
            return (self.answer(body), false);
        }
        if let Some(name) = head.strip_prefix("ky nang ") {
            let Some((requires, adds)) = body.split_once(" -> ") else {
                return invalid();
            };
            let (Some(requires), Some(adds)) = (facts(requires), facts(adds)) else {
                return invalid();
            };
            if !valid_name(name) {
                return invalid();
            }
            if self.skills.iter().any(|s| s.name == name) {
                return (
                    "Kỹ năng đã tồn tại; phản hồi kết quả sẽ điều chỉnh mức tin cậy.".into(),
                    false,
                );
            }
            if self.skills.len() >= MAX_SKILLS {
                return full();
            }
            self.skills.push(Skill {
                name: name.into(),
                requires,
                adds,
                blocked: false,
                successes: 0,
                failures: 0,
            });
            return (
                "Đã ghi mô tả kỹ năng để mô phỏng. Chưa thực thi thao tác nào.".into(),
                true,
            );
        }
        if let Some(name) = head.strip_prefix("ket qua ") {
            let success = match body {
                "thanh cong" => true,
                "that bai" => false,
                _ => return invalid(),
            };
            let Some(skill) = self.skills.iter_mut().find(|s| s.name == name) else {
                return ("Không có kỹ năng tương ứng để ghi phản hồi.".into(), false);
            };
            if (success && skill.successes > 0 && !skill.blocked) || (!success && skill.blocked) {
                return (
                    "Phản hồi này đã được ghi; không tăng điểm vì lặp lại cùng trạng thái.".into(),
                    false,
                );
            }
            if success {
                skill.successes = skill.successes.saturating_add(1);
                skill.blocked = false;
                self.execution_failures.retain(|n| n != name);
            } else {
                skill.failures = skill.failures.saturating_add(1);
                skill.blocked = true;
            }
            let mut reply =
                "Đã ghi kết quả do bạn xác nhận; tôi chưa tự kiểm chứng trên thiết bị.".to_string();
            if let Some((start, goal)) = self.last_plan.clone() {
                reply.push(' ');
                reply.push_str(&self.plan(&start, &goal));
            }
            return (reply, true);
        }
        if head == "lap ke hoach" {
            let Some((start, goal)) = body.split_once(" -> ") else {
                return invalid();
            };
            if facts(start).is_none() || goal_facts(goal).is_none() {
                return invalid();
            }
            self.last_plan = Some((start.into(), goal.into()));
            return (self.plan(start, goal), false);
        }
        if head == "lap lai ke hoach" {
            return (
                match self.last_plan.clone() {
                    Some((start, goal)) => self.plan(&start, &goal),
                    None => "Chưa có kế hoạch trong phiên này.".into(),
                },
                false,
            );
        }
        for (prefix, counter) in [("vi du ", false), ("phan vi du ", true)] {
            if let Some(class) = head.strip_prefix(prefix) {
                let Some((instance, outcome)) = body.split_once(" -> ") else {
                    return invalid();
                };
                let (instance, outcome) = (instance.trim(), outcome.trim());
                if !valid_name(class) || !valid_name(instance) || !valid_name(outcome) {
                    return invalid();
                }
                if let Some(old) = self
                    .examples
                    .iter_mut()
                    .find(|e| e.class == class && e.instance == instance)
                {
                    if old.outcome == outcome && old.counter == counter {
                        return (
                            "Ví dụ này đã có; không tính lặp thành bằng chứng mới.".into(),
                            false,
                        );
                    }
                    old.outcome = outcome.into();
                    old.counter = counter;
                } else {
                    if self.examples.len() >= MAX_EXAMPLES {
                        return full();
                    }
                    self.examples.push(Example {
                        class: class.into(),
                        instance: instance.into(),
                        outcome: outcome.into(),
                        counter,
                    });
                }
                return ("Đã cập nhật ví dụ; suy rộng cần ít nhất hai trường hợp khác nhau và không có phản ví dụ.".into(), true);
            }
        }
        if head == "suy rong" {
            let Some((class, target)) = body.split_once(" cho ") else {
                return invalid();
            };
            let examples: Vec<_> = self
                .examples
                .iter()
                .filter(|e| e.class == class.trim())
                .collect();
            if !valid_name(target) || examples.iter().any(|e| e.instance == target.trim()) {
                return (
                    "Hãy chọn một trường hợp mới chưa nằm trong các ví dụ.".into(),
                    false,
                );
            }
            if examples.len() < 2 {
                return (
                    "Chưa đủ hai ví dụ khác nhau theo tên trường hợp để suy rộng.".into(),
                    false,
                );
            }
            if examples
                .iter()
                .any(|e| e.counter || e.outcome != examples[0].outcome)
            {
                return (
                    "Có phản ví dụ hoặc kết quả khác nhau; tôi tạm giữ suy rộng.".into(),
                    false,
                );
            }
            return (format!("Giả thuyết cho {}: có thể đạt {} theo {} ví dụ khác nhau của lớp {}. Cần kiểm chứng; chưa ghi thành sự thật hay kỹ năng thực thi.",
                target.trim(), examples[0].outcome, examples.len(), class.trim()), false);
        }
        invalid()
    }

    fn pragmatic_entity_role(
        &self,
        entity: &str,
        current_subject: &str,
        current_object: &str,
    ) -> Option<bool> {
        let entity = normalize(entity).trim().to_string();
        let current_subject = normalize(current_subject).trim().to_string();
        let current_object = normalize(current_object).trim().to_string();
        if entity.is_empty() || current_subject.is_empty() || current_object.is_empty() {
            return None;
        }

        let (mut as_subject, mut as_object) = self
            .continuity
            .contextual_role_flags(&entity, &current_subject, &current_object);
        let parser = VietnameseSemanticParser;
        for source in self.sources.iter().filter(|s| s.kind != SourceKind::Hypothesis) {
            let scene = parser.parse(&source.text);
            for clause in scene.clauses.iter().take(8) {
                if clause.subject.text == entity && clause.object.text == current_object {
                    as_subject = true;
                }
                if clause.object.text == entity && clause.subject.text == current_subject {
                    as_object = true;
                }
            }
        }

        match (as_subject, as_object) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            _ => None,
        }
    }

    fn challenge_current_relation(&mut self) -> String {
        use crate::dialogue_goal_state::ImplicitDialogueGoal;
        let Some(q)=self.last_question.clone() else {
            return "Chưa có quan hệ hiện tại để tìm phản chứng.".into();
        };
        self.dialogue_goal.advance(ImplicitDialogueGoal::Challenge);

        let reasoner=SemanticReasoner::default();
        let scene=reasoner.parse(&q);
        let Some(query)=scene.query else {
            return "Quan hệ hiện tại chưa đủ rõ để tìm phản chứng.".into();
        };

        let mut world=WorldGraph::new(1024,512);
        for source in self.sources.iter().filter(|s|s.kind!=SourceKind::Hypothesis) {
            let mut parsed=reasoner.parse(&source.text);
            parsed.query=None;
            parsed.clauses.truncate(8);
            let quality:f32=if source.kind==SourceKind::Observation {0.95}else{0.75};
            for clause in &mut parsed.clauses {
                clause.confidence*=quality.sqrt();
            }
            VietnameseSemanticParser.ingest(&mut world,&parsed,0);
        }

        let (_,paths)=crate::reasoning::CausalReasoner::new(6,16)
            .infer_between_with_paths(&world,query.subject.id,query.object.id);
        let opposing:Vec<_>=paths.iter().filter(|path|path.inhibited).collect();
        if opposing.is_empty() {
            return "Tôi đã kiểm tra các nhánh Duyên hiện có nhưng chưa tìm thấy phản chứng nối từ nguyên nhân tới kết quả đang xét.".into();
        }

        let mut sources=Vec::new();
        for source in self.sources.iter().filter(|s|s.kind!=SourceKind::Hypothesis) {
            let parsed=reasoner.parse(&source.text);
            let contributes=parsed.clauses.iter().take(8).any(|clause|{
                matches!(clause.kind,crate::types::RelationKind::Inhibits)
                    && opposing.iter().any(|path|{
                        path.nodes.windows(2).any(|pair|{
                            pair[0]==clause.subject.id && pair[1]==clause.object.id
                        })
                    })
            });
            if contributes && !sources.contains(&source.name) {
                sources.push(source.name.clone());
            }
        }

        if sources.is_empty() {
            "Có nhánh phản đối trong đồ thị Duyên, nhưng chưa tách được một nguồn ức chế đơn nhất để trình bày; cần xem giải thích sâu.".into()
        } else {
            format!(
                "Có phản chứng trong mạch hiện tại. Nguồn tạo cạnh ức chế trên nhánh phản đối: {}. Tôi giữ nhánh này cùng nhánh ủng hộ thay vì tự loại một phía.",
                sources.join(", ")
            )
        }
    }

    fn conclude_current_relation(&mut self) -> String {
        use crate::dialogue_goal_state::ImplicitDialogueGoal;
        let Some(q)=self.last_question.clone() else {
            return "Chưa có quan hệ hiện tại để chốt kết luận.".into();
        };
        let had_challenge=self.dialogue_goal.has_visited(ImplicitDialogueGoal::Challenge);
        let had_verify=self.dialogue_goal.has_visited(ImplicitDialogueGoal::Verify);
        self.dialogue_goal.advance(ImplicitDialogueGoal::Conclude);
        let brief=self.answer_styled(&q,crate::expression_style::ExpressionStyle::Brief);
        let prefix=if had_challenge {
            "Sau bước kiểm tra phản chứng trong mạch này"
        } else if had_verify {
            "Sau bước kiểm tra lại"
        } else {
            "Theo bằng chứng hiện có"
        };
        format!("{prefix}, kết luận hiện tại: {brief}")
    }

    fn apply_grounding_entity_repair(
        &mut self,
        rejected: &str,
        replacement: &str,
        subject_role: bool,
    ) -> String {
        let Some(corrected) = self
            .continuity
            .correct_last_entity(replacement, rejected)
        else {
            self.grounding.note_unresolved();
            return format!(
                "Tôi chưa thể sửa “{rejected}” thành “{replacement}” vì nó không khớp rõ với quan hệ hiện tại."
            );
        };

        let scene = VietnameseSemanticParser.parse(&corrected);
        let Some(query) = scene.query else {
            self.grounding.note_unresolved();
            return "Tôi đã nhận ra yêu cầu sửa nhưng chưa dựng được quan hệ mới; hãy nói lại câu đầy đủ.".into();
        };

        self.grounding.note_repair();
        self.last_question = Some(corrected.clone());
        self.topic = Some(query.subject.text.clone());
        self.dialogue_goal.bind_relation(&corrected);
        let reply = self.answer_styled(
            &corrected,
            crate::expression_style::ExpressionStyle::Standard,
        );
        self.last_source_snapshot = self.relation_source_snapshot(&corrected);

        let role = if subject_role { "nguyên nhân" } else { "kết quả" };
        format!(
            "Hiểu rồi — tôi sửa {role} từ “{rejected}” thành “{replacement}”. {reply}"
        )
    }

    fn execute_dynamic_dialogue_plan(
        &mut self,
        plan: crate::dynamic_dialogue_intent::DynamicDialoguePlan,
    ) -> String {
        use crate::dynamic_dialogue_intent::DynamicIntent;

        let Some(q)=self.last_question.clone() else {
            return "Chưa có mạch hội thoại hiện tại để ghép các ý định này.".into();
        };

        let new_only=plan.contains(DynamicIntent::NewOnly);
        let doubt=plan.contains(DynamicIntent::Doubt);
        let expand=plan.contains(DynamicIntent::Expand);
        let acknowledge=plan.contains(DynamicIntent::Acknowledge);
        let confirm=plan.contains(DynamicIntent::Confirm);

        if doubt {
            self.dialogue_goal.advance(crate::dialogue_goal_state::ImplicitDialogueGoal::Verify);
        } else if expand {
            self.dialogue_goal.advance(crate::dialogue_goal_state::ImplicitDialogueGoal::Explore);
        }

        let mut deep_review=None;
        if plan.wants_deep_review() {
            deep_review=Some(self.answer_styled(
                &q,
                crate::expression_style::ExpressionStyle::Deep,
            ));
        }

        if new_only {
            let delta=self.response_delta();
            if doubt && delta.starts_with("Chưa có bằng chứng mới") {
                return format!("Tôi đã kiểm tra lại; {delta}");
            }
            return delta;
        }

        let mut parts=Vec::new();
        if acknowledge {
            parts.push("Đã hiểu.".to_string());
        }

        if doubt {
            let deep=deep_review.unwrap_or_else(|| {
                self.answer_styled(&q,crate::expression_style::ExpressionStyle::Deep)
            });
            parts.push(format!(
                "Tôi kiểm tra lại mà không tăng độ chắc chỉ vì bị hỏi lại. {deep}"
            ));
            self.last_source_snapshot=self.relation_source_snapshot(&q);
        } else if expand {
            let deep=deep_review.unwrap_or_else(|| {
                self.answer_styled(&q,crate::expression_style::ExpressionStyle::Deep)
            });
            parts.push(format!("Mở rộng thêm từ cùng mạch bằng chứng: {deep}"));
            self.last_source_snapshot=self.relation_source_snapshot(&q);
        } else if confirm {
            let brief=self.answer_styled(
                &q,
                crate::expression_style::ExpressionStyle::Brief,
            );
            parts.push(format!("Nếu bạn đang xác nhận kết luận vừa rồi: {brief}"));
        }

        if parts.is_empty() {
            "Tôi đã giữ các ý định hội thoại, nhưng chưa có phần trả lời mới cần phát ra.".into()
        } else {
            parts.join(" ")
        }
    }

    fn response_delta(&mut self) -> String {
        let Some(q)=self.last_question.clone() else {
            return "Chưa có câu trả lời gần đây để xác định phần mới.".into();
        };
        let before=self.last_source_snapshot.clone();
        let current=self.relation_source_snapshot(&q);

        let mut added=Vec::new();
        let mut changed=Vec::new();
        let mut removed=Vec::new();

        for (name, sig) in &current {
            match before.iter().find(|(old,_)|old==name) {
                None => added.push(name.clone()),
                Some((_,old_sig)) if old_sig!=sig => changed.push(name.clone()),
                _ => {}
            }
        }
        for (name, _) in &before {
            if !current.iter().any(|(now,_)|now==name) {
                removed.push(name.clone());
            }
        }

        if added.is_empty() && changed.is_empty() && removed.is_empty() {
            return "Chưa có bằng chứng mới liên quan đến quan hệ vừa xét; tôi không lặp lại phần cũ.".into();
        }

        let brief=self.answer_styled(&q,crate::expression_style::ExpressionStyle::Brief);
        self.last_source_snapshot=current;

        let mut delta=Vec::new();
        if !added.is_empty(){delta.push(format!("nguồn thêm: {}",added.join(", ")));}
        if !changed.is_empty(){delta.push(format!("nguồn thay đổi: {}",changed.join(", ")));}
        if !removed.is_empty(){delta.push(format!("nguồn đã rút: {}",removed.join(", ")));}
        format!("Phần mới — {}. Sau khi cập nhật: {brief}",delta.join("; "))
    }

    fn relation_source_snapshot(&self, question: &str) -> Vec<(String,u64)> {
        let scene=VietnameseSemanticParser.parse(question);
        let Some(query)=scene.query else{return Vec::new()};
        let mut out=Vec::new();
        for source in self.sources.iter().filter(|s|s.kind!=SourceKind::Hypothesis) {
            let parsed=VietnameseSemanticParser.parse(&source.text);
            let relevant=parsed.clauses.iter().take(8).any(|clause|{
                clause.subject.id==query.subject.id
                    || clause.object.id==query.object.id
                    || clause.subject.id==query.object.id
                    || clause.object.id==query.subject.id
            });
            if relevant {
                out.push((source.name.clone(),source_signature(source)));
            }
        }
        out.sort_by(|a,b|a.0.cmp(&b.0));
        out
    }

    fn update_topic(&mut self, text: &str) {
        let scene = VietnameseSemanticParser.parse(text);
        self.topic = if scene.clauses.len() == 1 {
            Some(scene.clauses[0].subject.text.clone())
        } else {
            None
        };
    }

    fn answer(&mut self, question: &str) -> String {
        self.answer_internal(question, None, true)
    }

    fn answer_styled(&mut self, question: &str, style: crate::expression_style::ExpressionStyle) -> String {
        self.answer_internal(question, Some(style), false)
    }

    fn answer_restate(&mut self, question: &str) -> String {
        self.answer_internal(question, None, false)
    }

    fn answer_internal(
        &mut self,
        question: &str,
        requested_style: Option<crate::expression_style::ExpressionStyle>,
        remember_turn: bool,
    ) -> String {
        let mut question = question.to_string();
        for prefix in ["no ", "dieu do "] {
            if let Some(rest) = question.strip_prefix(prefix) {
                let Some(topic) = &self.topic else {
                    return "“Nó/điều đó” chưa rõ chỉ đối tượng nào; hãy nêu tên cụ thể.".into();
                };
                question = format!("{topic} {rest}");
                break;
            }
        }
        let reasoner = SemanticReasoner::default();
        let scene = reasoner.parse(&question);
        let Some(query) = &scene.query else {
            return "Tôi chưa phân tích được câu hỏi này; thử “A có dẫn tới B không?”.".into();
        };
        let relation_continuity = self
            .continuity
            .relation_continuity(&query.subject.text, &query.object.text);
        if remember_turn {
            self.topic = Some(query.subject.text.clone());
            self.last_question = Some(question.clone());
            self.continuity
                .remember(&question, &query.subject.text, &query.object.text);
            self.dialogue_goal.bind_relation(&question);
            self.grounding.clear_unresolved();
        }
        let mut labels=std::collections::HashMap::new();
        let mut world = WorldGraph::new(1024, 512);
        let mut consulted = Vec::new();
        for source in &self.sources {
            if source.kind == SourceKind::Hypothesis {
                continue;
            }
            let mut parsed = reasoner.parse(&source.text);
            parsed.query = None;
            parsed.clauses.truncate(8);
            let quality: f32 = if source.kind == SourceKind::Observation {
                0.95
            } else {
                0.75
            };
            for clause in &mut parsed.clauses {
                labels.insert(clause.subject.id,clause.subject.text.clone());
                labels.insert(clause.object.id,clause.object.text.clone());
                clause.confidence *= quality.sqrt();
            }
            VietnameseSemanticParser.ingest(&mut world, &parsed, 0);
            if !parsed.clauses.is_empty() {
                consulted.push(source.name.as_str());
            }
        }
        let (mut answer, weave) = reasoner.answer_scene_with_weave(&world, &scene);
        let (support, opposition, depth, evidence) = match &answer {
            OpenAnswer::Supported { confidence, path, .. } => {
                (*confidence, 0.0, path.len(), path.len().saturating_sub(1))
            }
            OpenAnswer::Opposed { confidence, path, .. } => {
                (0.0, *confidence, path.len(), path.len().saturating_sub(1))
            }
            OpenAnswer::Contradicted {
                support,
                opposition,
                ..
            } => (*support, *opposition, 2, 2),
            OpenAnswer::Counterfactual {
                factual_support,
                counterfactual_support,
                ..
            } => (*factual_support, *counterfactual_support, 2, 2),
            OpenAnswer::Unknown => (0.0, 0.0, 0, 0),
        };
        let assessment = MetacognitiveController.assess(support, opposition, depth, evidence);
        let review = AutonomousCognitiveLoop::default().run(
            query.object.id,
            &answer,
            &assessment,
            0.15,
            evidence,
        );
        match &mut answer {
            OpenAnswer::Supported { confidence, .. } | OpenAnswer::Opposed { confidence, .. } => {
                *confidence = confidence.min(review.final_confidence)
            }
            _ => {}
        }
        let mut current_evidence=Vec::new();
        if let OpenAnswer::Supported{path,..}|OpenAnswer::Opposed{path,..}=&answer {
            for source in self.sources.iter().filter(|s|s.kind!=SourceKind::Hypothesis){
                let parsed=reasoner.parse(&source.text);
                if parsed.clauses.iter().take(8).any(|clause|path.windows(2).any(|pair|pair[0]==clause.subject.id&&pair[1]==clause.object.id)) {
                    current_evidence.push(source.name.clone());
                }
            }
        }
        self.last_evidence=current_evidence;
        let uncertainty = 1.0 - review.final_confidence;
        let style = requested_style.unwrap_or_else(|| {
            crate::adaptive_expression::AdaptiveExpressionSelector
                .choose(&answer, &weave, uncertainty)
                .style
        });
        let generated = GenerativeCognition.render_with_style(&answer, uncertainty, &weave, style);
        let mut text=crate::natural_surface::NaturalSurfaceRealizer.relation_contextual(
            &query.subject.text,
            &query.object.text,
            &generated.text,
            style,
            &weave,
            relation_continuity,
        );
        let explicit_deep = requested_style == Some(crate::expression_style::ExpressionStyle::Deep);
        let suppress_repeated_details = matches!(
            relation_continuity,
            crate::conversation_continuity::RelationContinuity::Repeat
        ) && !explicit_deep;
        if style != crate::expression_style::ExpressionStyle::Brief {
            if suppress_repeated_details {
                text.push_str(" Mạch bằng chứng không đổi so với lượt trước.");
            } else {
                if let OpenAnswer::Supported{path,..}|OpenAnswer::Opposed{path,..}=&answer {
                    let named:Vec<_>=path.iter().filter_map(|id|labels.get(id).cloned()).collect();
                    if named.len()==path.len(){text.push_str(&format!(" Đường suy luận: {}. Đây là quan hệ trong nguồn đã ghi, chưa phải xác minh độc lập.",named.join(" → ")));}
                }
                if !consulted.is_empty() {
                    text.push_str(&format!(" Đã xét các nguồn: {}.", consulted.join(", ")));
                }
            }
        }
        if remember_turn {
            self.last_source_snapshot=self.relation_source_snapshot(&question);
        }
        text
    }

    fn plan(&self, start: &str, goal: &str) -> String {
        let (Some(start), Some(goal)) = (facts(start), goal_facts(goal)) else {
            return invalid().0;
        };
        let mut model = WorldModel::default();
        for skill in self.skills.iter().filter(|s| !s.blocked) {
            model.add_transition(TransitionModel {
                action: concept_id(&skill.name),
                requires: skill.requires.clone(),
                adds: skill.adds.clone(),
                removes: Vec::new(),
                utility: 0.0,
                cost: 0.1,
                confidence: ((2.0 + f32::from(skill.successes))
                    / (3.0 + f32::from(skill.successes) + f32::from(skill.failures)))
                .min(0.95),
            });
        }
        let Some(plan) = DeliberativePlanner.plan(&model, &SimState::new(start), &goal) else {
            return "Chưa tìm được kế hoạch trong giới hạn hiện tại; cần thêm kỹ năng hoặc điều kiện.".into();
        };
        if !plan.reached_goal {
            return "Chỉ tìm được phương án một phần; chưa đạt mục tiêu nên chưa đề xuất thực thi."
                .into();
        }
        if plan.actions.is_empty() {
            return "Các điều kiện đã đáp ứng mục tiêu; không cần thêm bước.".into();
        }
        let names: Vec<_> = plan
            .actions
            .iter()
            .filter_map(|id| {
                self.skills
                    .iter()
                    .find(|s| concept_id(&s.name) == *id)
                    .map(|s| s.name.as_str())
            })
            .collect();
        format!("Kế hoạch mô phỏng: {}. Đạt mục tiêu trong mô hình đã khai báo; chưa thực thi trên thiết bị.", names.join(" → "))
    }

    pub fn executable_skill(&self, name: &str) -> Option<crate::capability::DeviceAction> {
        let name = normalize(name).trim().to_string();
        self.skills.iter().find(|s| s.name == name && !s.blocked)?;
        self.bindings
            .iter()
            .find(|b| b.0 == name)
            .map(|b| b.1.clone())
    }

    pub fn executable_plan(
        &self,
        input: &str,
    ) -> Option<Vec<(String, crate::capability::DeviceAction)>> {
        let (start, goal) = input.split_once("->")?;
        let (start, goal) = (normalize(start), normalize(goal));
        let mut model = WorldModel::default();
        for skill in self
            .skills
            .iter()
            .filter(|s| !s.blocked && self.bindings.iter().any(|b| b.0 == s.name))
        {
            model.add_transition(TransitionModel {
                action: concept_id(&skill.name),
                requires: skill.requires.clone(),
                adds: skill.adds.clone(),
                removes: Vec::new(),
                utility: 0.0,
                cost: 0.1,
                confidence: 0.8,
            });
        }
        let plan = DeliberativePlanner.plan(
            &model,
            &SimState::new(facts(start.trim())?),
            &goal_facts(goal.trim())?,
        )?;
        if !plan.reached_goal || plan.actions.is_empty() {
            return None;
        }
        plan.actions
            .iter()
            .map(|id| {
                let skill = self.skills.iter().find(|s| concept_id(&s.name) == *id)?;
                Some((skill.name.clone(), self.executable_skill(&skill.name)?))
            })
            .collect()
    }

    /// Adapter failures have a separate bounded slot per skill, even when the user journal is full.
    pub fn record_execution_failure(&mut self, name: &str) -> bool {
        let Some(skill) = self.skills.iter_mut().find(|s| s.name == name) else {
            return false;
        };
        if !self.execution_failures.iter().any(|n| n == name) {
            self.execution_failures.push(name.into());
            skill.failures = skill.failures.saturating_add(1);
        }
        skill.blocked = true;
        true
    }

    pub fn journal_len(&self) -> usize {
        self.journal.len()
    }
    pub fn export(&self) -> String {
        self.journal
            .iter()
            .map(|line| format!("J131\t{}", hex(line.as_bytes())))
            .chain(
                self.execution_failures
                    .iter()
                    .map(|name| format!("B132\t{}", hex(name.as_bytes()))),
            )
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Decode atomically and replay only bounded cognition commands, never tools.
    pub fn restore(&mut self, text: &str) -> bool {
        let lines: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("J131\t"))
            .take(MAX_JOURNAL + 1)
            .collect();
        if lines.len() > MAX_JOURNAL {
            return false;
        }
        let mut next = Self::default();
        for line in lines {
            let Some(command) = unhex(line) else {
                return false;
            };
            let before = next.journal.len();
            if next.handle(&command).is_none() || next.journal.len() != before + 1 {
                return false;
            }
        }
        for line in text.lines().filter_map(|l| l.strip_prefix("B132\t")) {
            let Some(name) = unhex(line) else {
                return false;
            };
            if next.execution_failures.len() >= MAX_SKILLS
                || next.execution_failures.contains(&name)
                || !next.record_execution_failure(&name)
            {
                return false;
            }
        }
        *self = next;
        true
    }
}

fn source_signature(source: &Source) -> u64 {
    let mut h=0xcbf29ce484222325_u64;
    for b in source.name.bytes().chain(std::iter::once(0)).chain(source.text.bytes()) {
        h^=u64::from(b);
        h=h.wrapping_mul(0x100000001b3);
    }
    h^=match source.kind {
        SourceKind::Observation=>1,
        SourceKind::Report=>2,
        SourceKind::Hypothesis=>3,
    };
    h
}

fn valid_name(s: &str) -> bool {
    !s.trim().is_empty() && s.chars().count() <= 80
}
fn invalid() -> (String, bool) {
    (
        "Cú pháp chưa đủ rõ; hãy kiểm tra tên, dấu hai chấm và các điều kiện.".into(),
        false,
    )
}
fn full() -> (String, bool) {
    (
        "Đã đạt giới hạn bộ nhớ cấu trúc; chưa ghi thêm dữ liệu.".into(),
        false,
    )
}
fn facts(text: &str) -> Option<Vec<u64>> {
    let items: Vec<_> = text.split(',').map(str::trim).collect();
    if items.is_empty() || items.len() > 8 || items.iter().any(|s| !valid_name(s)) {
        return None;
    }
    Some(items.into_iter().map(concept_id).collect())
}
fn goal_facts(text: &str) -> Option<GoalSpec> {
    let (desired, avoid) = text.split_once("; tranh ").unwrap_or((text, ""));
    Some(GoalSpec {
        desired: facts(desired)?,
        avoid: if avoid.is_empty() {
            Vec::new()
        } else {
            facts(avoid)?
        },
    })
}
pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn unhex(text: &str) -> Option<String> {
    if text.len() > MAX_INPUT * 8 || !text.len().is_multiple_of(2) || !text.is_ascii() {
        return None;
    }
    let bytes: Option<Vec<u8>> = text
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| {
            let s = std::str::from_utf8(p).ok()?;
            u8::from_str_radix(s, 16).ok()
        })
        .collect();
    String::from_utf8(bytes?).ok()
}
