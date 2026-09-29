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

/** Foreground preparation/signing; read-only recovery is separately OS-scheduled. */
public final class SolanaActivity extends Activity {
    final Handler ui=new Handler(Looper.getMainLooper());final ExecutorService worker=Executors.newSingleThreadExecutor();
    final SolanaWallet wallet=new SolanaWallet();SolanaRpc rpc=new SolanaRpc();TradeBook book;
    SolanaOrder order;SolanaPortfolio portfolio;volatile int generation;String reservation="";
    EditText amount,slip,fee,key,per,daily,loss,dd,exposure,count,cost;Spinner direction;
    TextView connection,market,receipt,analysis;Button connect,disconnect,watchButton,prepare,sign,status,configure;
    boolean busy,watching,foreground;String preparedForm="",pending="",lastSignature="";android.content.SharedPreferences journal;
    final Runnable refresh=new Runnable(){public void run(){if(watching && foreground && !busy && !wallet.busy())load(false);else if(watching)ui.postDelayed(this,15000);}};
    final Runnable recovery=new Runnable(){public void run(){if(foreground && !busy && !wallet.busy() && (book.hasOpen() || !journal.getString("pending","").isEmpty()))reconcile();if(foreground)ui.postDelayed(this,15000);}};
    EditText field(LinearLayout l,String hint,String value){TextView label=new TextView(this);label.setText(hint);l.addView(label);EditText e=new EditText(this);e.setSingleLine(true);e.setHint(hint);e.setText(value);e.setSaveEnabled(false);l.addView(e);return e;}
    Button button(LinearLayout l,String label){Button b=new Button(this);b.setText(label);l.addView(b);return b;}
    String form(){return direction.getSelectedItemPosition()+"|"+amount.getText()+"|"+slip.getText()+"|"+fee.getText()+"|"+wallet.owner();}
    void syncPending(){String old=journal.getString("pending","");pending=!old.isEmpty()?old:book.hasOpen()?"BOOK_PENDING":"";}
    @Override public void onCreate(Bundle state){super.onCreate(state);getWindow().addFlags(WindowManager.LayoutParams.FLAG_SECURE);
        journal=getSharedPreferences("solana-orders",MODE_PRIVATE);book=new TradeBook(this);book.recover();syncPending();lastSignature=journal.getString("signature","");if(book.hasOpen())TradeRecoveryService.schedule(this);
        ScrollView scroll=new ScrollView(this);LinearLayout root=new LinearLayout(this);root.setOrientation(1);root.setPadding(20,20,20,20);scroll.addView(root);setContentView(scroll);
        TextView intro=new TextView(this);intro.setText("BIA V137 • SOLANA MAINNET\nSOL ↔ USDC; tiền thật khi ký tại ví. Audit chỉ cho phép route đã giải mã; route mới bị chặn.\nKý từng lệnh tại ví. Không có quyền ký nền tự động; BIA không giữ khóa ví.");root.addView(intro);
        connect=button(root,"Liên kết ví Solana");disconnect=button(root,"Thu hồi kết nối tại ví");connection=new TextView(this);connection.setText("Chưa liên kết ví");connection.setTextIsSelectable(true);root.addView(connection);
        direction=new Spinner(this);direction.setAdapter(new ArrayAdapter<String>(this,android.R.layout.simple_spinner_dropdown_item,new String[]{"SOL → USDC","USDC → SOL"}));root.addView(direction);
        amount=field(root,"Số lượng token đầu vào","");amount.setInputType(InputType.TYPE_CLASS_NUMBER|InputType.TYPE_NUMBER_FLAG_DECIMAL);slip=field(root,"Slippage bps (0–100)","50");fee=field(root,"Ngân sách phí + rent SOL, tối đa 0.01", "0.005");
        key=field(root,"Jupiter API key tùy chọn (không phải khóa ví)","");key.setInputType(InputType.TYPE_CLASS_TEXT|InputType.TYPE_TEXT_VARIATION_PASSWORD);key.setImportantForAutofill(android.view.View.IMPORTANT_FOR_AUTOFILL_NO);
        per=field(root,"Giá trị tối đa mỗi lệnh, USDC","");daily=field(root,"Tổng lượng giao dịch tối đa/ngày, USDC","");loss=field(root,"Giảm tài sản tối đa/ngày, USDC","");dd=field(root,"Drawdown tối đa, %","");exposure=field(root,"Tỷ trọng SOL dự kiến tối đa sau mua, %","");count=field(root,"Số lệnh tối đa/ngày (giờ Việt Nam)","");
        configure=button(root,"Đọc danh mục & xem cấu hình hạn mức");configure.setOnClickListener(v->configure());
        cost=field(root,"Ngưỡng chi phí đánh giá tín hiệu, bps (tự nhập, không phải phí khớp)","100");
        watchButton=button(root,"Theo dõi giá chuẩn 0.001 SOL mỗi 15 giây");prepare=button(root,"Chuẩn bị lệnh + audit + kiểm tra rủi ro");sign=button(root,"Xem lệnh và mở ví ký");sign.setEnabled(false);
        button(root,"Dừng tác vụ hiện tại").setOnClickListener(v->stop());button(root,"DỪNG KHẨN — khóa giao dịch đến khi đặt lại mốc").setOnClickListener(v->{book.halt("Người dùng dừng khẩn");stop();});
        market=new TextView(this);market.setTextIsSelectable(true);root.addView(market);analysis=new TextView(this);root.addView(analysis);
        status=button(root,"Đối soát / xem nhật ký lệnh");receipt=new TextView(this);receipt.setTextIsSelectable(true);receipt.setText(book.report());root.addView(receipt);
        button(root,"Xuất nhật ký JSON").setOnClickListener(v->startActivityForResult(new Intent(Intent.ACTION_CREATE_DOCUMENT).setType("application/json").addCategory(Intent.CATEGORY_OPENABLE).putExtra(Intent.EXTRA_TITLE,"BIA-trading-audit.json"),137));
        button(root,"Mở giao dịch gần nhất trên Solscan").setOnClickListener(v->{if(!lastSignature.isEmpty())startActivity(new Intent(Intent.ACTION_VIEW,Uri.parse("https://solscan.io/tx/"+lastSignature)));});
        connect.setOnClickListener(v->connect(false));disconnect.setOnClickListener(v->connect(true));watchButton.setOnClickListener(v->{if(busy || wallet.busy())return;watching=true;ui.removeCallbacks(refresh);load(false);});
        prepare.setOnClickListener(v->{watching=false;ui.removeCallbacks(refresh);load(true);});sign.setOnClickListener(v->review());status.setOnClickListener(v->reconcile());
    }
    void controls(boolean value){busy=value;for(Button b:new Button[]{connect,disconnect,prepare,watchButton,status,configure})b.setEnabled(!value);direction.setEnabled(!value);for(EditText e:new EditText[]{amount,slip,fee,key,per,daily,loss,dd,exposure,count,cost})e.setEnabled(!value);syncPending();sign.setEnabled(!value && order!=null && order.transaction!=null && pending.isEmpty());}
    void connect(boolean revoke){if(busy || wallet.busy())return;watching=false;ui.removeCallbacks(refresh);order=null;controls(true);wallet.request(this,null,revoke,(address,bytes,error)->{controls(false);connection.setText(error==null?(address.isEmpty()?"Đã thu hồi kết nối":"Ví mainnet: "+address):error);});}
    void configure(){if(busy || wallet.owner().isEmpty()){market.setText("Liên kết ví trước");return;}syncPending();if(!pending.isEmpty()){market.setText("Còn lệnh chưa đối soát");return;}
        final long a,b,c;final int d,e,n;try{a=SolanaWire.units(per.getText().toString(),6).longValueExact();b=SolanaWire.units(daily.getText().toString(),6).longValueExact();c=SolanaWire.units(loss.getText().toString(),6).longValueExact();d=new BigDecimal(dd.getText().toString()).movePointRight(2).intValueExact();e=new BigDecimal(exposure.getText().toString()).movePointRight(2).intValueExact();n=Integer.parseInt(count.getText().toString());}catch(Exception ex){market.setText("Điền đầy đủ hạn mức hợp lệ");return;}
        final String owner=wallet.owner(),api=key.getText().toString().trim();final int g=++generation;final SolanaRpc r=rpc;order=null;controls(true);
        worker.execute(()->{try{SolanaPortfolio p=SolanaPortfolio.read(r,owner,api);ui.post(()->{if(g!=generation)return;controls(false);new AlertDialog.Builder(this).setTitle("Đặt hạn mức và mốc đo tài sản").setMessage(p.report()+"\nMỗi lệnh: "+per.getText()+" USDC; mỗi ngày: "+daily.getText()+" USDC\nGiảm tài sản: "+loss.getText()+" USDC; DD: "+dd.getText()+"%; SOL: "+exposure.getText()+"%; số lệnh: "+n+"\nĐặt lại mốc sẽ khởi tạo lại thước đo drawdown/lỗ, không xóa lệnh hay lượng đã dùng trong ngày. Không cấp quyền tự ký.").setNegativeButton("Hủy",null).setPositiveButton("Lưu cấu hình đã xem",(dlg,w)->{try{if(!owner.equals(wallet.owner()))throw new IllegalStateException("Ví đã đổi");book.configure(owner,a,b,c,d,e,n,p);market.setText("Đã lưu hạn mức. "+p.report());}catch(Exception x){market.setText(safe(x));}}).show();});}catch(Exception x){error(g,x);}});
    }
    void load(boolean trade){if(busy || wallet.busy())return;syncPending();if(trade && (wallet.owner().isEmpty() || !pending.isEmpty())){market.setText("Cần ví liên kết và không còn lệnh chờ");return;}
        final int bps;final long budget;final double costBps;try{bps=Integer.parseInt(slip.getText().toString());budget=SolanaWire.units(fee.getText().toString(),9).longValueExact();costBps=Double.parseDouble(cost.getText().toString());if(bps<0 || bps>100 || budget>10000000 || !Double.isFinite(costBps) || costBps<0 || costBps>10000)throw new IllegalArgumentException();}catch(Exception x){watching=false;market.setText("Kiểm tra slippage, phí và ngưỡng đánh giá");return;}
        final String qty=trade?amount.getText().toString():"0.001",api=key.getText().toString().trim(),owner=trade?wallet.owner():"",captured=form();final boolean sell=!trade || direction.getSelectedItemPosition()==0;final int g=++generation;final SolanaRpc r=rpc;order=null;controls(true);market.setText("Đang đọc dữ liệu thật…");
        worker.execute(()->{try{long slot=r.verifyMainnet();SolanaPortfolio p=trade?SolanaPortfolio.read(r,owner,api):null;SolanaOrder q=SolanaOrder.fetch(r,api,owner,sell,qty,bps,budget);if(trade){book.check(owner,q,p);q.preflight(r);}else{book.observe(System.currentTimeMillis(),new BigDecimal(q.outAmount).movePointLeft(6).divide(new BigDecimal("0.001")).doubleValue());}
                String quality=TradingNative.quality(book.observations(),System.currentTimeMillis(),costBps);ui.post(()->{if(g!=generation)return;order=q;portfolio=p;preparedForm=captured;market.setText("Slot confirmed: "+slot+"\n"+q.report()+(trade?"\nAudit route/ALT/tài khoản/delta và rủi ro: đạt\n"+p.report():""));analysis.setText(quality);controls(false);if(watching)ui.postDelayed(refresh,15000);});
            }catch(Exception x){error(g,x);}});
    }
    void error(int g,Exception x){ui.post(()->{if(g!=generation)return;order=null;market.setText("CHẶN: "+safe(x));controls(false);if(watching)ui.postDelayed(refresh,30000);});}
    void review(){if(busy || order==null || order.transaction==null)return;try{order.fresh();if(!preparedForm.equals(form()))throw new IllegalStateException("Thông số đổi; lấy lệnh mới");}catch(Exception x){order=null;sign.setEnabled(false);market.setText(safe(x));return;}final SolanaOrder q=order;
        new AlertDialog.Builder(this).setTitle("Giao dịch tiền thật").setMessage(q.report()+"\nVí: "+q.owner+"\nAudit không thay thế bảo đảm của chương trình/RPC. Xem nội dung tại ví. Sau ký, BIA gửi tối đa một lần.").setNegativeButton("Hủy",null).setPositiveButton("Kiểm tra lại và mở ví",(d,w)->requestSignature(q)).show();}
    void requestSignature(SolanaOrder q){if(busy || q!=order)return;final int g=++generation;final String api=key.getText().toString().trim();final SolanaRpc r=rpc;controls(true);
        worker.execute(()->{try{q.fresh();SolanaPortfolio p=SolanaPortfolio.read(r,q.owner,api);book.check(q.owner,q,p);q.fresh();if(g!=generation)return;String id=book.reserve(q,p);reservation=id;
            ui.post(()->{if(g!=generation){book.cancel(id);return;}wallet.request(this,q.transaction,false,(owner,bytes,error)->{
                if(g!=generation){book.cancel(id);return;}if(error!=null || bytes==null){book.cancel(id);reservation="";order=null;controls(false);market.setText(error==null?"Không có chữ ký":error);return;}
                try{q.fresh();SolanaWire.Transaction signed=q.transaction.signed(bytes,owner);String sig=signed.signature();book.signed(id,sig);lastSignature=sig;journal.edit().putString("signature",sig).apply();order=null;TradeRecoveryService.schedule(this);
                    worker.execute(()->{boolean attempted=false;try{SolanaPortfolio current=SolanaPortfolio.read(r,q.owner,api);book.recheck(id,q,current);q.preflight(r);if(g!=generation)throw new IllegalStateException("Đã dừng");q.fresh();book.dispatch(id);attempted=true;q.execute(r,api,signed);TradeReconcile.run(r,book);}catch(Exception x){try{if(attempted)book.mark(sig,"UNKNOWN","");else book.cancel(id);}catch(Exception ignored){}}finally{reservation="";TradeRecoveryService.schedule(this);ui.post(()->{if(g==generation){controls(false);receipt.setText(book.report());}});}});
                }catch(Exception x){book.cancel(id);reservation="";order=null;controls(false);market.setText("Chưa gửi: "+safe(x));}
            });});
        }catch(Exception x){error(g,x);}});
    }
    void reconcile(){if(busy || wallet.busy())return;final int g=++generation;final SolanaRpc r=rpc;controls(true);worker.execute(()->{try{
        String old=journal.getString("pending","");if(!old.isEmpty()){String state=r.status(old);if(state.equals("FINALIZED")||state.equals("FAILED"))journal.edit().remove("pending").commit();}
        TradeReconcile.run(r,book);ui.post(()->{if(g==generation){receipt.setText(book.report());controls(false);}});
    }catch(Exception x){ui.post(()->{if(g==generation){receipt.setText(book.report()+"\nChưa đối soát được: "+safe(x));controls(false);}});}});}
    @Override protected void onActivityResult(int request,int result,Intent data){super.onActivityResult(request,result,data);if(request==137 && result==RESULT_OK && data!=null && data.getData()!=null){final Uri uri=data.getData();worker.execute(()->{try(java.io.OutputStream out=getContentResolver().openOutputStream(uri,"wt")){if(out==null)throw new java.io.IOException();out.write(book.export().getBytes(java.nio.charset.StandardCharsets.UTF_8));ui.post(()->Toast.makeText(this,"Đã xuất nhật ký",Toast.LENGTH_LONG).show());}catch(Exception x){ui.post(()->Toast.makeText(this,"Không xuất được nhật ký",Toast.LENGTH_LONG).show());}});}}
    static String safe(Exception x){return x instanceof IllegalArgumentException || x instanceof IllegalStateException?String.valueOf(x.getMessage()):"Kết nối/định dạng/lưu trữ thất bại";}
    void stop(){generation++;watching=false;ui.removeCallbacks(refresh);wallet.cancel();if(!reservation.isEmpty())book.cancel(reservation);rpc.close();rpc=new SolanaRpc();order=null;key.setText("");controls(false);market.setText("Đã dừng tác vụ; không thể hủy lệnh đã phát. Dừng khẩn để khóa chính sách.");}
    @Override protected void onResume(){super.onResume();foreground=true;ui.removeCallbacks(recovery);ui.postDelayed(recovery,15000);}
    @Override protected void onPause(){foreground=false;watching=false;ui.removeCallbacks(refresh);ui.removeCallbacks(recovery);if(!wallet.busy()&&!busy){order=null;sign.setEnabled(false);key.setText("");}super.onPause();}
    @Override protected void onDestroy(){generation++;wallet.destroy();rpc.close();if(!reservation.isEmpty())book.cancel(reservation);worker.execute(book::close);worker.shutdown();ui.removeCallbacksAndMessages(null);super.onDestroy();}
}
