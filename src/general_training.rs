//! CPU curriculum for the existing symbolic learner, not foundation-model training.
use crate::integrated_cognition::IntegratedCognition;
use std::time::Instant;
const DOMAINS: &[&str] = &["tep", "sanpham", "hinhanh", "dulieu", "thietbi", "congviec"];
pub fn curriculum() -> Vec<String> {
    let mut rows=Vec::new();
    for d in DOMAINS {
        rows.extend([
            format!("Nguồn hoc144{d}a: hoc144{d}vao gây ra hoc144{d}giua."),
            format!("Nguồn hoc144{d}b: hoc144{d}giua gây ra hoc144{d}ra."),
            format!("Kỹ năng hoc144{d}doc: hoc144{d}vao -> hoc144{d}giua"),
            format!("Kỹ năng hoc144{d}kiem: hoc144{d}giua -> hoc144{d}ra"),
            format!("Ví dụ hoc144{d}: mau1 -> hoc144{d}dat"),
            format!("Ví dụ hoc144{d}: mau2 -> hoc144{d}dat"),
        ]);
    }
    rows
}
#[derive(Debug)]
pub struct Score {pub correct:usize,pub total:usize,pub answers:Vec<String>}
/// Evaluation reads clones. Held-out queries/instances are never training records.
pub fn evaluate(model:&IntegratedCognition)->Score {
    let mut score=Score{correct:0,total:0,answers:Vec::new()};
    for d in DOMAINS {
        let queries=[
            (format!("Hỏi: hoc144{d}vao có dẫn tới hoc144{d}ra không?"),"2 mắt xích".to_string()),
            (format!("Lập kế hoạch: hoc144{d}vao -> hoc144{d}ra"),format!("hoc144{d}doc → hoc144{d}kiem")),
            (format!("Suy rộng: hoc144{d} cho maugiu144"),format!("có thể đạt hoc144{d}dat")),
            (format!("Hỏi: hoc144{d}ra có gây ra hoc144{d}chuabiet không?"),"chưa".to_string()),
        ];
        for (query,expected) in queries {
            let mut probe=model.clone();let answer=probe.handle(&query).unwrap_or_default();
            score.total+=1;score.correct+=usize::from(answer.contains(&expected));score.answers.push(format!("{query}\t{answer}"));
        }
        let mut revised=model.clone();
        revised.handle(&format!("Đính chính hoc144{d}b: hoc144{d}giua ngăn hoc144{d}ra."));
        let answer=revised.handle(&format!("Hỏi: hoc144{d}vao có gây ra hoc144{d}ra không?")).unwrap_or_default();
        score.total+=1;score.correct+=usize::from(answer.contains("phản đối"));score.answers.push(format!("revision {d}\t{answer}"));
        let mut challenged=model.clone();
        challenged.handle(&format!("Phản ví dụ hoc144{d}: maubacbo144 -> hoc144{d}dat"));
        let answer=challenged.handle(&format!("Suy rộng: hoc144{d} cho maugiu144")).unwrap_or_default();
        score.total+=1;score.correct+=usize::from(answer.contains("tạm giữ"));score.answers.push(format!("counterexample {d}\t{answer}"));
    }
    score
}
pub struct TrainingRun {pub candidate:IntegratedCognition,pub report:String,pub qualified:bool}
pub fn train(base:&IntegratedCognition)->TrainingRun {
    let start=Instant::now();let before=evaluate(base);let mut candidate=base.clone();let mut learned=0;
    let rows=curriculum();
    for row in &rows {let old=candidate.export();candidate.handle(row);learned+=usize::from(candidate.export()!=old);}
    let after=evaluate(&candidate);let checkpoint=candidate.export();let mut restored=IntegratedCognition::default();
    let reload=restored.restore(&checkpoint) && restored.export()==checkpoint && evaluate(&restored).answers==after.answers;
    let qualified=learned==rows.len() && after.correct==after.total && after.correct>before.correct && reload;
    let report=format!("BIA_TRAINING_144\nmethod=symbolic_source_example_skill_learning\nprovenance=synthetic_curriculum_authored_in_repository\ndomains={}\ntraining_records={}\naccepted_records={}\nbaseline={}/{}\nheldout_after={}/{}\ncheckpoint_reload={}\nqualified_for_this_curriculum={}\nelapsed_ms={}\ngeneral_intelligence_proven=false\nreal_world_skill_quality_proven=false\nexternal_execution=false\n{}\n",
        DOMAINS.len(),rows.len(),learned,before.correct,before.total,after.correct,after.total,reload,qualified,start.elapsed().as_millis(),after.answers.join("\n"));
    TrainingRun{candidate,report,qualified}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn heldout_is_read_only_and_training_persists(){let base=IntegratedCognition::default();let run=train(&base);assert!(run.qualified,"{}",run.report);assert_eq!(base.export(),IntegratedCognition::default().export());let before=run.candidate.export();evaluate(&run.candidate);assert_eq!(before,run.candidate.export());assert!(!before.contains("maugiu144"));}
 #[test]fn repeated_training_does_not_fake_improvement(){let first=train(&IntegratedCognition::default());let again=train(&first.candidate);assert!(!again.qualified);assert_eq!(first.candidate.export(),again.candidate.export());}
}
