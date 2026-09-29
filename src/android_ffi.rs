use std::sync::{Mutex, OnceLock};

use jni::objects::{JClass, JString};
use jni::sys::{jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use crate::{BiaDca, BiaDcaConfig, DeviceState, OfflineMobileBia};

static RUNTIME: OnceLock<Mutex<OfflineMobileBia>> = OnceLock::new();

fn runtime() -> &'static Mutex<OfflineMobileBia> {
    RUNTIME.get_or_init(|| {
        Mutex::new(OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default())))
    })
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

    env.new_string(reply)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
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
