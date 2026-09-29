package com.bia.mobile;
import android.app.Activity;
import android.os.*;
import android.graphics.Color;
import android.text.InputType;
import android.view.*;
import android.widget.*;

/** Foreground-only read-only live market terminal. No broker account or trading key. */
public final class TradingActivity extends Activity {
    Spinner source;EditText symbols,key;TextView output;Button connect,stop;MarketFeed feed;
    final Handler ui=new Handler(Looper.getMainLooper());
    final Runnable refresh=new Runnable(){public void run(){if(feed!=null){output.setText(feed.snapshot());if(!feed.running)setEditing(true);}ui.postDelayed(this,1000);}};
    @Override public void onCreate(Bundle b){
        super.onCreate(b);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_SECURE);
        ScrollView scroll=new ScrollView(this);LinearLayout root=new LinearLayout(this);root.setOrientation(LinearLayout.VERTICAL);root.setPadding(20,20,20,20);scroll.addView(root);setContentView(scroll);
        TextView title=new TextView(this);title.setText("BIA Trading — dữ liệu realtime");title.setTextSize(22);root.addView(title);
        TextView note=new TextView(this);note.setText("Giá thật qua WebSocket. Chỉ đọc dữ liệu, chưa đặt lệnh. EMA/ATR là quy tắc kỹ thuật, chưa chứng minh khả năng sinh lời. Phiên dừng khi rời màn hình.\nBinance: crypto spot công khai. Twelve Data: Forex / cổ phiếu / ETF / crypto theo quyền nguồn; dữ liệu có thể bị trễ. Không nhập key sàn giao dịch.");root.addView(note);
        source=new Spinner(this);source.setAdapter(new ArrayAdapter<String>(this,android.R.layout.simple_spinner_dropdown_item,new String[]{"Binance — crypto spot","Twelve Data — đa thị trường"}));root.addView(source);
        symbols=new EditText(this);symbols.setSingleLine(true);symbols.setHint("1–3 mã: BTCUSDT,ETHUSDT hoặc EUR/USD,AAPL");symbols.setText("BTCUSDT,ETHUSDT");root.addView(symbols);
        key=new EditText(this);key.setSingleLine(true);key.setHint("Twelve Data API key dữ liệu — chỉ giữ trong phiên");key.setInputType(InputType.TYPE_CLASS_TEXT|InputType.TYPE_TEXT_VARIATION_PASSWORD);key.setSaveEnabled(false);if(Build.VERSION.SDK_INT>=26)key.setImportantForAutofill(View.IMPORTANT_FOR_AUTOFILL_NO_EXCLUDE_DESCENDANTS);root.addView(key);
        connect=new Button(this);connect.setText("Kết nối dữ liệu thật");root.addView(connect);connect.setOnClickListener(v->startFeed());
        stop=new Button(this);stop.setText("Dừng kết nối");root.addView(stop);stop.setOnClickListener(v->stopFeed());
        output=new TextView(this);output.setTextIsSelectable(true);output.setTextSize(15);output.setText("Chưa kết nối. Không có dữ liệu demo hoặc lệnh mô phỏng.");root.addView(output);
    }
    void startFeed(){
        stopFeed();
        try{feed=new MarketFeed(source.getSelectedItemPosition()==1,symbols.getText().toString().trim(),key.getText().toString().trim(),()->{});setEditing(false);feed.start();output.setText(feed.snapshot());}
        catch(IllegalArgumentException e){output.setText(e.getMessage());setEditing(true);}
    }
    void setEditing(boolean on){connect.setEnabled(on);source.setEnabled(on);symbols.setEnabled(on);key.setEnabled(on);}
    void stopFeed(){if(feed!=null){feed.stop();output.setText(feed.snapshot());feed=null;}setEditing(true);}
    @Override protected void onResume(){super.onResume();ui.post(refresh);}
    @Override protected void onPause(){ui.removeCallbacks(refresh);stopFeed();key.setText("");super.onPause();}
    @Override protected void onDestroy(){ui.removeCallbacksAndMessages(null);stopFeed();super.onDestroy();}
}
