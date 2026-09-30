package com.bia.mobile;
import org.json.*;
import okhttp3.*;
import java.math.*;
import java.util.*;
import java.util.concurrent.*;
import java.io.*;
import java.time.Instant;

/** Explicit whitelist of read-only RPC methods and verified V2 factories. */
final class DexFeed {
    static final String[] RPC={"https://ethereum-rpc.publicnode.com","https://bsc-rpc.publicnode.com"};
    static final String[] FACTORY={"0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f","0xca143ce32fe78f1f7019d7d551a6402fc5350c73"};
    static final String[] ROUTERS={"0x7a250d5630b4cf539739df2c5dacb4c659f2488d","0x10ed43c718714eb63d5aa57b78b54704e256024e"};
    static final int[] CHAINS={1,56},FEES={30,25};
    static final String[] NAMES={"Ethereum / Uniswap V2","BNB Chain / PancakeSwap V2"};
    static final class Snapshot {
        String block,hash,token0,token1;long blockMs,reserveMs,receivedNs;int decimals0,decimals1;BigInteger r0,r1,gas;boolean balancesMatch;
    }
    final int network;final String pool;
    final OkHttpClient client=new OkHttpClient.Builder().connectTimeout(10,TimeUnit.SECONDS).readTimeout(12,TimeUnit.SECONDS).callTimeout(15,TimeUnit.SECONDS).retryOnConnectionFailure(false).followRedirects(false).followSslRedirects(false).build();
    final ScheduledExecutorService worker=Executors.newSingleThreadScheduledExecutor();
    final ArrayDeque<Double> spots=new ArrayDeque<>();long lastBlockMs;double lastLiquidity;volatile boolean liquidityDrop;
    boolean cleanupStarted;volatile boolean running;volatile Snapshot latest;volatile String status="Chưa kết nối";
    DexFeed(int n,String p){if(n<0 || n>=2 || !p.matches("0x[0-9a-fA-F]{40}"))throw new IllegalArgumentException("Chọn mạng và địa chỉ pool V2 hợp lệ");network=n;pool=p.toLowerCase(Locale.ROOT);}
    void start(){running=true;status="Đang đọc blockchain thật…";worker.scheduleWithFixedDelay(this::refresh,0,15,TimeUnit.SECONDS);}
    Object rpc(String method,JSONArray params)throws Exception {return CoreSkills.call("dex.read",()->rpcCore(method,params));}
    Object rpcCore(String method,JSONArray params)throws Exception {
        if(!running)throw new IOException("stopped");
        if(!Arrays.asList("eth_chainId","eth_getBlockByNumber","eth_call","eth_gasPrice","eth_estimateGas").contains(method))throw new SecurityException();
        JSONObject body=new JSONObject().put("jsonrpc","2.0").put("id",1).put("method",method).put("params",params);
        Request request=new Request.Builder().url(RPC[network]).post(RequestBody.create(MediaType.get("application/json"),body.toString())).build();
        try(Response r=client.newCall(request).execute()){
            if(!r.isSuccessful() || r.body()==null)throw new IOException("HTTP "+r.code());
            try(InputStream in=r.body().byteStream();ByteArrayOutputStream out=new ByteArrayOutputStream()){
                byte[] b=new byte[4096];int count;while((count=in.read(b))!=-1){if(out.size()+count>131072)throw new IOException("large");out.write(b,0,count);}
                JSONObject obj=new JSONObject(out.toString("UTF-8"));if(obj.has("error") || obj.isNull("result"))throw new IOException("RPC rejected");return obj.get("result");
            }
        }
    }
    String call(String address,String data,String block)throws Exception {return (String)rpc("eth_call",new JSONArray().put(new JSONObject().put("to",address).put("data",data)).put(block));}
    static BigInteger hex(String value){if(!value.matches("0x[0-9a-fA-F]{1,64}"))throw new IllegalArgumentException("RPC hex");return new BigInteger(value.substring(2),16);}
    static BigInteger word(String hex,int n){if(!hex.matches("0x[0-9a-fA-F]+") || hex.length()<2+(n+1)*64)throw new IllegalArgumentException("ABI");return new BigInteger(hex.substring(2+n*64,2+(n+1)*64),16);}
    static String address(String encoded){BigInteger value=word(encoded,0);if(value.signum()==0 || value.bitLength()>160)throw new IllegalArgumentException("address");return "0x"+String.format(Locale.ROOT,"%040x",value);}
    static String arg(String address){return "000000000000000000000000"+address.substring(2);}
    void refresh(){
        if(!running)return;
        try{
            if(hex((String)rpc("eth_chainId",new JSONArray())).intValueExact()!=CHAINS[network])throw new IOException("Sai chainId");
            JSONObject head=(JSONObject)rpc("eth_getBlockByNumber",new JSONArray().put("latest").put(false));
            Snapshot s=new Snapshot();s.block=head.getString("number");s.hash=head.getString("hash");s.blockMs=hex(head.getString("timestamp")).longValueExact()*1000;
            if(!s.hash.matches("0x[0-9a-fA-F]{64}"))throw new IOException("block hash");
            if(!address(call(pool,"0xc45a0155",s.block)).equals(FACTORY[network]))throw new IOException("Factory không khớp");
            s.token0=address(call(pool,"0x0dfe1681",s.block));s.token1=address(call(pool,"0xd21220a7",s.block));
            if(s.token0.equals(s.token1) || !address(call(FACTORY[network],"0xe6a43905"+arg(s.token0)+arg(s.token1),s.block)).equals(pool))throw new IOException("Factory không xác nhận pool");
            s.decimals0=word(call(s.token0,"0x313ce567",s.block),0).intValueExact();s.decimals1=word(call(s.token1,"0x313ce567",s.block),0).intValueExact();
            if(s.decimals0<0 || s.decimals0>36 || s.decimals1<0 || s.decimals1>36)throw new IOException("Decimals không hỗ trợ");
            String reserve=call(pool,"0x0902f1ac",s.block);s.r0=word(reserve,0);s.r1=word(reserve,1);s.reserveMs=word(reserve,2).longValueExact()*1000;
            if(s.r0.signum()<=0 || s.r1.signum()<=0 || s.r0.bitLength()>112 || s.r1.bitLength()>112)throw new IOException("Pool không có reserves hợp lệ");
            BigInteger bal0=word(call(s.token0,"0x70a08231"+arg(pool),s.block),0),bal1=word(call(s.token1,"0x70a08231"+arg(pool),s.block),0);
            s.balancesMatch=bal0.equals(s.r0) && bal1.equals(s.r1);
            s.gas=hex((String)rpc("eth_gasPrice",new JSONArray()));
            JSONObject confirmed=(JSONObject)rpc("eth_getBlockByNumber",new JSONArray().put(s.block).put(false));
            if(!s.hash.equals(confirmed.getString("hash")))throw new IOException("Block đã đổi / reorg");
            s.receivedNs=System.nanoTime();
            if(running){
                synchronized(spots){
                    if(s.blockMs>lastBlockMs){
                        if(s.blockMs-lastBlockMs>60000)spots.clear();
                        double liquidity=Math.sqrt(s.r0.doubleValue()*s.r1.doubleValue());
                        if(lastLiquidity>0 && liquidity<lastLiquidity*.8)liquidityDrop=true;
                        lastLiquidity=liquidity;lastBlockMs=s.blockMs;
                        spots.add(s.r1.doubleValue()/s.r0.doubleValue()*Math.pow(10,s.decimals0-s.decimals1));
                        while(spots.size()>6)spots.removeFirst();
                    }
                }
                latest=s;status="Đã xác minh chainId, factory và getPair ở cùng block";}
        }catch(Exception e){if(running){latest=null;status="CHẶN: RPC lỗi hoặc pool không vượt xác minh. Kiểm tra mạng / địa chỉ pool.";}}
    }
    String report(String amount,boolean reverse,int slip){
        Snapshot s=latest;long now=System.currentTimeMillis();
        if(!running || s==null)return status+"\nKhông có báo giá hiệu lực. Ký giao dịch: TẮT.";
        StringBuilder out=new StringBuilder(NAMES[network]+"\n"+status+"\nPool: "+pool+"\nBlock: "+hex(s.block)+"\nHash: "+s.hash+"\nGiờ block: "+Instant.ofEpochMilli(s.blockMs)+"\nTuổi block: "+Math.max(0,(now-s.blockMs)/1000)+" giây\nToken0: "+s.token0+"\nToken1: "+s.token1+"\nReserves token0: "+DexMath.decimal(s.r0,s.decimals0)+"\nReserves token1: "+DexMath.decimal(s.r1,s.decimals1)+"\nGas mạng tham chiếu: "+DexMath.decimal(s.gas,9)+" gwei (chưa tính phí giao dịch)\n");
        if(now-s.blockMs>60000 || s.blockMs>now+5000 || System.nanoTime()-s.receivedNs>60_000_000_000L)return out+"CHẶN: snapshot cũ / sai giờ";
        if(s.reserveMs>s.blockMs || s.blockMs-s.reserveMs>300000)return out+"CHẶN: reserves không cập nhật trong 5 phút hoặc timestamp sai";
        if(liquidityDrop)return out+"CHẶN: thanh khoản hình học giảm >20% giữa hai lần đọc; cần kiểm tra lại pool";
        if(!s.balancesMatch)return out+"CHẶN: số dư token khác reserves; cần kiểm tra token/pool";
        try{
            BigInteger ri=reverse?s.r1:s.r0,ro=reverse?s.r0:s.r1;int di=reverse?s.decimals1:s.decimals0,do_=reverse?s.decimals0:s.decimals1;
            BigInteger input=DexMath.units(amount,di),quote=DexMath.out(input,ri,ro,FEES[network]);
            if(input.multiply(BigInteger.valueOf(100)).compareTo(ri)>0 || quote.signum()<=0)return out+"CHẶN: lượng vào >1% reserves hoặc lượng ra bằng 0";
            double effective=input.doubleValue()*(10000-FEES[network])/10000.0;double impact=100.0*effective/(ri.doubleValue()+effective);
            out.append(reverse?"Token1 → Token0\n":"Token0 → Token1\n").append("Lượng vào: ").append(amount).append("\nRa dự kiến sau phí LP: ").append(DexMath.decimal(quote,do_)).append("\nSàn lượng ra theo slippage ").append(slip).append(" bps: ").append(DexMath.decimal(DexMath.minimum(quote,slip),do_)).append("\nTác động giá AMM (không gồm phí): ").append(String.format(Locale.US,"%.4f%%",impact)).append("\nPhí pool: ").append(FEES[network]).append(" bps\n");
        }catch(Exception e){return out+"CHẶN: khối lượng/slippage không hợp lệ (slippage 0–100 bps)";}
        synchronized(spots){if(spots.size()>=6){double change=100*(spots.getLast()/spots.getFirst()-1);out.append("Biến động spot qua 6 snapshot: ").append(String.format(Locale.US,"%.4f%%",change)).append(" (token1/token0; quan sát, không phải tín hiệu mua/bán)\n");}else out.append("Đang tích lũy 6 snapshot on-chain để đo biến động\n");}
        return out+"ƯỚC TÍNH ON-CHAIN, KHÔNG PHẢI CAM KẾT KHỚP. Chưa xác minh honeypot, thuế token, blacklist, quyền owner, MEV, gas tổng hoặc khả năng bán. Block head chưa final. Không ký / gửi giao dịch.";
    }
    synchronized void stop(){running=false;latest=null;status="ĐÃ DỪNG DEX";worker.shutdownNow();if(!cleanupStarted){cleanupStarted=true;NetworkCleanup.close(client,null);}}
}
