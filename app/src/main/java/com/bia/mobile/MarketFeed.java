package com.bia.mobile;

import org.json.*;
import okhttp3.*;
import java.io.*;
import java.time.*;
import java.time.format.DateTimeFormatter;
import java.util.*;
import java.util.concurrent.*;

/** Read-only market-data endpoints. No account, order, credential logging or execution API. */
final class MarketFeed {
    interface Listener {void changed();}
    static final class Instrument {
        final String symbol;
        final TreeMap<Long,double[]> candles=new TreeMap<>();
        double price;long eventMs,receivedNs;String detail="Đang chờ dữ liệu";
        Instrument(String s){symbol=s;}
        void candle(long end,double o,double h,double l,double c){
            if(end<=0 || !Double.isFinite(o+h+l+c) || l<=0 || l>Math.min(o,c) || h<Math.max(o,c))throw new IllegalArgumentException();
            candles.put(end,new double[]{end,o,h,l,c});while(candles.size()>60)candles.pollFirstEntry();
        }
        double[] data(){double[] a=new double[candles.size()*5];int n=0;for(double[] row:candles.values())for(double x:row)a[n++]=x;return a;}
    }
    final LinkedHashMap<String,Instrument> instruments=new LinkedHashMap<>();
    final OkHttpClient client=new OkHttpClient.Builder().connectTimeout(12,TimeUnit.SECONDS).readTimeout(20,TimeUnit.SECONDS).callTimeout(25,TimeUnit.SECONDS).retryOnConnectionFailure(false).followRedirects(false).followSslRedirects(false).build();
    final ScheduledExecutorService scheduler=Executors.newSingleThreadScheduledExecutor();
    final Listener listener;final boolean twelve;final String key;
    volatile boolean running,connected;volatile String status="Chưa kết nối";
    WebSocket socket;
    MarketFeed(boolean td,String symbols,String apiKey,Listener l){
        twelve=td;key=apiKey;listener=l;
        String[] list=symbols.toUpperCase(Locale.ROOT).split(",",-1);
        if(list.length<1 || list.length>3)throw new IllegalArgumentException("Nhập 1–3 mã, cách nhau dấu phẩy.");
        for(String raw:list){String s=raw.trim();if(!s.matches(td?"[A-Z0-9./:_-]{1,32}":"[A-Z0-9]{3,24}"))throw new IllegalArgumentException("Mã không hợp lệ.");instruments.put(s,new Instrument(s));}
        if(td && !key.matches("[A-Za-z0-9_-]{8,128}"))throw new IllegalArgumentException("Cần API key dữ liệu Twelve Data hợp lệ.");
    }
    synchronized void start(){
        if(running)return;running=true;status="Đang kết nối nguồn thật…";
        // Seed REST first; websocket updates cannot overwrite or race an older seed request.
        scheduler.execute(()->{for(String s:instruments.keySet())if(running)seed(s);lastRefreshMinute=System.currentTimeMillis()/60000;if(running)open();});
        if(twelve)scheduler.scheduleWithFixedDelay(()->{WebSocket ws=socket;if(running && ws!=null)ws.send("{\"action\":\"heartbeat\"}");},10,10,TimeUnit.SECONDS);
    }
    private void open(){
        String url;
        if(twelve)url="wss://ws.twelvedata.com/v1/quotes/price?apikey="+key;
        else {StringJoiner streams=new StringJoiner("/");for(String s:instruments.keySet()){streams.add(s.toLowerCase(Locale.ROOT)+"@aggTrade");streams.add(s.toLowerCase(Locale.ROOT)+"@kline_1m");}url="wss://data-stream.binance.vision:443/stream?streams="+streams;}
        synchronized(this){if(!running)return;socket=client.newWebSocket(new Request.Builder().url(url).build(),new WebSocketListener(){
            @Override public void onOpen(WebSocket ws,Response response){
                synchronized(MarketFeed.this){if(!running){ws.cancel();return;}connected=true;status="WebSocket mở — chờ timestamp/giá của nguồn";}
                if(twelve){try{ws.send(new JSONObject().put("action","subscribe").put("params",new JSONObject().put("symbols",String.join(",",instruments.keySet()))).toString());}catch(JSONException e){fail("Không tạo được đăng ký");}}
                listener.changed();
            }
            @Override public void onMessage(WebSocket ws,String text){
                if(!running)return;if(text.length()>262144){fail("Bản tin quá lớn");return;}
                try{accept(text,System.currentTimeMillis(),System.nanoTime());}catch(Exception e){fail("Dữ liệu nguồn không hợp lệ; đã đóng kết nối");}
            }
            @Override public void onFailure(WebSocket ws,Throwable t,Response response){fail("Kết nối thất bại / bị chặn. Bấm Kết nối để thử lại.");}
            @Override public void onClosing(WebSocket ws,int code,String reason){ws.close(code,null);fail("Nguồn đóng kết nối. Bấm Kết nối để làm mới dữ liệu.");}
        });}
    }
    synchronized void accept(String text,long now,long received)throws JSONException {
        if(!running)return;
        JSONObject obj=new JSONObject(text);
        if(twelve){
            String event=obj.optString("event");
            if("subscribe-status".equals(event)){
                JSONArray success=obj.optJSONArray("success"),fails=obj.optJSONArray("fails");
                status="Đăng ký nguồn: nhận "+(success==null?0:success.length())+", từ chối "+(fails==null?0:fails.length())+". Quyền realtime phụ thuộc gói dữ liệu.";
                if(fails!=null)for(int i=0;i<fails.length();i++){JSONObject f=fails.optJSONObject(i);if(f!=null){Instrument item=instruments.get(f.optString("symbol"));if(item!=null)item.detail="Nguồn từ chối đăng ký mã này";}}
            }else if("price".equals(event)){
                Instrument item=instruments.get(obj.getString("symbol"));if(item==null)return;
                tick(item,obj.getDouble("price"),Math.multiplyExact(obj.getLong("timestamp"),1000),now,received);
                // Refresh official completed candles once per minute, not synthetic OHLC from sparse ticks.
                long minute=item.eventMs/60000;
                if(minute>lastRefreshMinute){lastRefreshMinute=minute;scheduler.execute(()->{for(String s:instruments.keySet())if(running)seed(s);});}
            }else if("error".equals(event) || "error".equals(obj.optString("status"))){fail("Nguồn từ chối yêu cầu; kiểm tra key, mã và quyền dữ liệu.");return;}
        }else{
            JSONObject data=obj.has("data")?obj.getJSONObject("data"):obj;
            if(data.has("code")){fail("Binance từ chối đăng ký");return;}
            Instrument item=instruments.get(data.optString("s"));if(item==null)return;
            if("aggTrade".equals(data.optString("e")))tick(item,data.getDouble("p"),data.getLong("T"),now,received);
            else if("kline".equals(data.optString("e"))){JSONObject k=data.getJSONObject("k");if(k.getBoolean("x")){long end=k.getLong("T")+1;if(end<=now)item.candle(end,k.getDouble("o"),k.getDouble("h"),k.getDouble("l"),k.getDouble("c"));}}
        }
        listener.changed();
    }
    long lastRefreshMinute;
    private void tick(Instrument item,double price,long time,long now,long received){
        if(!Double.isFinite(price) || price<=0 || time<=0 || time>now+5000)throw new IllegalArgumentException();
        if(time<item.eventMs)return; // Reordered payload cannot make a quote fresh.
        if(time==item.eventMs && price==item.price)return;
        item.price=price;item.eventMs=time;item.receivedNs=received;item.detail="Giá nhận từ nguồn; timestamp không được thay bằng giờ máy";
    }
    void seed(String symbol){
        try{
            HttpUrl.Builder b=HttpUrl.get(twelve?"https://api.twelvedata.com/time_series":"https://data-api.binance.vision/api/v3/klines").newBuilder().addQueryParameter("symbol",symbol).addQueryParameter("interval","1min");
            if(twelve)b.addQueryParameter("outputsize","60").addQueryParameter("timezone","UTC").addQueryParameter("apikey",key);
            else {b.setQueryParameter("interval","1m").addQueryParameter("limit","61");}
            String text;
            try(Response r=client.newCall(new Request.Builder().url(b.build()).build()).execute()){
                if(!r.isSuccessful() || r.body()==null)throw new IOException();
                try(InputStream in=r.body().byteStream();ByteArrayOutputStream out=new ByteArrayOutputStream()){
                    byte[] buf=new byte[8192];int n;while((n=in.read(buf))!=-1){if(out.size()+n>262144)throw new IOException();out.write(buf,0,n);}text=out.toString("UTF-8");
                }
            }
            loadSeed(symbol,text,System.currentTimeMillis());
        }catch(Exception e){synchronized(this){if(running)instruments.get(symbol).detail="Không lấy được nến: kiểm tra quyền nguồn / mạng. Giá vẫn có thể cập nhật; phân tích sẽ chặn nếu thiếu nến.";}}
        if(running)listener.changed();
    }
    synchronized void loadSeed(String symbol,String text,long now)throws JSONException {
        if(!running)return;Instrument item=instruments.get(symbol);if(item==null)return;
        TreeMap<Long,double[]> rows=new TreeMap<>();
        JSONArray values;
        if(twelve){JSONObject root=new JSONObject(text);if(root.has("status") && !"ok".equals(root.getString("status")))throw new JSONException("source");values=root.getJSONArray("values");}
        else values=new JSONArray(text);
        for(int i=0;i<values.length();i++){
            long end;double o,h,l,c;
            if(twelve){JSONObject v=values.getJSONObject(i);end=LocalDateTime.parse(v.getString("datetime"),DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss")).toInstant(ZoneOffset.UTC).toEpochMilli()+60000;o=v.getDouble("open");h=v.getDouble("high");l=v.getDouble("low");c=v.getDouble("close");}
            else {JSONArray v=values.getJSONArray(i);end=v.getLong(6)+1;o=v.getDouble(1);h=v.getDouble(2);l=v.getDouble(3);c=v.getDouble(4);}
            if(end<=now)rows.put(end,new double[]{end,o,h,l,c});
        }
        for(double[] v:rows.values())item.candle((long)v[0],v[1],v[2],v[3],v[4]);
        item.detail="Nến nguồn đã cập nhật; chỉ dùng nến đã đóng";
    }
    synchronized String snapshot(){
        long now=System.currentTimeMillis(),mono=System.nanoTime();StringBuilder out=new StringBuilder(status).append("\nCHẾ ĐỘ: dữ liệu thật, chỉ đọc; đặt lệnh TẮT\n");
        for(Instrument i:instruments.values()){
            boolean fresh=running && connected && i.receivedNs>0 && mono-i.receivedNs<=15_000_000_000L;
            out.append("\n").append(i.symbol).append(" — ").append(i.price>0?String.format(Locale.US,"%.8f",i.price):"chưa có giá").append("\n");
            if(i.eventMs>0)out.append("Giờ nguồn: ").append(Instant.ofEpochMilli(i.eventMs)).append(" | tuổi ").append(Math.max(0,(now-i.eventMs)/1000)).append(" giây\n");
            out.append(i.detail).append("\n").append(TradingNative.analyze(i.data(),i.price,i.eventMs,now,fresh)).append("\n");
        }
        return out.toString();
    }
    private void fail(String reason){synchronized(this){if(!running)return;connected=false;status=reason;running=false;if(socket!=null)socket.cancel();}scheduler.shutdownNow();client.dispatcher().cancelAll();listener.changed();}
    synchronized void stop(){running=false;connected=false;status="ĐÃ DỪNG — mọi tín hiệu vô hiệu";if(socket!=null)socket.cancel();client.dispatcher().cancelAll();scheduler.shutdownNow();client.connectionPool().evictAll();client.dispatcher().executorService().shutdown();}
}
