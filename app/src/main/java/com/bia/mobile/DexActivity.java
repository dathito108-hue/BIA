package com.bia.mobile;
import android.app.Activity;
import android.os.*;
import android.content.*;
import android.widget.*;
import java.util.concurrent.*;

public final class DexActivity extends Activity {
    Spinner chain,direction;EditText pool,amount,slip,owner;TextView output,draft;
    String draftForm="";
    String form(){return chain.getSelectedItemPosition()+"|"+pool.getText()+"|"+amount.getText()+"|"+slip.getText()+"|"+owner.getText()+"|"+direction.getSelectedItemPosition();}
    Button start,stop,prepare,copy;DexFeed feed;int generation;
    final Handler ui=new Handler(Looper.getMainLooper());
    final ExecutorService tasks=Executors.newSingleThreadExecutor();
    final Runnable refresh=new Runnable(){public void run(){if(feed!=null)output.setText(feed.report(amount.getText().toString(),direction.getSelectedItemPosition()==1,slippage()));ui.postDelayed(this,1000);}};
    int slippage(){try{return Integer.parseInt(slip.getText().toString());}catch(Exception e){return -1;}}
    EditText field(LinearLayout root,String hint,String value){EditText e=new EditText(this);e.setSingleLine(true);e.setHint(hint);e.setText(value);root.addView(e);return e;}
    Button button(LinearLayout root,String label){Button b=new Button(this);b.setText(label);root.addView(b);return b;}
    @Override public void onCreate(Bundle b){super.onCreate(b);ScrollView scroll=new ScrollView(this);LinearLayout root=new LinearLayout(this);root.setOrientation(1);root.setPadding(20,20,20,20);scroll.addView(root);setContentView(scroll);
        TextView intro=new TextView(this);intro.setText("BIA DEX — on-chain thật\nĐọc pool V2 mỗi 15 giây sau lần đọc trước. Báo giá AMM, chưa phải giao dịch. Hỗ trợ ERC20 chuẩn; chưa xác minh honeypot/thuế token. Không nhập seed/private key.\nLệnh chỉ được chuẩn bị chưa ký; cần ví xác nhận để giao dịch thực tế.");root.addView(intro);
        chain=new Spinner(this);chain.setAdapter(new ArrayAdapter<String>(this,android.R.layout.simple_spinner_dropdown_item,DexFeed.NAMES));root.addView(chain);
        pool=field(root,"Địa chỉ pool V2 (không phải địa chỉ token)","0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc");
        direction=new Spinner(this);direction.setAdapter(new ArrayAdapter<String>(this,android.R.layout.simple_spinner_dropdown_item,new String[]{"Token0 → Token1","Token1 → Token0"}));root.addView(direction);
        amount=field(root,"Lượng token đầu vào","1");slip=field(root,"Slippage bps (50 = 0.5%, tối đa 100)","50");
        start=button(root,"Đọc pool thật");stop=button(root,"Dừng DEX");
        output=new TextView(this);output.setTextIsSelectable(true);root.addView(output);
        owner=field(root,"Địa chỉ ví công khai — dùng làm from và recipient","");owner.setSaveEnabled(false);
        prepare=button(root,"Kiểm tra & tạo swap CHƯA KÝ");copy=button(root,"Sao chép JSON chưa ký");copy.setEnabled(false);
        draft=new TextView(this);draft.setTextIsSelectable(true);root.addView(draft);
        start.setOnClickListener(v->{stopFeed();try{feed=new DexFeed(chain.getSelectedItemPosition(),pool.getText().toString().trim());chain.setEnabled(false);pool.setEnabled(false);feed.start();}catch(Exception e){output.setText("Địa chỉ hoặc mạng không hợp lệ");}});
        stop.setOnClickListener(v->stopFeed());
        prepare.setOnClickListener(v->{if(feed==null)return;DexFeed f=feed;int token=generation;String capturedForm=form();String address=owner.getText().toString().trim(),qty=amount.getText().toString();boolean reverse=direction.getSelectedItemPosition()==1;int bps=slippage();prepare.setEnabled(false);copy.setEnabled(false);draft.setText("Đang kiểm tra số dư, allowance và preflight đọc-only…");tasks.execute(()->{String result;try{result=DexTransaction.prepare(f,address,qty,reverse,bps);}catch(Exception e){result="CHẶN: "+(e instanceof IllegalArgumentException || e instanceof IllegalStateException?e.getMessage():"RPC/preflight thất bại; không tạo giao dịch");}final String text=result;ui.post(()->{if(token==generation && feed==f){draftForm=capturedForm;draft.setText(text);copy.setEnabled(text.startsWith("{"));prepare.setEnabled(true);}});});});
        copy.setOnClickListener(v->{try{org.json.JSONObject obj=new org.json.JSONObject(draft.getText().toString());if(feed==null || !feed.running || !draftForm.equals(form()) || feed.report(amount.getText().toString(),direction.getSelectedItemPosition()==1,slippage()).contains("CHẶN") || System.currentTimeMillis()/1000>=obj.getLong("deadline_utc_s")){draft.setText("Bản nháp hết hạn; tạo lại trước khi dùng ví");copy.setEnabled(false);return;}getSystemService(android.content.ClipboardManager.class).setPrimaryClip(ClipData.newPlainText("BIA unsigned DEX transaction",obj.toString(2)));Toast.makeText(this,"Đã sao chép bản chưa ký; chưa có giao dịch được gửi",Toast.LENGTH_LONG).show();}catch(Exception e){copy.setEnabled(false);}});
    }
    void stopFeed(){generation++;if(feed!=null){feed.stop();feed=null;}chain.setEnabled(true);pool.setEnabled(true);if(draft!=null)draft.setText("");if(copy!=null)copy.setEnabled(false);if(prepare!=null)prepare.setEnabled(true);output.setText("ĐÃ DỪNG — báo giá và bản nháp vô hiệu");}
    @Override protected void onResume(){super.onResume();ui.post(refresh);}
    @Override protected void onPause(){ui.removeCallbacks(refresh);stopFeed();super.onPause();}
    @Override protected void onDestroy(){tasks.shutdownNow();ui.removeCallbacksAndMessages(null);super.onDestroy();}
}
