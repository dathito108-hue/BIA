#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurriculumDomain { Language, Commonsense, Causality, Planning, ToolUse, SelfCorrection }

#[derive(Clone, Debug, PartialEq)]
pub struct TrialResult {
    pub domain:CurriculumDomain,
    pub correct:bool,
    pub confidence:f32,
    pub latency_ms:u32,
    pub memory_kb:u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurriculumScore {
    pub accuracy:f32,
    pub calibration_error:f32,
    pub efficiency:f32,
    pub passed:bool,
}

pub fn score(results:&[TrialResult], min_accuracy:f32)->CurriculumScore {
    if results.is_empty(){return CurriculumScore{accuracy:0.0,calibration_error:1.0,efficiency:0.0,passed:false}}
    let accuracy=results.iter().filter(|r|r.correct).count() as f32/results.len() as f32;
    let calibration_error=results.iter().map(|r|{
        let y=if r.correct{1.0}else{0.0};
        (r.confidence.clamp(0.0,1.0)-y).abs()
    }).sum::<f32>()/results.len() as f32;
    let efficiency=results.iter().map(|r|1.0/(1.0+r.latency_ms as f32/1000.0+r.memory_kb as f32/1_000_000.0)).sum::<f32>()/results.len() as f32;
    CurriculumScore{accuracy,calibration_error,efficiency,passed:accuracy>=min_accuracy && calibration_error<=0.35}
}
