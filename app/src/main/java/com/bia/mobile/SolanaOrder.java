package com.bia.mobile;

import org.json.*;
import okhttp3.*;
import java.math.*;
import android.os.SystemClock;
import android.util.Base64;

/** Jupiter is a trusted routing/building service, not a BIA intelligence backend. */
final class SolanaOrder {
    static final String SOL="So11111111111111111111111111111111111111112";
    static final String USDC="EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    final String owner,input,output,requestId;
    final BigInteger inAmount,outAmount,minOut;
    final int slippage;final long feeBudget,receivedAt,networkFee,rentFee,platformBps;
    final double impact;
    final SolanaWire.Transaction transaction;
    SolanaOrder(JSONObject q,String owner,boolean sellSol,BigInteger amount,int bps,long maxFee,long began)throws Exception{
        this.owner=owner;input=sellSol?SOL:USDC;output=sellSol?USDC:SOL;inAmount=amount;slippage=bps;feeBudget=maxFee;receivedAt=began;
        if(bps<0 || bps>100 || maxFee<=0 || maxFee>10000000)throw new IllegalArgumentException("Slippage 0–100 bps; ngân sách phí tối đa 0.01 SOL");
        if(!input.equals(q.getString("inputMint")) || !output.equals(q.getString("outputMint")) || !amount.equals(new BigInteger(q.getString("inAmount"))) || !"ExactIn".equals(q.getString("swapMode")) || bps!=q.getInt("slippageBps"))throw new IllegalStateException("Báo giá không khớp yêu cầu");
        if(!"metis".equals(q.getString("router")) || q.has("error") || q.has("errorCode"))throw new IllegalStateException("Router/lệnh chưa hỗ trợ");
        outAmount=new BigInteger(q.getString("outAmount"));minOut=new BigInteger(q.getString("otherAmountThreshold"));
        BigInteger floor=outAmount.multiply(BigInteger.valueOf(10000-bps)).divide(BigInteger.valueOf(10000));
        if(outAmount.signum()<=0 || outAmount.bitLength()>63 || minOut.signum()<=0 || minOut.compareTo(floor)<0 || minOut.compareTo(outAmount)>0)throw new IllegalStateException("Mức nhận tối thiểu không hợp lệ");
        impact=Double.parseDouble(q.getString("priceImpactPct"));if(!Double.isFinite(impact) || Math.abs(impact)>0.01)throw new IllegalStateException("Price impact vượt 1%");
        networkFee=Math.addExact(nonnegative(q,"signatureFeeLamports"),nonnegative(q,"prioritizationFeeLamports"));rentFee=nonnegative(q,"rentFeeLamports");
        if(Math.addExact(networkFee,rentFee)>maxFee)throw new IllegalStateException("Phí mạng + rent vượt hạn mức");
        platformBps=nonnegative(q,"feeBps");if(platformBps>100)throw new IllegalStateException("Phí nền tảng vượt 1%");
        requestId=q.getString("requestId");if(requestId.length()>128 || requestId.isEmpty())throw new IllegalStateException("Thiếu định danh lệnh");
        if(owner.isEmpty())transaction=null;
        else {
            SolanaWire.address(owner);if(!owner.equals(q.getString("taker")))throw new IllegalStateException("Ví nhận báo giá không khớp");
            if(q.optBoolean("gasless",false))throw new IllegalStateException("Chưa hỗ trợ giao dịch tài trợ phí");
            String payload=q.optString("transaction","");if(payload.length()>1800 || payload.isEmpty())throw new IllegalStateException("Jupiter chưa tạo được giao dịch; kiểm tra số dư");
            transaction=new SolanaWire.Transaction(Base64.decode(payload,Base64.DEFAULT),owner);transaction.unsigned();
        }
        fresh();
    }
    static long nonnegative(JSONObject o,String key)throws Exception{long n=o.getLong(key);if(n<0)throw new IllegalStateException("Phí âm");return n;}
    void fresh(){long age=SystemClock.elapsedRealtime()-receivedAt;if(age<0 || age>60000)throw new IllegalStateException("Báo giá quá 60 giây; cần lấy lại");}
    static SolanaOrder fetch(SolanaRpc rpc,String apiKey,String owner,boolean sellSol,String quantity,int bps,long maxFee)throws Exception{
        BigInteger n=SolanaWire.units(quantity,sellSol?9:6);long began=SystemClock.elapsedRealtime();
        HttpUrl.Builder url=HttpUrl.get("https://api.jup.ag/swap/v2/order").newBuilder().addQueryParameter("inputMint",sellSol?SOL:USDC).addQueryParameter("outputMint",sellSol?USDC:SOL).addQueryParameter("amount",n.toString()).addQueryParameter("slippageBps",Integer.toString(bps)).addQueryParameter("excludeRouters","jupiterz,dflow,okx");
        if(!owner.isEmpty()){SolanaWire.address(owner);url.addQueryParameter("taker",owner);}
        Request.Builder req=new Request.Builder().url(url.build());if(!apiKey.isEmpty())req.header("x-api-key",apiKey);
        return new SolanaOrder(rpc.request(req.build()),owner,sellSol,n,bps,maxFee,began);
    }
    long preflight(SolanaRpc rpc)throws Exception{fresh();if(transaction==null)throw new IllegalStateException("Chưa liên kết ví");rpc.verifyMainnet();long fee=rpc.preflight(transaction,feeBudget-rentFee);BigInteger need=BigInteger.valueOf(Math.addExact(fee,rentFee));if(input.equals(SOL))need=need.add(inAmount);if(rpc.balance(owner).compareTo(need)<0)throw new IllegalStateException("Thiếu SOL đầu vào/phí/rent");fresh();return fee;}
    JSONObject execute(SolanaRpc rpc,String apiKey,SolanaWire.Transaction signed)throws Exception{
        fresh();if(!java.util.Arrays.equals(transaction.message,signed.message))throw new IllegalStateException("Nội dung ký không khớp");
        JSONObject body=SolanaRpc.object("signedTransaction",Base64.encodeToString(signed.bytes,Base64.NO_WRAP),"requestId",requestId);
        Request.Builder r=new Request.Builder().url("https://api.jup.ag/swap/v2/execute").post(RequestBody.create(body.toString(),MediaType.get("application/json")));if(!apiKey.isEmpty())r.header("x-api-key",apiKey);
        return rpc.request(r.build()); // exactly one submission attempt; reconcile by signature after any uncertainty
    }
    String report(){int a=input.equals(SOL)?9:6,b=output.equals(SOL)?9:6;return "MAINNET • Jupiter/Metis\n"+SolanaWire.display(inAmount,a)+(a==9?" SOL":" USDC")+" → "+SolanaWire.display(outAmount,b)+(b==9?" SOL":" USDC")+"\nNhận tối thiểu: "+SolanaWire.display(minOut,b)+"\nSlippage: "+slippage+" bps • Impact: "+String.format(java.util.Locale.US,"%.4f%%",impact*100)+"\nPhí mạng dự kiến: "+SolanaWire.display(BigInteger.valueOf(networkFee),9)+" SOL; rent: "+SolanaWire.display(BigInteger.valueOf(rentFee),9)+" SOL\nPhí nền tảng: "+platformBps+" bps (đã nằm trong báo giá)\nTuổi yêu cầu: "+((SystemClock.elapsedRealtime()-receivedAt)/1000)+" giây / tối đa 60\n"+(transaction==null?"Báo giá đọc-only, chưa gắn ví.":"Chưa ký. Cần kiểm tra nội dung tại ví; preflight không chứng minh mọi chỉ thị an toàn.");}
}
