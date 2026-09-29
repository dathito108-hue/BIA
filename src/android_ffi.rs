use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use crate::{
    encode_action, read_file, write_atomic, BiaDca, BiaDcaConfig, DeviceState,
    OfflineMobileBia,
};

static RUNTIME: OnceLock<Mutex<OfflineMobileBia>> = OnceLock::new();

fn runtime() -> &'static Mutex<OfflineMobileBia> {
    RUNTIME.get_or_init(|| {
        Mutex::new(OfflineMobileBia::new(BiaDca::new(
            BiaDcaConfig::default(),
        )))
    })
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
pub extern "system" fn Java_com_bia_mobile_MainActivity_nativeResolveAction(
    _env: JNIEnv,
    _class: JClass,
    success: jboolean,
    timestamp: jlong,
) {
    if let Ok(mut app) = runtime().lock() {
        app.resolve_pending_action(success != 0, timestamp.max(0) as u64);
    }
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
            *app = OfflineMobileBia::new(BiaDca::from_snapshot(
                BiaDcaConfig::default(),
                snapshot,
            ));
            let _ = app.continuity_import(&continuity);
            1
        }
        Err(_) => 0,
    }
}
