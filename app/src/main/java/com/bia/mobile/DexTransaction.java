package com.bia.mobile;
import java.math.*;
import java.util.Locale;
import org.json.*;

/** Unsigned transaction draft only. No approval, wallet secrets or broadcast. */
final class DexTransaction {
    static String word(BigInteger n){if(n.signum()<0 || n.bitLength()>256)throw new IllegalArgumentException();return String.format(Locale.ROOT,"%064x",n);}
    static String calldata(BigInteger amount,BigInteger min,String tokenIn,String tokenOut,String recipient,long deadline){
        return "0x38ed1739"+word(amount)+word(min)+word(BigInteger.valueOf(160))+DexFeed.arg(recipient)+word(BigInteger.valueOf(deadline))+word(BigInteger.valueOf(2))+DexFeed.arg(tokenIn)+DexFeed.arg(tokenOut);
    }
    static String prepare(DexFeed f,String owner,String amount,boolean reverse,int slip)throws Exception {return CoreSkills.call("dex.prepare",()->prepareCore(f,owner,amount,reverse,slip));}
    static String prepareCore(DexFeed f,String owner,String amount,boolean reverse,int slip)throws Exception {
        if(!owner.matches("0x[0-9a-fA-F]{40}") || new BigInteger(owner.substring(2),16).signum()==0)throw new IllegalArgumentException("Cần địa chỉ ví công khai hợp lệ; không nhập private key");
        String report=f.report(amount,reverse,slip);
        if(!f.running || f.latest==null || report.contains("CHẶN") || report.contains("Không có báo giá"))throw new IllegalStateException("Báo giá chưa vượt điều kiện kiểm tra");
        DexFeed.Snapshot s=f.latest;long now=System.currentTimeMillis();
        if(now-s.blockMs>30000 || s.blockMs>now+5000 || s.reserveMs>s.blockMs || s.blockMs-s.reserveMs>300000 || System.nanoTime()-s.receivedNs>30_000_000_000L || f.liquidityDrop)throw new IllegalStateException("Cần snapshot mới trong 30 giây");
        String tokenIn=reverse?s.token1:s.token0,tokenOut=reverse?s.token0:s.token1,router=DexFeed.ROUTERS[f.network];
        BigInteger input=DexMath.units(amount,reverse?s.decimals1:s.decimals0),ri=reverse?s.r1:s.r0,ro=reverse?s.r0:s.r1;
        // Repeat bounds on the captured snapshot so concurrent refresh cannot change draft limits.
        if(!s.balancesMatch || input.multiply(BigInteger.valueOf(100)).compareTo(ri)>0)throw new IllegalStateException("Snapshot không đủ điều kiện");
        BigInteger quote=DexMath.out(input,ri,ro,DexFeed.FEES[f.network]),minimum=DexMath.minimum(quote,slip);
        if(minimum.signum()<=0)throw new IllegalStateException("Lượng ra tối thiểu bằng 0");
        BigInteger balance=DexFeed.word(f.call(tokenIn,"0x70a08231"+DexFeed.arg(owner),"latest"),0);
        BigInteger allowance=DexFeed.word(f.call(tokenIn,"0xdd62ed3e"+DexFeed.arg(owner)+DexFeed.arg(router),"latest"),0);
        if(balance.compareTo(input)<0)throw new IllegalStateException("Ví không đủ token đầu vào");
        if(allowance.compareTo(input)<0)throw new IllegalStateException("Allowance chưa đủ. BIA không tự approve; kiểm tra quyền trong ví");
        long deadline=System.currentTimeMillis()/1000+120;
        String data=calldata(input,minimum,tokenIn,tokenOut,owner,deadline);
        JSONObject tx=new JSONObject().put("from",owner.toLowerCase(Locale.ROOT)).put("to",router).put("data",data).put("value","0x0");
        // Read-only EVM preflight is not a paper-trading environment and does not send a transaction.
        Object checked=f.rpc("eth_call",new JSONArray().put(tx).put("latest"));
        BigInteger actual=DexFeed.word((String)checked,3); // dynamic uint[]: offset,length,input,output
        if(actual.compareTo(minimum)<0)throw new IllegalStateException("Preflight không đạt minOut");
        BigInteger gas=DexFeed.hex((String)f.rpc("eth_estimateGas",new JSONArray().put(tx)));
        if(!f.running || f.liquidityDrop || System.currentTimeMillis()-s.blockMs>30000 || System.currentTimeMillis()/1000>=deadline-30)throw new IllegalStateException("Phiên dừng hoặc báo giá hết hạn");
        tx.put("chainId","0x"+Integer.toHexString(DexFeed.CHAINS[f.network])).put("gas","0x"+gas.multiply(BigInteger.valueOf(120)).divide(BigInteger.valueOf(100)).toString(16));
        return new JSONObject().put("schema","BIA_DEX_UNSIGNED_V135").put("broadcast",false).put("deadline_utc_s",deadline).put("pool",f.pool).put("quoted_block_hash",s.hash).put("token_in",tokenIn).put("token_out",tokenOut).put("amount_in_raw",input.toString()).put("minimum_out_raw",minimum.toString()).put("transaction",tx).put("notice","CHƯA KÝ/CHƯA GỬI. Preflight không bảo đảm khớp hoặc token an toàn. Ví phải kiểm tra chain/router/token/amount/minOut/deadline/gas và xác nhận lại.").toString(2);
    }
}
