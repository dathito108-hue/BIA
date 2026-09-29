package com.bia.mobile;

import android.app.*;
import android.os.*;
import android.content.*;
import android.net.Uri;
import android.text.InputType;
import android.view.WindowManager;
import android.widget.*;
import org.json.*;
import java.math.*;
import java.util.concurrent.*;

/** Foreground terminal. Each live order requires an explicit review and the external wallet. */
public final class SolanaActivity extends Activity {
    final Handler ui=new Handler(Looper.getMainLooper());
    final ExecutorService worker=Executors.newSingleThreadExecutor();
    final SolanaWallet wallet=new SolanaWallet();
    SolanaRpc rpc=new SolanaRpc();SolanaOrder order;volatile int generation;
    EditText amount,slip,fee,key;Spinner direction;TextView connection,market,receipt;
    Button connect,disconnect,watchButton,prepare,sign,status;boolean busy,watching,foreground;
    String preparedForm="",pending="",lastSignature="";android.content.SharedPreferences journal;
    final Runnable refresh=new Runnable(){public void run(){if(watching && foreground && !busy && !wallet.busy())load(false);else if(watching)ui.postDelayed(this,15000);}};
    EditText field(LinearLayout l,String hint,String value){EditText e=new EditText(this);e.setSingleLine(true);e.setHint(hint);e.setText(value);e.setSaveEnabled(false);l.addView(e);return e;}
    Button button(LinearLayout l,String label){Button b=new Button(this);b.setText(label);l.addView(b);return b;}
    String form(){return direction.getSelectedItemPosition()+"|"+amount.getText()+"|"+slip.getText()+"|"+fee.getText()+"|"+wallet.owner();}
    @Override public void onCreate(Bundle state){super.onCreate(state);getWindow().addFlags(WindowManager.LayoutParams.FLAG_SECURE);
        journal=getSharedPreferences("solana-orders",MODE_PRIVATE);pending=journal.getString("pending","");lastSignature=journal.getString("signature","");
        ScrollView scroll=new ScrollView(this);LinearLayout root=new LinearLayout(this);root.setOrientation(1);root.setPadding(20,20,20,20);scroll.addView(root);setContentView(scroll);
        TextView intro=new TextView(this);intro.setText("BIA • SOLANA MAINNET\nSOL ↔ USDC qua Jupiter/Metis. Tiền thật khi bạn ký tại ví. BIA không giữ seed/private key.\nTự động đọc báo giá mỗi 15 giây; mọi lệnh cần bạn xem và ký tại ví. Chưa hỗ trợ ký nền không cần xác nhận.");root.addView(intro);
        connect=button(root,"Liên kết ví Solana");disconnect=button(root,"Thu hồi kết nối tại ví");connection=new TextView(this);connection.setText("Chưa liên kết ví");connection.setTextIsSelectable(true);root.addView(connection);
        direction=new Spinner(this);direction.setAdapter(new ArrayAdapter<String>(this,android.R.layout.simple_spinner_dropdown_item,new String[]{"SOL → USDC","USDC → SOL"}));root.addView(direction);
        amount=field(root,"Số lượng token đầu vào","");amount.setInputType(InputType.TYPE_CLASS_NUMBER|InputType.TYPE_NUMBER_FLAG_DECIMAL);
        slip=field(root,"Slippage bps (0–100)","50");fee=field(root,"Ngân sách phí mạng + rent, SOL (tối đa 0.01)","0.005");
        key=field(root,"Jupiter API key (tùy chọn; chỉ giữ trong phiên)","");key.setInputType(InputType.TYPE_CLASS_TEXT|InputType.TYPE_TEXT_VARIATION_PASSWORD);key.setImportantForAutofill(android.view.View.IMPORTANT_FOR_AUTOFILL_NO);
        watchButton=button(root,"Theo dõi báo giá thật");prepare=button(root,"Lấy lệnh mới & kiểm tra mainnet");sign=button(root,"Xem lệnh rồi yêu cầu ví ký");sign.setEnabled(false);
        button(root,"Dừng / vô hiệu lệnh chưa gửi").setOnClickListener(v->stop());
        market=new TextView(this);market.setTextIsSelectable(true);root.addView(market);
        status=button(root,"Đối soát giao dịch gần nhất trên chuỗi");receipt=new TextView(this);receipt.setTextIsSelectable(true);receipt.setText(journal.getString("last","Chưa có giao dịch gửi từ BIA"));root.addView(receipt);
        button(root,"Mở giao dịch gần nhất trên Solscan").setOnClickListener(v->{if(!lastSignature.isEmpty())startActivity(new Intent(Intent.ACTION_VIEW,Uri.parse("https://solscan.io/tx/"+lastSignature)));});
        connect.setOnClickListener(v->connect(false));disconnect.setOnClickListener(v->connect(true));
        watchButton.setOnClickListener(v->{if(busy || wallet.busy())return;watching=true;ui.removeCallbacks(refresh);load(false);});
        prepare.setOnClickListener(v->{watching=false;ui.removeCallbacks(refresh);load(true);});
        sign.setOnClickListener(v->review());status.setOnClickListener(v->reconcile());
    }
    void controls(boolean value){busy=value;connect.setEnabled(!value);disconnect.setEnabled(!value);prepare.setEnabled(!value);watchButton.setEnabled(!value);status.setEnabled(!value);direction.setEnabled(!value);amount.setEnabled(!value);slip.setEnabled(!value);fee.setEnabled(!value);key.setEnabled(!value);sign.setEnabled(!value && order!=null && order.transaction!=null && pending.isEmpty());}
    void connect(boolean revoke){if(busy || wallet.busy())return;watching=false;ui.removeCallbacks(refresh);order=null;controls(true);
        wallet.request(this,null,revoke,(address,bytes,error)->{controls(false);connection.setText(error==null?(address.isEmpty()?"Đã thu hồi kết nối":"Ví mainnet: "+address):error);});}
    void load(boolean trade){if(busy || wallet.busy())return;
        if(trade && (wallet.owner().isEmpty() || !pending.isEmpty())){market.setText(wallet.owner().isEmpty()?"Liên kết ví trước khi tạo lệnh":"Có giao dịch chưa đối soát; không tạo lệnh mới");return;}
        final int bps;final long budget;try{bps=Integer.parseInt(slip.getText().toString());budget=SolanaWire.units(fee.getText().toString(),9).longValueExact();if(bps<0 || bps>100 || budget>10000000)throw new IllegalArgumentException();}catch(Exception e){watching=false;market.setText("Kiểm tra slippage 0–100 bps và ngân sách phí tối đa 0.01 SOL");return;}
        final String qty=amount.getText().toString(),apiKey=key.getText().toString().trim(),owner=trade?wallet.owner():"",captured=form();final boolean sell=direction.getSelectedItemPosition()==0;
        final int g=++generation;final SolanaRpc r=rpc;order=null;controls(true);market.setText("Đang đọc mạng và báo giá thật…");
        worker.execute(()->{try{long slot=r.verifyMainnet();SolanaOrder q=SolanaOrder.fetch(r,apiKey,owner,sell,qty,bps,budget);long actualFee=trade?q.preflight(r):-1;
                ui.post(()->{if(g!=generation)return;order=q;preparedForm=captured;market.setText("Slot confirmed: "+slot+"\n"+q.report()+(trade?"\nPreflight đạt; phí RPC: "+actualFee+" lamports":""));controls(false);if(watching)ui.postDelayed(refresh,15000);});
            }catch(Exception e){ui.post(()->{if(g!=generation)return;order=null;market.setText("CHẶN: "+safe(e));controls(false);if(watching)ui.postDelayed(refresh,15000);});}});
    }
    void review(){if(busy || order==null || order.transaction==null || !pending.isEmpty())return;try{order.fresh();if(!preparedForm.equals(form()))throw new IllegalStateException("Thông số đã đổi; lấy lệnh mới");}catch(Exception e){order=null;sign.setEnabled(false);market.setText(safe(e));return;}
        final SolanaOrder q=order;new AlertDialog.Builder(this).setTitle("Xem giao dịch tiền thật").setMessage(q.report()+"\n\nVí: "+q.owner+"\nBIA chưa giải mã/kiểm toán toàn bộ chỉ thị Jupiter. Chỉ ký nếu màn hình ví xác nhận đúng tài sản, lượng và người nhận. Sau khi ký, BIA sẽ gửi lệnh một lần.").setNegativeButton("Hủy",null).setPositiveButton("Mở ví để kiểm tra & ký",(d,w)->requestSignature(q)).show();
    }
    void requestSignature(SolanaOrder q){if(busy || q!=order || !pending.isEmpty())return;try{q.fresh();if(!preparedForm.equals(form()))throw new IllegalStateException("Thông số đã đổi");}catch(Exception e){market.setText(safe(e));return;}
        final int g=++generation;final String apiKey=key.getText().toString().trim();controls(true);
        wallet.request(this,q.transaction,false,(address,bytes,error)->{
            if(g!=generation)return;if(error!=null || bytes==null){order=null;controls(false);market.setText(error==null?"Không nhận được chữ ký":error);return;}
            try {q.fresh();SolanaWire.Transaction signed=q.transaction.signed(bytes,address);String sig=signed.signature();
                // Persist identity BEFORE network submission; never persist a signed transaction or auth key.
                if(!journal.edit().putString("pending",sig).putString("signature",sig).putString("last","PREPARED_SIGNED / chưa biết đã gửi: "+sig).commit())throw new IllegalStateException("Không ghi được nhật ký; không gửi");
                pending=sig;lastSignature=sig;order=null;receipt.setText("Đã ký; đang kiểm tra lại trước khi gửi: "+sig);final SolanaRpc r=rpc;
                worker.execute(()->{boolean attempted=false;try{q.preflight(r);if(g!=generation)throw new IllegalStateException("Đã dừng trước khi gửi");q.fresh();attempted=true;
                        JSONObject result=q.execute(r,apiKey,signed);
                        if(!sig.equals(result.optString("signature",sig)))throw new IllegalStateException("Chữ ký dịch vụ trả về không khớp; cần đối soát");
                        String chain=r.status(sig);record(sig,chain);ui.post(()->{if(g==generation){controls(false);receipt.setText(chain+"\n"+sig);}});
                    }catch(Exception e){final String reason=attempted?"UNKNOWN — có thể đã gửi; đối soát, không gửi lại":"NOT_SUBMITTED — "+safe(e);record(sig,attempted?"UNKNOWN":"NOT_SUBMITTED");ui.post(()->{if(g==generation){controls(false);receipt.setText(reason+"\n"+sig);}});}
                });
            }catch(Exception e){order=null;controls(false);market.setText("Không gửi: "+safe(e));}
        });
    }
    synchronized void record(String sig,String state){boolean terminal=state.equals("CONFIRMED") || state.equals("FINALIZED") || state.equals("FAILED") || state.equals("NOT_SUBMITTED");
        android.content.SharedPreferences.Editor e=journal.edit().putString("last",state+"\n"+sig).putString("signature",sig);if(terminal)e.remove("pending");e.commit();
        ui.post(()->{pending=journal.getString("pending","");});
    }
    void reconcile(){if(busy || lastSignature.isEmpty())return;final String sig=lastSignature;final SolanaRpc r=rpc;final int g=++generation;controls(true);
        worker.execute(()->{try{r.verifyMainnet();String state=r.status(sig);record(sig,state);ui.post(()->{if(g==generation){receipt.setText(state+"\n"+sig);controls(false);}});}catch(Exception e){ui.post(()->{if(g==generation){receipt.setText("Đối soát chưa thành công; giữ trạng thái cũ. "+safe(e));controls(false);}});}});
    }
    static String safe(Exception e){return e instanceof IllegalArgumentException || e instanceof IllegalStateException?String.valueOf(e.getMessage()):"Kết nối/định dạng dữ liệu thất bại";}
    void stop(){generation++;watching=false;ui.removeCallbacks(refresh);wallet.cancel();rpc.close();rpc=new SolanaRpc();order=null;key.setText("");controls(false);market.setText("Đã dừng. Lệnh đã gửi không thể thu hồi; đối soát bằng chữ ký.");}
    @Override protected void onResume(){super.onResume();foreground=true;}
    @Override protected void onPause(){foreground=false;watching=false;ui.removeCallbacks(refresh);if(!wallet.busy() && !busy){order=null;sign.setEnabled(false);key.setText("");}super.onPause();}
    @Override protected void onDestroy(){generation++;wallet.destroy();rpc.close();worker.shutdownNow();ui.removeCallbacksAndMessages(null);super.onDestroy();}
}
