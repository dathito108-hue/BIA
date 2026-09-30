use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use crate::{
    encode_action, read_file, write_atomic, BiaDca, BiaDcaConfig, DeviceState, OfflineMobileBia,
};

static RUNTIME: OnceLock<Mutex<OfflineMobileBia>> = OnceLock::new();

fn runtime() -> &'static Mutex<OfflineMobileBia> {
    RUNTIME.get_or_init(|| Mutex::new(OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()))))
}

fn java_string(env: &mut JNIEnv, value: String) -> jstring {
    env.new_string(value)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeChat(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
    timestamp: jlong,
    battery: jfloat,
    thermal: jfloat,
    load: jfloat,
    memory_mb: jint,
) -> jstring {
    let text = env
        .get_string(&input)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();

    let device = DeviceState {
        battery: battery.clamp(0.0, 1.0),
        thermal: thermal.clamp(0.0, 1.0),
        load: load.clamp(0.0, 1.0),
        available_memory_mb: memory_mb.max(64) as u32,
    };

    let reply = runtime()
        .lock()
        .ok()
        .and_then(|mut app| app.converse(&text, timestamp.max(0) as u64, device))
        .map(|r| r.text)
        .unwrap_or_else(|| "BIA chưa hình thành được phản hồi từ cảnh hiện tại.".to_string());

    java_string(&mut env, reply)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeImmediateToken(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input = env
        .get_string(&input)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let token = runtime()
        .lock()
        .ok()
        .and_then(|mut app| app.immediate_tokens(&input).into_iter().next())
        .map(|t| t.text)
        .unwrap_or_default();
    java_string(&mut env, token)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativePendingAction(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let encoded = runtime()
        .lock()
        .ok()
        .and_then(|app| app.pending_action().map(encode_action))
        .unwrap_or_default();
    java_string(&mut env, encoded)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeClaimAction(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    timestamp: jlong,
) -> jboolean {
    let Some(id) = env
        .get_string(&id)
        .ok()
        .and_then(|s| s.to_string_lossy().parse::<u64>().ok())
    else {
        return 0;
    };
    runtime()
        .lock()
        .map(|mut app| u8::from(app.claim_approved_action(id, timestamp.max(0) as u64)))
        .unwrap_or(0)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeCompleteAction(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    success: jboolean,
    timestamp: jlong,
) -> jboolean {
    let Some(id) = env
        .get_string(&id)
        .ok()
        .and_then(|s| s.to_string_lossy().parse::<u64>().ok())
    else {
        return 0;
    };
    runtime()
        .lock()
        .map(|mut app| {
            u8::from(app.complete_device_action(id, success != 0, timestamp.max(0) as u64))
        })
        .unwrap_or(0)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeCancelAction(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
) -> jboolean {
    let Some(id) = env
        .get_string(&id)
        .ok()
        .and_then(|s| s.to_string_lossy().parse::<u64>().ok())
    else {
        return 0;
    };
    runtime()
        .lock()
        .map(|mut app| u8::from(app.cancel_device_action(id)))
        .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeExportContinuity(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let state = runtime()
        .lock()
        .map(|app| app.continuity_export())
        .unwrap_or_default();
    java_string(&mut env, state)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeImportContinuity(
    mut env: JNIEnv,
    _class: JClass,
    state: JString,
) -> jboolean {
    let state = match env.get_string(&state) {
        Ok(value) => value.to_string_lossy().into_owned(),
        Err(_) => return 0,
    };
    runtime()
        .lock()
        .map(|mut app| if app.continuity_import(&state) { 1 } else { 0 })
        .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeIngestContent(
    mut env: JNIEnv,
    _class: JClass,
    source: JString,
    kind: jint,
    content: JString,
    timestamp: jlong,
    confidence: jfloat,
) -> jint {
    let source = env
        .get_string(&source)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let content = env
        .get_string(&content)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let provenance = match kind {
        1 => crate::ProvenanceKind::SharedText,
        2 => crate::ProvenanceKind::LocalDocument,
        3 => crate::ProvenanceKind::WebExcerpt,
        4 => crate::ProvenanceKind::System,
        _ => crate::ProvenanceKind::User,
    };
    runtime()
        .lock()
        .map(|mut app| {
            app.ingest_content(
                &source,
                provenance,
                &content,
                timestamp.max(0) as u64,
                confidence.clamp(0.0, 1.0),
            ) as jint
        })
        .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeBenchmarkV12(
    mut env: JNIEnv,
    _class: JClass,
    iterations: jint,
) -> jstring {
    let loops = iterations.clamp(100, 20_000) as usize;
    let start = Instant::now();
    let mut tokens = 0usize;

    if let Ok(mut app) = runtime().lock() {
        for i in 0..loops {
            let input = match i % 5 {
                0 => "Mở YouTube",
                1 => "Tìm web Phật giáo Trúc Lâm",
                2 => "Tại sao cần provenance",
                3 => "Suy luận logic trừu tượng",
                _ => "Không thực thi nếu chưa xác nhận",
            };
            tokens += app.decoder.generate(input, 8).tokens.len();
        }
        let elapsed = start.elapsed();
        let ns_per_token = elapsed.as_nanos() / tokens.max(1) as u128;
        let tokens_per_sec = if elapsed.as_nanos() == 0 {
            0
        } else {
            (tokens as u128 * 1_000_000_000u128) / elapsed.as_nanos()
        };
        return java_string(
            &mut env,
            format!(
                "V12 device benchmark: loops={loops}, tokens={tokens}, elapsed_ms={}, ns/token={ns_per_token}, tokens/s={tokens_per_sec}, vocab={}",
                elapsed.as_millis(),
                app.learned_vocab_len()
            ),
        );
    }

    java_string(&mut env, "V12 benchmark unavailable".to_string())
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeStressV14(
    mut env: JNIEnv,
    _class: JClass,
    iterations: jint,
) -> jstring {
    let loops = iterations.clamp(100, 50_000) as usize;
    let report = crate::run_v14_stress(loops);
    java_string(
        &mut env,
        format!(
            "V14 stress: pass={}, loops={}, bounded={}/{}, state={}/{}, noise={}/{}, deterministic={}/{}, elapsed_ms={}, ns/iter={}",
            report.passed(),
            report.iterations,
            report.bounded_token_passes,
            report.iterations,
            report.finite_state_passes,
            report.iterations,
            report.noise_passes,
            report.iterations,
            report.deterministic_replay_passes,
            report.iterations,
            report.elapsed.as_millis(),
            report.ns_per_iteration()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeReasoningV15(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v15_reasoning_evaluation();
    java_string(
        &mut env,
        format!(
            "V15 reasoning: pass={}, cases={}, multi_hop={}/{}, contradiction={}/{}, revision={}/{}, persistence={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.multi_hop_passes,
            report.cases,
            report.contradiction_passes,
            report.cases,
            report.revision_passes,
            report.cases,
            report.causal_persistence_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeGeneralizeV16(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v16_generalization_evaluation();
    java_string(
        &mut env,
        format!(
            "V16 generalization: pass={}, cases={}, long_chain={}/{}, distractor={}/{}, counterfactual={}/{}, reversal={}/{}, persistence={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.long_chain_passes,
            report.cases,
            report.distractor_passes,
            report.cases,
            report.counterfactual_passes,
            report.cases,
            report.reversal_passes,
            report.cases,
            report.persistence_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeOpenReasoningV18(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v18_open_reasoning_evaluation();
    java_string(
        &mut env,
        format!(
            "V18 open reasoning: pass={}, cases={}, parse={}/{}, composition={}/{}, counterfactual={}/{}, contradiction={}/{}, paraphrase={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.parse_passes,
            report.cases,
            report.composition_passes,
            report.cases,
            report.counterfactual_passes,
            report.cases,
            report.contradiction_passes,
            report.cases,
            report.paraphrase_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeDeepIntelligenceV21(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v21_deep_intelligence_evaluation();
    java_string(
        &mut env,
        format!(
            "V21 deep intelligence: pass={}, cases={}, abstraction={}/{}, analogy={}/{}, induction={}/{}, composition={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.abstraction_passes,
            report.cases,
            report.analogy_passes,
            report.cases,
            report.induction_passes,
            report.cases,
            report.compositional_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeEmergentV25(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v25_emergent_intelligence_evaluation();
    java_string(
        &mut env,
        format!(
            "V25 emergent: pass={}, cases={}, episodic={}/{}, discovery={}/{}, rules={}/{}, competition={}/{}, multidomain={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.episodic_passes,
            report.cases,
            report.discovery_passes,
            report.cases,
            report.rule_passes,
            report.cases,
            report.competition_passes,
            report.cases,
            report.multidomain_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeAutonomyV31(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v31_autonomous_knowledge_evaluation();
    java_string(
        &mut env,
        format!(
            "V31 autonomy: pass={}, cases={}, hierarchy={}/{}, hypothesis={}/{}, falsification={}/{}, meta_rule={}/{}, governance={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.hierarchy_passes,
            report.cases,
            report.hypothesis_passes,
            report.cases,
            report.falsification_passes,
            report.cases,
            report.meta_rule_passes,
            report.cases,
            report.governance_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeDeliberationV37(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v37_deliberation_evaluation();
    java_string(
        &mut env,
        format!(
            "V37 deliberation: pass={}, cases={}, prediction={}/{}, planning={}/{}, avoidance={}/{}, replan={}/{}, bounded={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.prediction_passes,
            report.cases,
            report.planning_passes,
            report.cases,
            report.avoidance_passes,
            report.cases,
            report.replan_passes,
            report.cases,
            report.bounded_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeMaxIntelligenceV45(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v45_max_intelligence_evaluation();
    java_string(
        &mut env,
        format!(
            "V45 max intelligence: pass={}, cases={}, meta={}/{}, calibration={}/{}, evidence={}/{}, recursive={}/{}, routing={}/{}, transfer={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.metacognition_passes,
            report.cases,
            report.calibration_passes,
            report.cases,
            report.evidence_passes,
            report.cases,
            report.recursive_passes,
            report.cases,
            report.compute_routing_passes,
            report.cases,
            report.transfer_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeLearnedSemanticV61(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v61_learned_semantic_evaluation();
    java_string(
        &mut env,
        format!(
            "V61 semantic: pass={}, cases={}, embedding={}/{}, latent_memory={}/{}, relation={}/{}, retrieval={}/{}, compression={}/{}, hybrid={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.embedding_passes,
            report.cases,
            report.latent_memory_passes,
            report.cases,
            report.relation_passes,
            report.cases,
            report.vector_retrieval_passes,
            report.cases,
            report.compression_passes,
            report.cases,
            report.hybrid_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeContinualV81(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v81_continual_generative_evaluation();
    java_string(
        &mut env,
        format!(
            "V81 continual: pass={}, cases={}, continual={}/{}, anchor={}/{}, composition={}/{}, symbol={}/{}, consolidation={}/{}, generation={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.continual_passes,
            report.cases,
            report.anchor_passes,
            report.cases,
            report.composition_passes,
            report.cases,
            report.symbol_passes,
            report.cases,
            report.consolidation_passes,
            report.cases,
            report.generation_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeAutonomousLoopV101(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::run_v101_autonomous_loop_evaluation();
    java_string(
        &mut env,
        format!(
            "V101 loop: pass={}, cases={}, questions={}/{}, critic={}/{}, loop={}/{}, self_review={}/{}, idle={}/{}, authority={}/{}, accuracy={:.3}, elapsed_us={}",
            report.passed(),
            report.cases,
            report.question_passes,
            report.cases,
            report.critic_passes,
            report.cases,
            report.loop_passes,
            report.cases,
            report.self_review_passes,
            report.cases,
            report.idle_passes,
            report.cases,
            report.authority_passes,
            report.cases,
            report.accuracy(),
            report.elapsed.as_micros()
        ),
    )
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeEvidenceV130(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::evidence_evaluation::run_evidence_evaluation();
    java_string(&mut env, format!(
        "V130 evidence: pass={}, cases={}, chains={}, conflicts={}, unchanged_memory={}, top3_baseline={}, elapsed_ms={}",
        report.passed(), report.cases, report.chain_passes,
        report.contradiction_passes, report.isolation_passes,
        report.top_three_chain_passes, report.elapsed.as_millis()
    ))
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeIntegratedV131(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let report = crate::integrated_evaluation::run_integrated_evaluation();
    java_string(&mut env, format!(
        "V131 integrated: pass={}, cases={}, language={}, revision={}, feedback={}, planning={}, transfer={}, continuity={}, elapsed_ms={}",
        report.passed(), report.cases, report.language, report.revision,
        report.feedback, report.planning, report.transfer, report.continuity,
        report.elapsed.as_millis()
    ))
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeCycleCount(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    runtime()
        .lock()
        .map(|app| app.bia.cycle() as jlong)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeStatus(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let status = runtime()
        .lock()
        .map(|app| {
            let goal = app
                .goals
                .active()
                .map(|g| format!("  •  Mục tiêu {:.0}%", g.progress * 100.0))
                .unwrap_or_default();
            let queue = if app.queue_len() > 0 {
                format!("  •  Hàng đợi {}", app.queue_len())
            } else {
                String::new()
            };
            format!(
                "Chu kỳ {}  •  Cảnh {}  •  Chủng tử {}  •  Nguồn {}  •  Từ vựng {}{}{}",
                app.bia.cycle(),
                app.bia.world.len(),
                app.bia.memory.len(),
                app.knowledge.len(),
                app.learned_vocab_len(),
                goal,
                queue
            )
        })
        .unwrap_or_else(|_| "BIA đang bận".to_string());
    java_string(&mut env, status)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeSave(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jboolean {
    let path = match env.get_string(&path) {
        Ok(value) => value.to_string_lossy().into_owned(),
        Err(_) => return 0,
    };

    runtime()
        .lock()
        .ok()
        .and_then(|app| write_atomic(Path::new(&path), &app.bia.snapshot()).ok())
        .map(|_| 1)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeLoad(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jboolean {
    let path = match env.get_string(&path) {
        Ok(value) => value.to_string_lossy().into_owned(),
        Err(_) => return 0,
    };

    let snapshot = match read_file(Path::new(&path)) {
        Ok(snapshot) => snapshot,
        Err(_) => return 0,
    };

    match runtime().lock() {
        Ok(mut app) => {
            let continuity = app.continuity_export();
            if app.tools.busy() {return 0}
            *app = OfflineMobileBia::new(BiaDca::from_snapshot(BiaDcaConfig::default(), snapshot));
            let _ = app.continuity_import(&continuity);
            1
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeApprovalSnapshot(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let value = runtime()
        .lock()
        .map(|app| app.approval_snapshot())
        .unwrap_or_default();
    java_string(&mut env, value)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeApproveExecution(
    mut env: JNIEnv,
    _class: JClass,
    snapshot: JString,
    mode: jint,
    timestamp: jlong,
) -> jboolean {
    let mode = match mode {
        1 => crate::execution_authority::ApprovalMode::Step,
        2 => crate::execution_authority::ApprovalMode::Batch,
        3 => crate::execution_authority::ApprovalMode::Session,
        _ => return 0,
    };
    let Ok(snapshot) = env
        .get_string(&snapshot)
        .map(|s| s.to_string_lossy().into_owned())
    else {
        return 0;
    };
    runtime()
        .lock()
        .map(|mut app| u8::from(app.approve_execution(&snapshot, mode, timestamp.max(0) as u64)))
        .unwrap_or(0)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeIsActionApproved(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    timestamp: jlong,
) -> jboolean {
    let Some(id) = env
        .get_string(&id)
        .ok()
        .and_then(|s| s.to_string_lossy().parse::<u64>().ok())
    else {
        return 0;
    };
    runtime()
        .lock()
        .map(|app| u8::from(app.execution_is_approved(id, timestamp.max(0) as u64)))
        .unwrap_or(0)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeExecutionPermissionStatus(
    mut env: JNIEnv,
    _class: JClass,
    timestamp: jlong,
) -> jstring {
    let value = runtime()
        .lock()
        .map(|app| app.execution_permission_status(timestamp.max(0) as u64))
        .unwrap_or_default();
    java_string(&mut env, value)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeStopAutomation(
    _env: JNIEnv,
    _class: JClass,
) {
    if let Ok(mut app) = runtime().lock() {
        app.stop_automation();
    }
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeRevokeApproval(
    _env: JNIEnv,
    _class: JClass,
) {
    if let Ok(mut app) = runtime().lock() {
        app.revoke_execution_approval();
    }
}


#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_GameNative_reset(_env: JNIEnv, _class: JClass) {
    if let Ok(mut app) = runtime().lock() {
        app.game.reset();
    }
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_GameNative_observe(
    env: JNIEnv,
    _class: JClass,
    pixels: jni::objects::JIntArray,
    width: jint,
    height: jint,
    params: jni::objects::JFloatArray,
    timestamp: jlong,
) -> jni::sys::jfloatArray {
    let mut output = [0.0f32; 9];
    let count = if (8..=512).contains(&width) && (8..=512).contains(&height) {
        (width * height) as usize
    } else {
        0
    };
    let mut p = [0.0f32; 9];
    if count > 0
        && env.get_array_length(&pixels).ok() == Some(count as i32)
        && env.get_array_length(&params).ok() == Some(9)
        && env.get_float_array_region(&params, 0, &mut p).is_ok()
    {
        let mut data = vec![0i32; count];
        if env.get_int_array_region(&pixels, 0, &mut data).is_ok()
            && p.iter().all(|x| x.is_finite())
        {
            let profile = crate::game_agent::GameProfile {
                roi: [p[0], p[1], p[2], p[3]],
                rgb: [p[4] as u8, p[5] as u8, p[6] as u8],
                tolerance: p[7] as u8,
                moba: p[8] > 0.5,
            };
            if let Ok(mut app) = runtime().lock() {
                let data: Vec<u32> = data.into_iter().map(|x| x as u32).collect();
                let d = app.game.observe(
                    &data,
                    width as usize,
                    height as usize,
                    &profile,
                    timestamp.max(0) as u64,
                );
                if let Some(t) = d.target {
                    output[0] = t[0];
                    output[1] = t[1];
                }
                output[2] = d.movement[0];
                output[3] = d.movement[1];
                output[4] = d.aim[0];
                output[5] = d.aim[1];
                output[6] = u8::from(d.fire) as f32;
                output[7] = u8::from(d.skill) as f32;
                output[8] = u8::from(d.confirmed) as f32;
            }
        }
    }
    let Ok(array) = env.new_float_array(9) else {
        return std::ptr::null_mut();
    };
    if env.set_float_array_region(&array, 0, &output).is_err() {
        return std::ptr::null_mut();
    }
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_TradingNative_analyzeCore(
    mut env: JNIEnv, _class:JClass, candles:jni::objects::JDoubleArray,
    price:jni::sys::jdouble, event_ms:jlong, now_ms:jlong, connected:jboolean,
)->jstring {
    let length=env.get_array_length(&candles).unwrap_or(0);
    let value=if length>0 && length<=600 && length%5==0 && event_ms>=0 && now_ms>0 {
        let mut data=vec![0.0;length as usize];
        if env.get_double_array_region(&candles,0,&mut data).is_err(){"CHẶN PHÂN TÍCH: lỗi dữ liệu JNI".into()}else{
            let bars:Vec<_>=data.chunks_exact(5).map(|v|crate::trading_live::Candle{close_ms:if v[0].is_finite() && v[0]>0.0 {v[0] as u64}else{0},open:v[1],high:v[2],low:v[3],close:v[4]}).collect();
            crate::trading_live::analyze(&bars,price,event_ms as u64,now_ms as u64,connected!=0).text()
        }
    }else{"CHẶN PHÂN TÍCH: đang chờ nến từ nguồn".into()};
    java_string(&mut env,value)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_TradingNative_qualityCore(
    mut env: JNIEnv, _class:JClass, points:jni::objects::JDoubleArray,
    now_ms:jlong, cost:jni::sys::jdouble,
)->jstring {
    let n=env.get_array_length(&points).unwrap_or(0);
    let value=if (42..=480).contains(&n) && n%2==0 && now_ms>0 {
        let mut data=vec![0.0;n as usize];
        if env.get_double_array_region(&points,0,&mut data).is_err(){"Lỗi quan sát JNI".into()}else{
            let observations:Vec<_>=data.chunks_exact(2).map(|v|(if v[0].is_finite() && v[0]>0.0 {v[0] as u64}else{0},v[1])).collect();
            crate::trading_quality::text(&observations,now_ms as u64,cost)
        }
    }else{"Chờ ít nhất 21 báo giá thật; chưa đủ bằng chứng".into()};
    java_string(&mut env,value)
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_ProductNative_compile(mut env: JNIEnv, _class: JClass, spec: JString) -> jstring {
    let value=env.get_string(&spec).map(|s|s.to_string_lossy().into_owned()).unwrap_or_default();
    java_string(&mut env,crate::product_studio::compile_wire(&value))
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_ProductNative_compatibility(mut env: JNIEnv, _class: JClass, before: JString, after: JString) -> jstring {
    let a=env.get_string(&before).map(|s|s.to_string_lossy().into_owned()).unwrap_or_default();
    let b=env.get_string(&after).map(|s|s.to_string_lossy().into_owned()).unwrap_or_default();
    let result=crate::product_studio::Spec::parse(&a).and_then(|a|crate::product_studio::Spec::parse(&b).and_then(|b|crate::product_studio::compatible(&a,&b)));
    java_string(&mut env,match result{Ok(s)=>format!("OK: {s}"),Err(s)=>format!("CHẶN: {s}")})
}

#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_CoreSkills_begin(mut env:JNIEnv,_class:JClass,skill:JString)->jlong{
 let Ok(skill)=env.get_string(&skill) else{return 0};
 runtime().lock().ok().and_then(|mut app|app.tools.begin(&skill.to_string_lossy())).unwrap_or(0) as jlong
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_CoreSkills_finish(_env:JNIEnv,_class:JClass,id:jlong,success:jboolean)->jboolean{
 runtime().lock().map(|mut app|u8::from(id>0 && app.tools.finish(id as u64,success!=0))).unwrap_or(0)
}
#[no_mangle]
pub extern "system" fn Java_com_bia_mobile_CoreSkills_report(mut env:JNIEnv,_class:JClass)->jstring{
 let text=runtime().lock().map(|app|app.tools.report()).unwrap_or_else(|_|"Lõi không sẵn sàng".into());java_string(&mut env,text)
}
