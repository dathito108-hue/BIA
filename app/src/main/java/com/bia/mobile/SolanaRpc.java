package com.bia.mobile;

import okhttp3.*;
import org.json.*;
import java.util.concurrent.TimeUnit;
import java.math.BigInteger;

/** Real mainnet RPC. No private key and no sendTransaction method. */
final class SolanaRpc implements AutoCloseable {
    static final String URL="https://solana-rpc.publicnode.com";
    static final String GENESIS="5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";
    final OkHttpClient http=new OkHttpClient.Builder().connectTimeout(8,TimeUnit.SECONDS).readTimeout(15,TimeUnit.SECONDS).callTimeout(20,TimeUnit.SECONDS).followRedirects(false).followSslRedirects(false).retryOnConnectionFailure(false).build();
    volatile boolean closed;
    static JSONObject object(Object... values)throws JSONException{JSONObject o=new JSONObject();for(int i=0;i<values.length;i+=2)o.put((String)values[i],values[i+1]);return o;}
    JSONObject request(Request req)throws Exception {
        if(closed)throw new IllegalStateException("Đã dừng phiên");
        try(Response r=http.newCall(req).execute()){
            if(!r.isSuccessful())throw new IllegalStateException("Dịch vụ trả HTTP "+r.code());
            if(r.body()==null || r.body().contentLength()>524288)throw new IllegalStateException("Phản hồi quá lớn");
            okio.BufferedSource source=r.body().source();source.request(524289);if(source.buffer().size()>524288)throw new IllegalStateException("Phản hồi quá lớn");
            JSONObject o=new JSONObject(source.readUtf8());if(closed)throw new IllegalStateException("Đã dừng phiên");return o;
        }
    }
    Object call(String method,JSONArray params)throws Exception{
        if(!java.util.Arrays.asList("getAccountInfo","getMultipleAccounts","getTokenAccountsByOwner","getTransaction","getGenesisHash","getSlot","getBlockTime","getBalance","isBlockhashValid","getFeeForMessage","simulateTransaction","getSignatureStatuses").contains(method))throw new IllegalArgumentException("RPC chưa được cấp quyền");
        JSONObject body=object("jsonrpc","2.0","id",1,"method",method,"params",params);
        JSONObject r=request(new Request.Builder().url(URL).post(RequestBody.create(body.toString(),MediaType.get("application/json"))).build());
        if(r.has("error") || !r.has("result"))throw new IllegalStateException("RPC không trả kết quả hợp lệ");return r.get("result");
    }
    long verifyMainnet()throws Exception {
        if(!GENESIS.equals(call("getGenesisHash",new JSONArray())))throw new IllegalStateException("RPC không phải Solana mainnet");
        long slot=((Number)call("getSlot",new JSONArray().put(object("commitment","confirmed")))).longValue();
        Object time=call("getBlockTime",new JSONArray().put(slot));if(!(time instanceof Number))throw new IllegalStateException("Không có thời gian block");
        long age=System.currentTimeMillis()/1000-((Number)time).longValue();if(age< -5 || age>60)throw new IllegalStateException("Dữ liệu Solana quá cũ/lệch giờ");return slot;
    }
    BigInteger balance(String owner)throws Exception{SolanaWire.address(owner);JSONObject r=(JSONObject)call("getBalance",new JSONArray().put(owner).put(object("commitment","confirmed")));return new BigInteger(r.get("value").toString());}
    long preflight(SolanaWire.Transaction tx,long maxFee)throws Exception{
        JSONObject config=object("commitment","confirmed");
        JSONObject valid=(JSONObject)call("isBlockhashValid",new JSONArray().put(tx.blockhash).put(config));if(!valid.getBoolean("value"))throw new IllegalStateException("Blockhash hết hạn");
        String msg=android.util.Base64.encodeToString(tx.message,android.util.Base64.NO_WRAP);
        JSONObject fee=(JSONObject)call("getFeeForMessage",new JSONArray().put(msg).put(config));
        if(fee.isNull("value"))throw new IllegalStateException("Không tính được phí");long lamports=fee.getLong("value");if(lamports<0 || lamports>maxFee)throw new IllegalStateException("Phí vượt hạn mức");
        return lamports;
    }

    String status(String signature)throws Exception{
        if(signature==null || !signature.matches("[1-9A-HJ-NP-Za-km-z]{64,88}"))throw new IllegalArgumentException("Chữ ký không hợp lệ");
        JSONObject r=(JSONObject)call("getSignatureStatuses",new JSONArray().put(new JSONArray().put(signature)).put(object("searchTransactionHistory",true)));
        Object s=r.getJSONArray("value").get(0);if(s==JSONObject.NULL)return "UNKNOWN";JSONObject o=(JSONObject)s;if(!o.isNull("err"))return "finalized".equals(o.optString("confirmationStatus"))?"FAILED":"FAILED_PENDING";
        String c=o.optString("confirmationStatus");return "finalized".equals(c)?"FINALIZED":"confirmed".equals(c)?"CONFIRMED":"PENDING";
    }
    public void close(){if(closed)return;closed=true;NetworkCleanup.close(http,null);}
}
