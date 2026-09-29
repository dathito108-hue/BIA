package com.bia.mobile;

import android.app.Activity;
import android.app.ActivityManager;
import android.app.AlertDialog;
import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.Context;
import android.content.Intent;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.net.Uri;
import android.os.Bundle;
import android.os.SystemClock;
import android.provider.Settings;
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
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;

public class MainActivity extends Activity {
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

    public static native long nativeCycleCount();
    public static native String nativeStatus();
    public static native boolean nativeSave(String path);
    public static native boolean nativeLoad(String path);
    public static native String nativePendingAction();
    public static native void nativeResolveAction(boolean success, long timestamp);

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

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);

        Window window = getWindow();
        window.setStatusBarColor(BG);
        window.setNavigationBarColor(BG);

        memoryPath = new File(getFilesDir(), "bia_dharma_memory.bin").getAbsolutePath();
        boolean restored = nativeLoad(memoryPath);

        LinearLayout root = column();
        root.setBackgroundColor(BG);
        root.setPadding(dp(18), dp(14), dp(18), dp(14));

        root.addView(buildHeader());
        root.addView(buildStatusCard());

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

        addBubble(
                restored
                        ? "Tôi đã khôi phục ký ức từ lần sử dụng trước."
                        : "Tôi đang hoạt động cục bộ trên thiết bị. Hãy cho tôi một Cảnh để bắt đầu.",
                false
        );

        addBubble(
                "Bạn có thể thử: “Nhớ rằng…”, “Mục tiêu: …”, “Tìm web …”, “Mở YouTube”, “Mở cài đặt”, hoặc “Sao chép …”.",
                false
        );

        root.addView(buildComposer());
        setContentView(root);
        refreshStatus();
    }

    @Override
    protected void onPause() {
        nativeSave(memoryPath);
        super.onPause();
    }

    @Override
    protected void onDestroy() {
        nativeSave(memoryPath);
        super.onDestroy();
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
        LinearLayout.LayoutParams markParams =
                new LinearLayout.LayoutParams(dp(54), dp(54));
        header.addView(mark, markParams);

        LinearLayout labels = column();
        labels.setPadding(dp(14), 0, 0, 0);
        labels.addView(text("BIA", 24, TEXT, Typeface.BOLD));
        labels.addView(text(
                "Trí tuệ Duyên khởi • Capability V3",
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

        card.addView(text("TĨNH → QUÁN → HÀNH", 11, GOLD, Typeface.BOLD));
        return card;
    }

    private View buildComposer() {
        LinearLayout composer = row();
        composer.setGravity(Gravity.BOTTOM);
        composer.setPadding(0, dp(10), 0, 0);

        input = new EditText(this);
        input.setTextColor(TEXT);
        input.setHintTextColor(Color.rgb(111, 128, 120));
        input.setTextSize(16f);
        input.setHint("Nói điều bạn muốn BIA hiểu hoặc làm...");
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
        inputParams.setMargins(0, 0, dp(10), 0);
        composer.addView(input, inputParams);

        Button send = new Button(this);
        send.setText("Quán");
        send.setTextColor(BG);
        send.setTextSize(14f);
        send.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        send.setAllCaps(false);
        send.setPadding(dp(16), dp(10), dp(16), dp(10));
        send.setBackground(rounded(GOLD, dp(20)));
        send.setOnClickListener(v -> submit());
        composer.addView(
                send,
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                        dp(52)
                )
        );

        return composer;
    }

    private void submit() {
        String text = input.getText().toString().trim();
        if (text.isEmpty()) return;

        addBubble(text, true);
        input.setText("");

        ActivityManager am = (ActivityManager) getSystemService(ACTIVITY_SERVICE);
        ActivityManager.MemoryInfo mi = new ActivityManager.MemoryInfo();
        am.getMemoryInfo(mi);
        int memoryMb =
                (int) Math.max(64L, mi.availMem / (1024L * 1024L));

        long now = SystemClock.elapsedRealtime();
        String reply = nativeChat(
                text,
                now,
                0.75f,
                0.20f,
                0.20f,
                memoryMb
        );

        addBubble(reply, false);
        handlePendingAction();
        nativeSave(memoryPath);
        refreshStatus();
    }

    private void handlePendingAction() {
        String encoded = nativePendingAction();
        if (encoded == null || encoded.isEmpty()) return;

        String[] parts = encoded.split("\t", -1);
        if (parts.length < 4) return;

        String kind = parts[0];
        String label = unescape(parts[1]);
        String payload = unescape(parts[2]);

        new AlertDialog.Builder(this)
                .setTitle("BIA đề xuất hành động")
                .setMessage(label + "\n\nChỉ thực thi khi bạn xác nhận.")
                .setNegativeButton("Hủy", (dialog, which) -> {
                    nativeResolveAction(false, SystemClock.elapsedRealtime());
                    addBubble("Hành động đã được hủy. Tôi đã ghi nhận kết quả này.", false);
                    nativeSave(memoryPath);
                    refreshStatus();
                })
                .setPositiveButton("Thực thi", (dialog, which) -> {
                    boolean success = executeAction(kind, payload);
                    nativeResolveAction(success, SystemClock.elapsedRealtime());
                    addBubble(
                            success
                                    ? "Hành động đã được thực thi và kết quả đã được huân tập."
                                    : "Hành động không thực hiện được; tôi đã ghi nhận thất bại để điều chỉnh.",
                            false
                    );
                    nativeSave(memoryPath);
                    refreshStatus();
                })
                .show();
    }

    private boolean executeAction(String kind, String payload) {
        try {
            switch (kind) {
                case "OPEN_SETTINGS":
                    startActivity(new Intent(Settings.ACTION_SETTINGS));
                    return true;

                case "OPEN_URL":
                    startActivity(new Intent(Intent.ACTION_VIEW, Uri.parse(payload)));
                    return true;

                case "SEARCH_WEB":
                    String query = URLEncoder.encode(payload, StandardCharsets.UTF_8.name());
                    startActivity(new Intent(
                            Intent.ACTION_VIEW,
                            Uri.parse("https://www.google.com/search?q=" + query)
                    ));
                    return true;

                case "LAUNCH_PACKAGE":
                    Intent launch = getPackageManager().getLaunchIntentForPackage(payload);
                    if (launch == null) {
                        Toast.makeText(this, "Không tìm thấy ứng dụng: " + payload, Toast.LENGTH_SHORT).show();
                        return false;
                    }
                    startActivity(launch);
                    return true;

                case "CLIPBOARD_WRITE":
                    ClipboardManager clipboard =
                            (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
                    clipboard.setPrimaryClip(ClipData.newPlainText("BIA", payload));
                    return true;

                default:
                    return false;
            }
        } catch (Exception e) {
            Toast.makeText(this, "Không thể thực thi hành động.", Toast.LENGTH_SHORT).show();
            return false;
        }
    }

    private String unescape(String value) {
        return value
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\\\", "\\");
    }

    private void refreshStatus() {
        if (status != null) {
            status.setText(nativeStatus());
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
