package com.bia.mobile;
import org.json.*;
import java.math.*;

/** Read-only reconciliation; never signs, submits or replaces an order. */
final class TradeReconcile {
    static BigInteger tokenSum(JSONArray a,String owner)throws Exception{BigInteger sum=BigInteger.ZERO;for(int i=0;i<a.length();i++){JSONObject t=a.getJSONObject(i);if(owner.equals(t.optString("owner")) && SolanaOrder.USDC.equals(t.getString("mint")))sum=sum.add(new BigInteger(t.getJSONObject("uiTokenAmount").getString("amount")));}return sum;}
    static String receipt(SolanaRpc rpc,String sig,String owner)throws Exception{
        Object value=rpc.call("getTransaction",new JSONArray().put(sig).put(SolanaRpc.object("encoding","json","commitment","finalized","maxSupportedTransactionVersion",0)));
        if(value==JSONObject.NULL)throw new IllegalStateException("Chưa có receipt finalized; giữ khóa lệnh");JSONObject t=(JSONObject)value,tx=t.getJSONObject("transaction"),meta=t.getJSONObject("meta");
        if(!sig.equals(tx.getJSONArray("signatures").getString(0)) || !owner.equals(tx.getJSONObject("message").getJSONArray("accountKeys").getString(0)))throw new IllegalStateException("Receipt không khớp chữ ký/ví");
        BigInteger pre=new BigInteger(meta.getJSONArray("preBalances").get(0).toString()),post=new BigInteger(meta.getJSONArray("postBalances").get(0).toString());
        BigInteger u=tokenSum(meta.getJSONArray("postTokenBalances"),owner).subtract(tokenSum(meta.getJSONArray("preTokenBalances"),owner));
        return SolanaRpc.object("owner",owner,"signature",sig,"slot",t.getLong("slot"),"feeLamports",meta.getLong("fee"),"preSol",pre.toString(),"solDelta",post.subtract(pre).toString(),"usdcDelta",u.toString(),"failed",!meta.isNull("err")).toString();
    }
    static void run(SolanaRpc rpc,TradeBook book)throws Exception{rpc.verifyMainnet();for(String[] p:book.pending()){String state=rpc.status(p[0]);String r=(state.equals("FINALIZED") || state.equals("FAILED"))?receipt(rpc,p[0],p[1]):"";if(!r.isEmpty() && new JSONObject(r).getBoolean("failed")!=state.equals("FAILED"))throw new IllegalStateException("Receipt/status không nhất quán");book.mark(p[0],state,r);}}
}
