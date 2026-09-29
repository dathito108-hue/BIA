package com.bia.mobile;

import android.app.Activity;
import android.app.ActivityManager;
import android.os.Bundle;
import android.os.SystemClock;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

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

    private TextView history;
    private EditText input;

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(24, 24, 24, 24);

        TextView title = new TextView(this);
        title.setText("BIA — Trí tuệ Duyên khởi");
        title.setTextSize(22f);
        title.setGravity(Gravity.CENTER_HORIZONTAL);
        root.addView(title, new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));

        history = new TextView(this);
        history.setText("BIA: Tôi đang hoạt động hoàn toàn cục bộ trên thiết bị.\n");
        history.setTextSize(16f);
        history.setTextIsSelectable(true);

        ScrollView scroll = new ScrollView(this);
        scroll.addView(history);
        LinearLayout.LayoutParams scrollParams = new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                0,
                1f
        );
        root.addView(scroll, scrollParams);

        input = new EditText(this);
        input.setHint("Nhập điều bạn muốn BIA quan sát hoặc xử lý...");
        input.setSingleLine(false);
        root.addView(input, new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));

        Button send = new Button(this);
        send.setText("Quán");
        send.setOnClickListener(v -> submit());
        root.addView(send, new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));

        TextView status = new TextView(this);
        status.setText("Offline • Rust native core • D0–D10");
        status.setGravity(Gravity.CENTER_HORIZONTAL);
        root.addView(status, new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        ));

        setContentView(root);
    }

    private void submit() {
        String text = input.getText().toString().trim();
        if (text.isEmpty()) return;

        ActivityManager am = (ActivityManager) getSystemService(ACTIVITY_SERVICE);
        ActivityManager.MemoryInfo mi = new ActivityManager.MemoryInfo();
        am.getMemoryInfo(mi);
        int memoryMb = (int) Math.max(64L, mi.availMem / (1024L * 1024L));

        long now = SystemClock.elapsedRealtime();
        String reply = nativeChat(text, now, 0.75f, 0.20f, 0.20f, memoryMb);

        history.append("\nBạn: " + text + "\nBIA: " + reply + "\n");
        input.setText("");
    }
}
