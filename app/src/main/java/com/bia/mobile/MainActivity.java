package com.bia.mobile;

import android.app.Activity;
import android.app.ActivityManager;
import android.app.AlertDialog;
import android.os.BatteryManager;
import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.Context;
import android.content.Intent;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.os.PowerManager;
import android.os.SystemClock;
import android.provider.Settings;
import android.speech.RecognizerIntent;
import android.speech.tts.TextToSpeech;
import android.view.Gravity;
import android.view.View;
import android.view.ViewGroup;
import android.view.Window;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;
import android.widget.Toast;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.ByteArrayOutputStream;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Locale;

public class MainActivity extends Activity implements TextToSpeech.OnInitListener {
    private static final int REQ_SPEECH = 97;
    private final Object executionLock = new Object();
    private boolean actionDialogOpen;
    private boolean executionResumed;
    private boolean awaitingExternalReturn;
    private final android.os.Handler executionHandler = new android.os.Handler(android.os.Looper.getMainLooper());
    private final Runnable executionPump = this::handlePendingAction;
    private TextView executionStatus;
    private static final int REQ_DOCUMENT = 98;

    static {
        System.loadLibrary("bia_core");
    }

    public static native String nativeChat(
            String input,
            long timestamp,
            float battery,
            float thermal,
            float load,
            int memoryMb
    );

    public static native String nativeImmediateToken(String input);
    public static native void nativeResetDialogueForTests();
    public static native String nativeBenchmarkV12(int iterations);
    public static native String nativeStressV14(int iterations);
    public static native String nativeReasoningV15();
    public static native String nativeGeneralizeV16();
    public static native String nativeOpenReasoningV18();
    public static native String nativeDeepIntelligenceV21();
    public static native String nativeEmergentV25();
    public static native String nativeAutonomyV31();
    public static native String nativeDeliberationV37();
    public static native String nativeMaxIntelligenceV45();
    public static native String nativeLearnedSemanticV61();
    public static native String nativeContinualV81();
    public static native String nativeAutonomousLoopV101();
    public static native String nativeEvidenceV130();
    public static native String nativeIntegratedV131();
    public static native long nativeCycleCount();
    public static native String nativeStatus();
    public static native boolean nativeSave(String path);
    public static native boolean nativeLoad(String path);
    public static native String nativePendingAction();
    public static native boolean nativeClaimAction(String id, long timestamp);
    public static native String nativeApprovalSnapshot();
    public static native boolean nativeApproveExecution(String snapshot, int mode, long timestamp);
    public static native boolean nativeIsActionApproved(String id, long timestamp);
    public static native String nativeExecutionPermissionStatus(long timestamp);
    public static native void nativeStopAutomation();
    public static native void nativeRevokeApproval();
    public static native boolean nativeCompleteAction(String id, boolean success, long timestamp);
    public static native boolean nativeCancelAction(String id);
    public static native String nativeExportContinuity();
    public static native boolean nativeImportContinuity(String state);
    public static native int nativeIngestContent(
            String source,
            int kind,
            String content,
            long timestamp,
            float confidence
    );

    private static final int BG = Color.rgb(12, 18, 16);
    private static final int SURFACE = Color.rgb(24, 34, 30);
    private static final int SURFACE_2 = Color.rgb(31, 45, 39);
    private static final int GOLD = Color.rgb(220, 181, 92);
    private static final int JADE = Color.rgb(87, 181, 137);
    private static final int TEXT = Color.rgb(239, 244, 241);
    private static final int MUTED = Color.rgb(161, 178, 170);

    private LinearLayout messages;
    private ScrollView scroll;
    private EditText input;
    private TextView status;
    private String memoryPath;
    private String continuityPath;
    private TextToSpeech tts;
    private boolean ttsReady;
    private boolean voiceTurn;
    private String lastReply = "";

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);

        Window window = getWindow();
        window.setStatusBarColor(BG);
        window.setNavigationBarColor(BG);

        memoryPath = new File(getFilesDir(), "bia_dharma_memory.bin").getAbsolutePath();
        continuityPath = new File(getFilesDir(), "bia_runtime_continuity.txt").getAbsolutePath();

        boolean restoredMemory = nativeLoad(memoryPath);
        boolean restoredRuntime = loadContinuity();

        tts = new TextToSpeech(this, this);

        LinearLayout root = column();
        root.setBackgroundColor(BG);
        root.setPadding(dp(18), dp(14), dp(18), dp(14));

        root.addView(buildHeader());
        root.addView(buildStatusCard());
        executionStatus = text("Tự duyệt: tắt", 12, MUTED, Typeface.NORMAL);
        root.addView(executionStatus);
        Button stopExecution = compactButton("Dừng chuỗi / Tắt tự duyệt");
        stopExecution.setOnClickListener(v -> {
            executionHandler.removeCallbacks(executionPump);
            synchronized (executionLock) { nativeStopAutomation(); persistAll(); }
            addBubble("Đã dừng các bước chưa chạy và thu hồi quyền tự duyệt. Bước chưa rõ kết quả được giữ để kiểm tra.", false);
            refreshStatus();
        });
        root.addView(stopExecution);
        Button creative = compactButton("Xưởng ảnh & 3D: tạo và xuất tài nguyên");
        creative.setOnClickListener(v -> startActivity(new Intent(this, CreativeActivity.class)));
        root.addView(creative);
        Button products = compactButton("Xưởng sản phẩm: tạo mã và gói bàn giao");
        products.setOnClickListener(v -> startActivity(new Intent(this, ProductActivity.class)));
        root.addView(products);
        Button game = compactButton("Game: quan sát và đa chạm");
        game.setOnClickListener(v -> startActivity(new Intent(this, GameSetupActivity.class)));
        root.addView(game);
        Button trading = compactButton("Trading: dữ liệu realtime");
        trading.setOnClickListener(v -> startActivity(new Intent(this, TradingActivity.class)));
        root.addView(trading);
        Button dex = compactButton("DEX: pool thật và chuẩn bị swap");
        dex.setOnClickListener(v -> startActivity(new Intent(this, DexActivity.class)));
        root.addView(dex);
        Button solana = compactButton("Solana: ví và giao dịch mainnet");
        solana.setOnClickListener(v -> startActivity(new Intent(this, SolanaActivity.class)));
        root.addView(solana);

        scroll = new ScrollView(this);
        scroll.setFillViewport(true);
        messages = column();
        messages.setPadding(0, dp(12), 0, dp(12));
        scroll.addView(messages);

        LinearLayout.LayoutParams scrollParams =
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT,
                        0,
                        1f
                );
        root.addView(scroll, scrollParams);

        if (restoredMemory || restoredRuntime) {
            addBubble(
                    "Tôi đã nối lại ký ức và trạng thái công việc từ lần sử dụng trước.",
                    false
            );
        } else {
            addBubble(
                    "Tôi đang hoạt động cục bộ trên thiết bị. Hãy cho tôi một Cảnh hoặc mục tiêu để bắt đầu.",
                    false
            );
        }

        addBubble(
                "Mic, Tệp và Android Share đã sẵn sàng. BIA có thể Quán văn bản/tài liệu có provenance, lập kế hoạch nhiều bước và tiếp tục công việc qua restart.",
                false
        );

        root.addView(buildComposer());
        setContentView(root);
        refreshStatus();

        if (restoredRuntime && !nativePendingAction().isEmpty()) {
            addBubble("Có một hành động đang chờ từ phiên trước.", false);
            handlePendingAction();
        }
        handleIncomingShare(getIntent());
    }

    @Override
    protected void onResume() {
        super.onResume();
        executionResumed = true;
        awaitingExternalReturn = false;
        scheduleExecution();
    }

    private void scheduleExecution() {
        executionHandler.removeCallbacks(executionPump);
        if (executionResumed && !awaitingExternalReturn && !isFinishing()) {
            executionHandler.postDelayed(executionPump, 150);
        }
    }

    @Override
    protected void onPause() {
        executionResumed = false;
        executionHandler.removeCallbacks(executionPump);
        persistAll();
        super.onPause();
    }

    @Override
    protected void onDestroy() {
        executionResumed = false;
        executionHandler.removeCallbacks(executionPump);
        nativeRevokeApproval();
        persistAll();
        if (tts != null) {
            tts.stop();
            tts.shutdown();
        }
        super.onDestroy();
    }

    @Override
    public void onInit(int statusCode) {
        if (statusCode == TextToSpeech.SUCCESS) {
            int result = tts.setLanguage(new Locale("vi", "VN"));
            ttsReady = result != TextToSpeech.LANG_MISSING_DATA
                    && result != TextToSpeech.LANG_NOT_SUPPORTED;
        }
    }

    private View buildHeader() {
        LinearLayout header = row();
        header.setGravity(Gravity.CENTER_VERTICAL);
        header.setPadding(0, dp(4), 0, dp(12));

        TextView mark = text("◉", 28, GOLD, Typeface.BOLD);
        GradientDrawable markBg = rounded(SURFACE_2, dp(18));
        markBg.setStroke(dp(1), GOLD);
        mark.setBackground(markBg);
        mark.setGravity(Gravity.CENTER);
        header.addView(mark, new LinearLayout.LayoutParams(dp(54), dp(54)));

        LinearLayout labels = column();
        labels.setPadding(dp(14), 0, 0, 0);
        labels.addView(text("BIA", 24, TEXT, Typeface.BOLD));
        labels.addView(text(
                "Trí tuệ Duyên khởi • Tam-Thiên Matrix V9",
                13,
                MUTED,
                Typeface.NORMAL
        ));

        header.addView(
                labels,
                new LinearLayout.LayoutParams(
                        0,
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                        1f
                )
        );

        TextView badge = text("LOCAL", 11, JADE, Typeface.BOLD);
        badge.setGravity(Gravity.CENTER);
        badge.setPadding(dp(10), dp(7), dp(10), dp(7));
        GradientDrawable badgeBg = rounded(Color.rgb(21, 55, 42), dp(18));
        badgeBg.setStroke(dp(1), Color.rgb(57, 129, 96));
        badge.setBackground(badgeBg);
        header.addView(badge);

        return header;
    }

    private View buildStatusCard() {
        LinearLayout card = row();
        card.setGravity(Gravity.CENTER_VERTICAL);
        card.setPadding(dp(14), dp(12), dp(14), dp(12));

        GradientDrawable bg = rounded(SURFACE, dp(18));
        bg.setStroke(dp(1), Color.rgb(45, 64, 56));
        card.setBackground(bg);

        card.addView(text("●", 12, JADE, Typeface.BOLD));

        status = text("Đang khởi tạo...", 13, MUTED, Typeface.NORMAL);
        status.setPadding(dp(8), 0, 0, 0);
        card.addView(
                status,
                new LinearLayout.LayoutParams(
                        0,
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                        1f
                )
        );

        card.addView(text("CẢNH → QUÁN → HÀNH", 11, GOLD, Typeface.BOLD));
        return card;
    }

    private View buildComposer() {
        LinearLayout composer = row();
        composer.setGravity(Gravity.BOTTOM | Gravity.CENTER_VERTICAL);
        composer.setPadding(0, dp(10), 0, 0);

        Button mic = compactButton("Mic");
        mic.setOnClickListener(v -> startVoiceRecognition());
        composer.addView(mic, new LinearLayout.LayoutParams(dp(58), dp(52)));

        Button file = compactButton("Tệp");
        file.setOnClickListener(v -> openDocument());
        LinearLayout.LayoutParams fileParams = new LinearLayout.LayoutParams(dp(58), dp(52));
        fileParams.setMargins(dp(6), 0, 0, 0);
        composer.addView(file, fileParams);

        input = new EditText(this);
        input.setTextColor(TEXT);
        input.setHintTextColor(Color.rgb(111, 128, 120));
        input.setTextSize(16f);
        input.setHint("Nói hoặc nhập điều BIA cần hiểu/làm...");
        input.setMinLines(1);
        input.setMaxLines(4);
        input.setPadding(dp(16), dp(12), dp(16), dp(12));
        input.setBackground(rounded(SURFACE, dp(22)));

        LinearLayout.LayoutParams inputParams =
                new LinearLayout.LayoutParams(
                        0,
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                        1f
                );
        inputParams.setMargins(dp(8), 0, dp(8), 0);
        composer.addView(input, inputParams);

        Button speak = compactButton("Đọc");
        speak.setOnClickListener(v -> speakLastReply());
        composer.addView(speak, new LinearLayout.LayoutParams(dp(58), dp(52)));

        Button send = new Button(this);
        send.setText("Quán");
        send.setTextColor(BG);
        send.setTextSize(14f);
        send.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        send.setAllCaps(false);
        send.setBackground(rounded(GOLD, dp(20)));
        send.setOnClickListener(v -> submit());
        LinearLayout.LayoutParams sendParams =
                new LinearLayout.LayoutParams(dp(72), dp(52));
        sendParams.setMargins(dp(8), 0, 0, 0);
        composer.addView(send, sendParams);

        return composer;
    }

    private Button compactButton(String label) {
        Button button = new Button(this);
        button.setText(label);
        button.setTextColor(TEXT);
        button.setTextSize(12f);
        button.setAllCaps(false);
        button.setBackground(rounded(SURFACE_2, dp(18)));
        return button;
    }

    private void startVoiceRecognition() {
        try {
            Intent intent = new Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH);
            intent.putExtra(
                    RecognizerIntent.EXTRA_LANGUAGE_MODEL,
                    RecognizerIntent.LANGUAGE_MODEL_FREE_FORM
            );
            intent.putExtra(RecognizerIntent.EXTRA_LANGUAGE, "vi-VN");
            intent.putExtra(RecognizerIntent.EXTRA_PROMPT, "Nói với BIA");
            startActivityForResult(intent, REQ_SPEECH);
        } catch (Exception e) {
            Toast.makeText(
                    this,
                    "Thiết bị chưa có dịch vụ nhận dạng giọng nói.",
                    Toast.LENGTH_SHORT
            ).show();
        }
    }

    private void openDocument() {
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.setType("text/*");
        startActivityForResult(intent, REQ_DOCUMENT);
    }

    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        setIntent(intent);
        handleIncomingShare(intent);
    }

    private void handleIncomingShare(Intent intent) {
        if (intent == null || !Intent.ACTION_SEND.equals(intent.getAction())) return;
        String shared = intent.getStringExtra(Intent.EXTRA_TEXT);
        if (shared == null || shared.trim().isEmpty()) return;
        int count = nativeIngestContent(
                "android-share",
                1,
                shared,
                SystemClock.elapsedRealtime(),
                0.90f
        );
        addBubble("Đã tiếp nhận nội dung được chia sẻ: " + count + " Cảnh có provenance.", false);
        persistAll();
        refreshStatus();
    }

    private void ingestDocument(Uri uri) {
        if (uri == null) return;
        try (InputStream in = getContentResolver().openInputStream(uri);
             ByteArrayOutputStream out = new ByteArrayOutputStream()) {
            if (in == null) return;
            byte[] buffer = new byte[4096];
            int total = 0;
            int n;
            while ((n = in.read(buffer)) > 0 && total < 256 * 1024) {
                int take = Math.min(n, 256 * 1024 - total);
                out.write(buffer, 0, take);
                total += take;
            }
            String content = new String(out.toByteArray(), StandardCharsets.UTF_8);
            int count = nativeIngestContent(
                    uri.toString(),
                    2,
                    content,
                    SystemClock.elapsedRealtime(),
                    0.85f
            );
            addBubble(
                    "Đã Quán tài liệu cục bộ và ghi " + count + " Cảnh kèm nguồn gốc.",
                    false
            );
            persistAll();
            refreshStatus();
        } catch (Exception e) {
            Toast.makeText(this, "Không đọc được tài liệu này.", Toast.LENGTH_SHORT).show();
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode == REQ_SPEECH && resultCode == RESULT_OK && data != null) {
            ArrayList<String> results =
                    data.getStringArrayListExtra(RecognizerIntent.EXTRA_RESULTS);
            if (results != null && !results.isEmpty()) {
                input.setText(results.get(0));
                voiceTurn = true;
                submit();
            }
        } else if (requestCode == REQ_DOCUMENT && resultCode == RESULT_OK && data != null) {
            ingestDocument(data.getData());
        }
    }

    private void submit() {
        String text = input.getText().toString().trim();
        if (text.isEmpty()) return;

        addBubble(text, true);
        input.setText("");

        if (text.equalsIgnoreCase("/bench") || text.equalsIgnoreCase("benchmark")) {
            new Thread(() -> {
                String report = nativeBenchmarkV12(5000);
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-device-benchmark").start();
            return;
        }

        if (text.equalsIgnoreCase("/stress") || text.equalsIgnoreCase("stress")) {
            new Thread(() -> {
                String report = nativeStressV14(10000);
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-device-stress").start();
            return;
        }

        if (text.equalsIgnoreCase("/reason") || text.equalsIgnoreCase("reasoning")) {
            new Thread(() -> {
                String report = nativeReasoningV15();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-reasoning-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/generalize") || text.equalsIgnoreCase("generalize")) {
            new Thread(() -> {
                String report = nativeGeneralizeV16();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-generalization-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/openproof") || text.equalsIgnoreCase("openproof")) {
            new Thread(() -> {
                String report = nativeOpenReasoningV18();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-open-reasoning-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/deep") || text.equalsIgnoreCase("deep")) {
            new Thread(() -> {
                String report = nativeDeepIntelligenceV21();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-deep-intelligence-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/emergent") || text.equalsIgnoreCase("emergent")) {
            new Thread(() -> {
                String report = nativeEmergentV25();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-emergent-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/autonomy") || text.equalsIgnoreCase("autonomy")) {
            new Thread(() -> {
                String report = nativeAutonomyV31();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-autonomous-knowledge-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/world") || text.equalsIgnoreCase("world")) {
            new Thread(() -> {
                String report = nativeDeliberationV37();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-world-model-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/maxintel") || text.equalsIgnoreCase("maxintel")) {
            new Thread(() -> {
                String report = nativeMaxIntelligenceV45();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-max-intelligence-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/semantic") || text.equalsIgnoreCase("semantic")) {
            new Thread(() -> {
                String report = nativeLearnedSemanticV61();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-learned-semantic-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/continual") || text.equalsIgnoreCase("continual")) {
            new Thread(() -> {
                String report = nativeContinualV81();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-continual-generative-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/loop") || text.equalsIgnoreCase("loop")) {
            new Thread(() -> {
                String report = nativeAutonomousLoopV101();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-autonomous-loop-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/evidence")) {
            new Thread(() -> {
                String report = nativeEvidenceV130();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-evidence-proof").start();
            return;
        }

        if (text.equalsIgnoreCase("/integrated")) {
            new Thread(() -> {
                String report = nativeIntegratedV131();
                runOnUiThread(() -> {
                    lastReply = report;
                    addBubble(report, false);
                    refreshStatus();
                });
            }, "bia-integrated-proof").start();
            return;
        }

        String instant = nativeImmediateToken(text);
        if (instant != null && !instant.isEmpty()) {
            addBubble(instant + " …", false);
        }

        ActivityManager am = (ActivityManager) getSystemService(ACTIVITY_SERVICE);
        ActivityManager.MemoryInfo mi = new ActivityManager.MemoryInfo();
        am.getMemoryInfo(mi);
        int memoryMb =
                (int) Math.max(64L, mi.availMem / (1024L * 1024L));

        float battery = readBattery();
        float thermal = readThermal();
        float load = mi.lowMemory ? 0.90f : 0.25f;
        boolean shouldSpeak = voiceTurn;
        voiceTurn = false;

        new Thread(() -> {
            String reply;
            synchronized (executionLock) {
                reply = nativeChat(
                    text,
                    SystemClock.elapsedRealtime(),
                    battery,
                    thermal,
                    load,
                    memoryMb
                );
            }

            runOnUiThread(() -> {
                lastReply = reply;
                addBubble(reply, false);

                if (shouldSpeak) {
                    speak(reply);
                }

                scheduleExecution();
                persistAll();
                refreshStatus();
            });
        }, "bia-tam-thien").start();
    }

    private float readBattery() {
        BatteryManager battery =
                (BatteryManager) getSystemService(BATTERY_SERVICE);
        int percent = battery.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY);
        if (percent < 0 || percent > 100) return 0.5f;
        return percent / 100f;
    }

    private float readThermal() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            return 0.2f;
        }
        PowerManager power = (PowerManager) getSystemService(POWER_SERVICE);
        int value = power.getCurrentThermalStatus();
        return Math.min(1f, Math.max(0f, value / 6f));
    }

    private void handlePendingAction() {
        if (!executionResumed || awaitingExternalReturn || actionDialogOpen || isFinishing()) return;
        String snapshot = nativeApprovalSnapshot();
        if (snapshot == null || snapshot.isEmpty()) { refreshStatus(); return; }
        String[] rows = snapshot.split("\n");
        String[] first = rows[0].split("\t", -1);
        if (first.length != 4) return;
        if (nativeIsActionApproved(first[3], SystemClock.elapsedRealtime())) {
            executeApprovedStep(first);
            return;
        }
        StringBuilder preview = new StringBuilder();
        for (int i = 0; i < rows.length; i++) {
            String[] p = rows[i].split("\t", -1);
            if (p.length != 4) return;
            preview.append(i + 1).append(". ").append(unescape(p[1]))
                    .append("\n").append(unescape(p[2])).append("\n\n");
        }
        preview.append("Tự duyệt chỉ khớp đúng loại thao tác và nội dung ở trên, kể cả các lệnh mới bạn gửi. ")
                .append("Tối đa 100 lượt / 30 phút; mở lại tiến trình phải cấp lại. ")
                .append("Mở ứng dụng/URL chỉ xác nhận Android tiếp nhận; chuỗi chờ bạn quay lại BIA.");
        LinearLayout content = column();
        content.setPadding(dp(16), dp(8), dp(16), dp(8));
        android.widget.RadioGroup modes = new android.widget.RadioGroup(this);
        String[] choices = {"Chỉ bước đầu", "Cả chuỗi hiện tại", "Tự duyệt thao tác đã liệt kê: 30 phút / 100 lượt"};
        for (int i = 0; i < choices.length; i++) {
            android.widget.RadioButton option = new android.widget.RadioButton(this);
            option.setId(i + 1); option.setText(choices[i]); option.setTextColor(TEXT);
            modes.addView(option);
        }
        modes.check(1);
        content.addView(modes);
        TextView details = text(preview.toString(), 14, TEXT, Typeface.NORMAL);
        details.setTextIsSelectable(true);
        content.addView(details);
        ScrollView review = new ScrollView(this);
        review.addView(content);
        actionDialogOpen = true;
        AlertDialog dialog = new AlertDialog.Builder(this)
                .setTitle("Duyệt phạm vi thực thi")
                .setView(review)
                .setNegativeButton("Dừng", (d, which) -> {
                    synchronized (executionLock) { nativeStopAutomation(); persistAll(); }
                    refreshStatus();
                })
                .setPositiveButton("Cấp quyền và chạy", (d, which) -> {
                    boolean approved;
                    synchronized (executionLock) {
                        approved = nativeApproveExecution(snapshot, modes.getCheckedRadioButtonId(), SystemClock.elapsedRealtime());
                    }
                    if (!approved) addBubble("Hàng đợi đã đổi hoặc có bước chưa rõ kết quả. Chưa cấp quyền; hãy kiểm tra Trạng thái thực thi.", false);
                    if (approved) scheduleExecution();
                    refreshStatus();
                })
                .create();
        dialog.setOnDismissListener(d -> actionDialogOpen = false);
        dialog.show();
    }

    private void executeApprovedStep(String[] parts) {
        String kind = parts[0];
        String payload = unescape(parts[2]);
        String id = parts[3];
        boolean continueQueue = false;
        synchronized (executionLock) {
            if (!nativeClaimAction(id, SystemClock.elapsedRealtime())) {
                lastReply = "Đã dừng: quyền hết hạn, bước thay đổi hoặc kết quả chưa rõ.";
            } else if (!saveContinuity(nativeExportContinuity())) {
                nativeRevokeApproval();
                lastReply = "Không chạy vì chưa lưu được trạng thái an toàn; đã tắt tự duyệt.";
            } else {
                boolean external = !kind.equals("CLIPBOARD_WRITE");
                awaitingExternalReturn = external;
                boolean success = executeAction(kind, payload);
                if (!success) awaitingExternalReturn = false;
                boolean accepted = nativeCompleteAction(id, success, SystemClock.elapsedRealtime());
                boolean saved = saveContinuity(nativeExportContinuity());
                if (!accepted || !saved) nativeRevokeApproval();
                lastReply = !accepted || !saved ? "Kết quả chưa được lưu đầy đủ; đã tắt tự duyệt và dừng để kiểm tra."
                        : success ? (external ? "Android đã tiếp nhận thao tác. Quay lại BIA để tiếp tục chuỗi."
                        : "Đã ghi và kiểm tra clipboard; tiếp tục bước được cấp quyền.")
                        : "Thao tác thất bại; đã dừng chuỗi, tắt tự duyệt và ghi nhận lỗi kỹ năng.";
                continueQueue = accepted && saved && success;
            }
        }
        addBubble(lastReply, false);
        refreshStatus();
        if (continueQueue) scheduleExecution();
    }

    private boolean executeAction(String kind, String payload) {
        try {
            switch (kind) {
                case "OPEN_SETTINGS":
                    startActivity(new Intent(Settings.ACTION_SETTINGS));
                    return true;
                case "OPEN_URL":
                    if (!(payload.startsWith("https://") || payload.startsWith("http://"))) return false;
                    startActivity(new Intent(Intent.ACTION_VIEW, Uri.parse(payload)));
                    return true;
                case "SEARCH_WEB":
                    String query = URLEncoder.encode(
                            payload,
                            StandardCharsets.UTF_8.name()
                    );
                    startActivity(new Intent(
                            Intent.ACTION_VIEW,
                            Uri.parse("https://www.google.com/search?q=" + query)
                    ));
                    return true;
                case "LAUNCH_PACKAGE":
                    Intent launch =
                            getPackageManager().getLaunchIntentForPackage(payload);
                    if (launch == null) {
                        Toast.makeText(
                                this,
                                "Không tìm thấy ứng dụng: " + payload,
                                Toast.LENGTH_SHORT
                        ).show();
                        return false;
                    }
                    startActivity(launch);
                    return true;
                case "CLIPBOARD_WRITE":
                    ClipboardManager clipboard =
                            (ClipboardManager) getSystemService(
                                    Context.CLIPBOARD_SERVICE
                            );
                    clipboard.setPrimaryClip(
                            ClipData.newPlainText("BIA", payload)
                    );
                    ClipData readBack = clipboard.getPrimaryClip();
                    return readBack != null && readBack.getItemCount() > 0
                            && payload.contentEquals(readBack.getItemAt(0).coerceToText(this));
                default:
                    return false;
            }
        } catch (Exception e) {
            Toast.makeText(
                    this,
                    "Không thể thực thi hành động.",
                    Toast.LENGTH_SHORT
            ).show();
            return false;
        }
    }

    private void speakLastReply() {
        if (lastReply == null || lastReply.isEmpty()) {
            Toast.makeText(this, "Chưa có phản hồi để đọc.", Toast.LENGTH_SHORT).show();
            return;
        }
        speak(lastReply);
    }

    private void speak(String value) {
        if (!ttsReady || tts == null) {
            Toast.makeText(
                    this,
                    "Dịch vụ đọc tiếng Việt chưa sẵn sàng.",
                    Toast.LENGTH_SHORT
            ).show();
            return;
        }
        tts.speak(value, TextToSpeech.QUEUE_FLUSH, null, "bia-reply");
    }

    private void persistAll() {
        nativeSave(memoryPath);
        saveContinuity(nativeExportContinuity());
    }

    private boolean loadContinuity() {
        File file = new File(continuityPath);
        try (FileInputStream in = new android.util.AtomicFile(file).openRead()) {
            ByteArrayOutputStream data = new ByteArrayOutputStream();
            byte[] buffer = new byte[4096];
            int n;
            while ((n = in.read(buffer)) != -1) {
                if (data.size() + n > 1024 * 1024) return false;
                data.write(buffer, 0, n);
            }
            String state = new String(data.toByteArray(), StandardCharsets.UTF_8);
            return nativeImportContinuity(state);
        } catch (Exception e) {
            return false;
        }
    }

    private boolean saveContinuity(String state) {
        android.util.AtomicFile file = new android.util.AtomicFile(new File(continuityPath));
        FileOutputStream out = null;
        try {
            out = file.startWrite();
            out.write(state.getBytes(StandardCharsets.UTF_8));
            out.flush();
            out.getFD().sync();
            file.finishWrite(out);
            // AtomicFile logs some rename failures instead of throwing: verify the committed bytes.
            try (FileInputStream committed = file.openRead()) {
                ByteArrayOutputStream data = new ByteArrayOutputStream();
                byte[] buffer = new byte[4096];
                int n;
                while ((n = committed.read(buffer)) != -1) {
                    if (data.size() + n > 1024 * 1024) return false;
                    data.write(buffer, 0, n);
                }
                return java.util.Arrays.equals(data.toByteArray(), state.getBytes(StandardCharsets.UTF_8));
            }
        } catch (Exception error) {
            if (out != null) file.failWrite(out);
            return false;
        }
    }

    private String unescape(String value) {
        StringBuilder out = new StringBuilder();
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            if (c == '\\' && i + 1 < value.length()) {
                char next = value.charAt(++i);
                if (next == 'n') out.append('\n');
                else if (next == 't') out.append('\t');
                else if (next == '\\') out.append('\\');
                else { out.append('\\'); out.append(next); }
            } else out.append(c);
        }
        return out.toString();
    }

    private void refreshStatus() {
        if (executionStatus != null) executionStatus.setText(nativeExecutionPermissionStatus(SystemClock.elapsedRealtime()));
        if (status != null) {
            status.setText(
                    nativeStatus()
                            + String.format(
                                    Locale.US,
                                    "  •  Pin %.0f%%  •  Nhiệt %.0f%%",
                                    readBattery() * 100f,
                                    readThermal() * 100f
                            )
            );
        }
    }

    private void addBubble(String value, boolean user) {
        TextView bubble = text(
                value,
                15,
                user ? Color.rgb(15, 27, 22) : TEXT,
                Typeface.NORMAL
        );
        bubble.setTextIsSelectable(true);
        bubble.setLineSpacing(0f, 1.12f);
        bubble.setPadding(dp(15), dp(11), dp(15), dp(11));
        bubble.setBackground(
                rounded(
                        user ? Color.rgb(180, 218, 194) : SURFACE_2,
                        dp(20)
                )
        );

        LinearLayout holder = row();
        holder.setGravity(user ? Gravity.END : Gravity.START);
        holder.addView(
                bubble,
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                        ViewGroup.LayoutParams.WRAP_CONTENT
                )
        );

        LinearLayout.LayoutParams holderParams =
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT,
                        ViewGroup.LayoutParams.WRAP_CONTENT
                );
        holderParams.setMargins(
                user ? dp(44) : 0,
                dp(5),
                user ? 0 : dp(44),
                dp(5)
        );
        messages.addView(holder, holderParams);
        scroll.post(() -> scroll.fullScroll(View.FOCUS_DOWN));
    }

    private LinearLayout column() {
        LinearLayout v = new LinearLayout(this);
        v.setOrientation(LinearLayout.VERTICAL);
        return v;
    }

    private LinearLayout row() {
        LinearLayout v = new LinearLayout(this);
        v.setOrientation(LinearLayout.HORIZONTAL);
        return v;
    }

    private TextView text(String value, float size, int color, int style) {
        TextView view = new TextView(this);
        view.setText(value);
        view.setTextSize(size);
        view.setTextColor(color);
        view.setTypeface(Typeface.DEFAULT, style);
        return view;
    }

    private GradientDrawable rounded(int color, float radius) {
        GradientDrawable drawable = new GradientDrawable();
        drawable.setColor(color);
        drawable.setCornerRadius(radius);
        return drawable;
    }

    private int dp(int value) {
        return Math.round(value * getResources().getDisplayMetrics().density);
    }
}
