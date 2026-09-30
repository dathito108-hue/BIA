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
                return Some(self.answer(&q));
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
        let Some(frame)=understand(expanded.as_deref().unwrap_or(input)) else {self.last_question=None;self.topic=None;return None};
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
                self.answer(&q);
                let mut out=String::from("Nguồn của đường suy luận vừa xét (thông tin được báo lại, chưa xác minh độc lập):");
                for name in &self.last_evidence {if let Some(source)=self.sources.iter().find(|s|&s.name==name){out.push_str(&format!("\n{}: {}",source.name,source.text));}}
                if self.last_evidence.is_empty(){out.push_str(" chưa xác định được đường bằng chứng đơn nhất; hãy xem phần giải thích.");}
                Some(out)
            },
            Frame::Reply(text)=>{if text.starts_with("Tôi chưa") {self.last_question=None;self.topic=None;}Some(text.into())},
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
        if remember_turn {
            self.last_evidence=current_evidence;
        }
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
