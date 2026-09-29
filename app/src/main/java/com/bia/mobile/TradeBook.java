package com.bia.mobile;

import android.content.*;
import android.database.Cursor;
import android.database.sqlite.*;
import org.json.*;
import java.math.*;
import java.util.*;

/** Durable spend reservations and audit history. All amounts are integer micro-USDC/lamports. */
final class TradeBook extends SQLiteOpenHelper {
    static final String OPEN="('WAITING_SIGNATURE','SIGNED','DISPATCHING','UNKNOWN','PENDING','CONFIRMED','FAILED_PENDING')";
    TradeBook(Context c){this(c,"bia-trading-v137.db");}
    TradeBook(Context c,String name){super(c.getApplicationContext(),name,null,1);setWriteAheadLoggingEnabled(true);}
    public void onCreate(SQLiteDatabase db){
        db.execSQL("CREATE TABLE policy(owner TEXT PRIMARY KEY,per_order INTEGER,daily INTEGER,loss INTEGER,drawdown INTEGER,exposure INTEGER,max_count INTEGER,base INTEGER,peak INTEGER,day_base INTEGER,day INTEGER,sol TEXT,usdc TEXT,halt TEXT NOT NULL DEFAULT '')");
        db.execSQL("CREATE TABLE orders(id TEXT PRIMARY KEY,owner TEXT NOT NULL,day INTEGER,notional INTEGER,status TEXT NOT NULL,signature TEXT UNIQUE,created INTEGER,updated INTEGER,intent TEXT NOT NULL,receipt TEXT NOT NULL DEFAULT '')");
        db.execSQL("CREATE INDEX pending_orders ON orders(status)");
        db.execSQL("CREATE TABLE events(seq INTEGER PRIMARY KEY AUTOINCREMENT,time INTEGER,type TEXT,detail TEXT)");
        db.execSQL("CREATE TABLE observations(time INTEGER PRIMARY KEY,price REAL NOT NULL)");
    }
    public void onUpgrade(SQLiteDatabase db,int a,int b){throw new IllegalStateException("Cần migration rõ ràng");}
    static long day(long now){return (now+7*3600000L)/86400000L;}
    static void valid(long n){if(n<=0)throw new IllegalArgumentException("Hạn mức phải dương");}
    synchronized void event(String type,String detail){ContentValues v=new ContentValues();v.put("time",System.currentTimeMillis());v.put("type",type);v.put("detail",detail);getWritableDatabase().insertOrThrow("events",null,v);}
    synchronized void configure(String owner,long per,long daily,long loss,int dd,int exposure,int count,SolanaPortfolio p)throws Exception{
        SolanaWire.address(owner);valid(per);valid(daily);valid(loss);if(per>daily || dd<1 || dd>10000 || exposure<1 || exposure>10000 || count<1 || count>1000)throw new IllegalArgumentException("Hạn mức không hợp lệ");p.fresh();valid(p.equity());
        SQLiteDatabase db=getWritableDatabase();db.beginTransaction();try{if(hasOpen(db))throw new IllegalStateException("Còn lệnh chưa đối soát");ContentValues v=new ContentValues();v.put("owner",owner);v.put("per_order",per);v.put("daily",daily);v.put("loss",loss);v.put("drawdown",dd);v.put("exposure",exposure);v.put("max_count",count);v.put("base",p.equity());v.put("peak",p.equity());v.put("day_base",p.equity());v.put("day",day(System.currentTimeMillis()));v.put("sol",p.sol.toString());v.put("usdc",p.usdc.toString());v.put("halt","");db.insertWithOnConflict("policy",null,v,SQLiteDatabase.CONFLICT_REPLACE);event("POLICY_BASELINE",owner+" per="+per+" daily="+daily+" loss="+loss+" ddBps="+dd+" exposureBps="+exposure+" count="+count+" equity="+p.equity());db.setTransactionSuccessful();}finally{db.endTransaction();}
    }
    static boolean hasOpen(SQLiteDatabase db){try(Cursor c=db.rawQuery("SELECT 1 FROM orders WHERE status IN "+OPEN+" LIMIT 1",null)){return c.moveToFirst();}}
    synchronized boolean hasOpen(){return hasOpen(getReadableDatabase());}
    static long num(Cursor c,String name){return c.getLong(c.getColumnIndexOrThrow(name));}
    static String str(Cursor c,String name){return c.getString(c.getColumnIndexOrThrow(name));}
    synchronized long check(String owner,SolanaOrder q,SolanaPortfolio p)throws Exception{return checkLocked(getWritableDatabase(),owner,q,p,"");}
    private long checkLocked(SQLiteDatabase db,String owner,SolanaOrder q,SolanaPortfolio p,String excluded)throws Exception{
        p.fresh();boolean other;try(Cursor open=db.rawQuery("SELECT 1 FROM orders WHERE status IN "+OPEN+" AND id<>? LIMIT 1",new String[]{excluded})){other=open.moveToFirst();}if(other)throw new IllegalStateException("Lệnh chưa hoàn tất; không cấp lệnh thứ hai");
        try(Cursor c=db.rawQuery("SELECT * FROM policy WHERE owner=?",new String[]{owner})){if(!c.moveToFirst())throw new IllegalStateException("Chưa thiết lập hạn mức và mốc tài sản cho ví này");String halt=str(c,"halt");if(!halt.isEmpty())throw new IllegalStateException("DỪNG RỦI RO: "+halt);
            long now=System.currentTimeMillis(),today=day(now),stored=num(c,"day");if(today<stored)throw new IllegalStateException("Đồng hồ lùi ngày");
            String reason="";if(!p.sol.toString().equals(str(c,"sol")) || !p.usdc.toString().equals(str(c,"usdc")))reason="Số dư thay đổi ngoài nhật ký; kiểm tra nạp/rút rồi đặt lại mốc";
            long eq=p.equity(),peak=Math.max(num(c,"peak"),eq),base=today==stored?num(c,"day_base"):eq;
            if(base-eq>=num(c,"loss"))reason="Chạm giới hạn giảm tài sản trong ngày";
            if(BigInteger.valueOf(peak-eq).multiply(BigInteger.valueOf(10000)).compareTo(BigInteger.valueOf(peak).multiply(BigInteger.valueOf(num(c,"drawdown"))))>=0)reason="Chạm giới hạn drawdown";
            ContentValues update=new ContentValues();update.put("day",today);update.put("day_base",base);update.put("peak",peak);if(!reason.isEmpty())update.put("halt",reason);db.update("policy",update,"owner=?",new String[]{owner});if(!reason.isEmpty())throw new IllegalStateException(reason);
            long n=q.input.equals(SolanaOrder.USDC)?q.inAmount.longValueExact():p.value(q.inAmount);if(n>num(c,"per_order"))throw new IllegalStateException("Vượt hạn mức mỗi lệnh");
            try(Cursor sum=db.rawQuery("SELECT COALESCE(SUM(notional),0),COUNT(*) FROM orders WHERE owner=? AND day=? AND status NOT IN ('NOT_SUBMITTED','CANCELED') AND id<>?",new String[]{owner,Long.toString(today),excluded})){sum.moveToFirst();if(Math.addExact(sum.getLong(0),n)>num(c,"daily") || sum.getLong(1)>=num(c,"max_count"))throw new IllegalStateException("Vượt tổng lượng/số lệnh trong ngày");}
            if(q.input.equals(SolanaOrder.USDC)){BigInteger maxOut=q.outAmount.max(q.minOut);long projected=Math.addExact(p.value(p.sol),p.value(maxOut));if(BigInteger.valueOf(projected).multiply(BigInteger.valueOf(10000)).compareTo(BigInteger.valueOf(eq).multiply(BigInteger.valueOf(num(c,"exposure"))))>0)throw new IllegalStateException("Vượt tỷ trọng SOL cho phép");}
            return n;
        }
    }
    synchronized String reserve(SolanaOrder q,SolanaPortfolio p)throws Exception{
        check(q.owner,q,p);SQLiteDatabase db=getWritableDatabase();db.beginTransaction();try{long n=checkLocked(db,q.owner,q,p,"");ContentValues v=new ContentValues();v.put("id",q.requestId);v.put("owner",q.owner);v.put("day",day(System.currentTimeMillis()));v.put("notional",n);v.put("status","WAITING_SIGNATURE");v.put("created",System.currentTimeMillis());v.put("updated",System.currentTimeMillis());v.put("intent",SolanaRpc.object("input",q.input,"output",q.output,"amount",q.inAmount.toString(),"minOut",q.minOut.toString(),"feeBudget",q.feeBudget,"blockhash",q.transaction.blockhash).toString());db.insertOrThrow("orders",null,v);event("RESERVE",q.requestId);db.setTransactionSuccessful();return q.requestId;}finally{db.endTransaction();}
    }
    synchronized void recheck(String id,SolanaOrder q,SolanaPortfolio p)throws Exception{
        checkLocked(getWritableDatabase(),q.owner,q,p,id);SQLiteDatabase db=getWritableDatabase();db.beginTransaction();try{long n=checkLocked(db,q.owner,q,p,id);ContentValues v=new ContentValues();v.put("notional",n);v.put("day",day(System.currentTimeMillis()));if(db.update("orders",v,"id=? AND status='SIGNED'",new String[]{id})!=1)throw new IllegalStateException("Không còn lệnh đã ký hợp lệ");db.setTransactionSuccessful();}finally{db.endTransaction();}
    }
    synchronized void signed(String id,String signature){if(!signature.matches("[1-9A-HJ-NP-Za-km-z]{64,88}"))throw new IllegalArgumentException("Chữ ký");ContentValues v=new ContentValues();v.put("signature",signature);v.put("status","SIGNED");v.put("updated",System.currentTimeMillis());if(getWritableDatabase().update("orders",v,"id=? AND status='WAITING_SIGNATURE'",new String[]{id})!=1)throw new IllegalStateException("Lệnh không còn chờ ký");event("SIGNED",id+" "+signature);}
    synchronized void dispatch(String id){ContentValues v=new ContentValues();v.put("status","DISPATCHING");v.put("updated",System.currentTimeMillis());if(getWritableDatabase().update("orders",v,"id=? AND status='SIGNED' AND owner IN (SELECT owner FROM policy WHERE halt='')",new String[]{id})!=1)throw new IllegalStateException("Lệnh đã gửi/hủy hoặc dừng khẩn; không phát lại");event("DISPATCH_ONCE",id);}
    synchronized void cancel(String id){getWritableDatabase().execSQL("UPDATE orders SET status='NOT_SUBMITTED',updated=? WHERE id=? AND status IN ('WAITING_SIGNATURE','SIGNED')",new Object[]{System.currentTimeMillis(),id});}
    synchronized void recover(){getWritableDatabase().execSQL("UPDATE orders SET status='NOT_SUBMITTED' WHERE status IN ('WAITING_SIGNATURE','SIGNED')");getWritableDatabase().execSQL("UPDATE orders SET status='UNKNOWN' WHERE status='DISPATCHING'");}
    synchronized void mark(String signature,String state,String receipt)throws Exception{
        if(!Arrays.asList("UNKNOWN","PENDING","CONFIRMED","FINALIZED","FAILED","FAILED_PENDING").contains(state))throw new IllegalArgumentException("Trạng thái");SQLiteDatabase db=getWritableDatabase();db.beginTransaction();try{
            if(state.equals("FINALIZED") || state.equals("FAILED")){
                if(receipt.isEmpty())throw new IllegalStateException("Cần receipt finalized trước khi đóng lệnh");JSONObject r=new JSONObject(receipt);if(!signature.equals(r.getString("signature")) || r.getBoolean("failed")!=state.equals("FAILED"))throw new IllegalStateException("Receipt sai chữ ký/trạng thái");
                try(Cursor c=db.rawQuery("SELECT owner,intent FROM orders WHERE signature=? AND status IN "+OPEN,new String[]{signature})){if(!c.moveToFirst())return;String owner=c.getString(0);if(!owner.equals(r.getString("owner")))throw new IllegalStateException("Receipt sai ví");
                    try(Cursor p=db.rawQuery("SELECT sol,usdc FROM policy WHERE owner=?",new String[]{owner})){if(p.moveToFirst()){
                        String expectedSol=p.getString(0),expectedUsdc=p.getString(1);ContentValues v=new ContentValues();BigInteger solDelta=new BigInteger(r.getString("solDelta")),uDelta=new BigInteger(r.getString("usdcDelta"));
                        JSONObject intent=new JSONObject(c.getString(1));BigInteger amount=new BigInteger(intent.getString("amount")),minimum=new BigInteger(intent.getString("minOut")),budget=BigInteger.valueOf(intent.getLong("feeBudget"));
                        boolean bad=r.getBoolean("failed") ? uDelta.signum()!=0 || solDelta.negate().compareTo(budget)>0 : intent.getString("input").equals(SolanaOrder.SOL) ? solDelta.negate().compareTo(amount.add(budget))>0 || uDelta.compareTo(minimum)<0 : uDelta.negate().compareTo(amount)>0 || solDelta.compareTo(minimum.subtract(budget))<0;
                        if(bad)v.put("halt","Receipt vượt giới hạn ý định đã duyệt");
                        if(!expectedSol.equals(r.getString("preSol")))v.put("halt","Số dư SOL ngoài nhật ký trước giao dịch");
                        v.put("sol",new BigInteger(expectedSol).add(solDelta).toString());v.put("usdc",new BigInteger(expectedUsdc).add(uDelta).toString());db.update("policy",v,"owner=?",new String[]{owner});
                    }}
                }
            }
            ContentValues v=new ContentValues();v.put("status",state);v.put("receipt",receipt);v.put("updated",System.currentTimeMillis());db.update("orders",v,"signature=? AND status IN "+OPEN,new String[]{signature});event("RECONCILE",signature+" "+state);db.setTransactionSuccessful();
        }finally{db.endTransaction();}
    }
    synchronized ArrayList<String[]> pending(){ArrayList<String[]> out=new ArrayList<>();try(Cursor c=getReadableDatabase().rawQuery("SELECT signature,owner FROM orders WHERE status IN ('DISPATCHING','UNKNOWN','PENDING','CONFIRMED','FAILED_PENDING') AND signature IS NOT NULL",null)){while(c.moveToNext())out.add(new String[]{c.getString(0),c.getString(1)});}return out;}
    synchronized void halt(String reason){ContentValues v=new ContentValues();v.put("halt",reason);getWritableDatabase().update("policy",v,null,null);event("EMERGENCY_HALT",reason);}
    synchronized String report(){StringBuilder s=new StringBuilder("Nhật ký lệnh (không có khóa/chữ ký payload)\n");try(Cursor c=getReadableDatabase().rawQuery("SELECT created,status,notional,signature,receipt FROM orders ORDER BY created DESC LIMIT 20",null)){while(c.moveToNext())s.append(new java.util.Date(c.getLong(0))).append(" | ").append(c.getString(1)).append(" | ").append(SolanaWire.display(BigInteger.valueOf(c.getLong(2)),6)).append(" USDC\n").append(c.getString(3)).append("\n").append(c.getString(4)).append("\n");}return s.toString();}
    synchronized String export()throws Exception{JSONObject root=SolanaRpc.object("schema","BIA_TRADING_AUDIT_V137","exported",System.currentTimeMillis());for(String table:new String[]{"orders","events","policy"}){JSONArray rows=new JSONArray();try(Cursor c=getReadableDatabase().rawQuery("SELECT * FROM "+table+" LIMIT 10000",null)){while(c.moveToNext()){JSONObject row=new JSONObject();for(int i=0;i<c.getColumnCount();i++)row.put(c.getColumnName(i),c.isNull(i)?JSONObject.NULL:c.getString(i));rows.put(row);}}root.put(table,rows);try(Cursor count=getReadableDatabase().rawQuery("SELECT COUNT(*) FROM "+table,null)){count.moveToFirst();root.put(table+"_total",count.getLong(0));root.put(table+"_truncated",count.getLong(0)>rows.length());}}return root.toString(2);}
    synchronized void observe(long time,double price){if(!Double.isFinite(price)||price<=0)return;ContentValues v=new ContentValues();v.put("time",time);v.put("price",price);getWritableDatabase().insertWithOnConflict("observations",null,v,SQLiteDatabase.CONFLICT_IGNORE);getWritableDatabase().execSQL("DELETE FROM observations WHERE time < ?",new Object[]{time-86400000L});}
    synchronized double[] observations(){ArrayList<Double> a=new ArrayList<>();try(Cursor c=getReadableDatabase().rawQuery("SELECT time,price FROM (SELECT time,price FROM observations ORDER BY time DESC LIMIT 240) ORDER BY time",null)){while(c.moveToNext()){long time=c.getLong(0);if(!a.isEmpty() && (time-a.get(a.size()-2)>60000 || time-a.get(a.size()-2)<5000))a.clear();a.add((double)time);a.add(c.getDouble(1));}}double[] out=new double[a.size()];for(int i=0;i<out.length;i++)out[i]=a.get(i);return out;}
}
