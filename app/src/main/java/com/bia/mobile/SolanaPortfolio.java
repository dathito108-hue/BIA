package com.bia.mobile;
import java.math.*;
import org.json.*;
import android.os.SystemClock;

/** Only the SOL/USDC sleeve, denominated in USDC. No invented cost basis or USD peg. */
final class SolanaPortfolio {
    final BigInteger sol,usdc;final BigDecimal price;final long at;
    SolanaPortfolio(BigInteger sol,BigInteger usdc,BigDecimal price,long at){if(sol.signum()<0 || usdc.signum()<0 || price.signum()<=0)throw new IllegalArgumentException("Danh mục không hợp lệ");this.sol=sol;this.usdc=usdc;this.price=price;this.at=at;}
    void fresh(){long age=SystemClock.elapsedRealtime()-at;if(age<0 || age>30000)throw new IllegalStateException("Định giá danh mục quá 30 giây");}
    long value(BigInteger lamports){return new BigDecimal(lamports).multiply(price).movePointLeft(3).setScale(0,RoundingMode.CEILING).longValueExact();}
    long equity(){return Math.addExact(value(sol),usdc.longValueExact());}
    static SolanaPortfolio read(SolanaRpc rpc,String owner,String apiKey)throws Exception{
        long started=SystemClock.elapsedRealtime();SolanaWire.address(owner);rpc.verifyMainnet();SolanaOrder q=SolanaOrder.fetch(rpc,apiKey,"",true,"0.001",50,10000000);
        BigDecimal price=new BigDecimal(q.outAmount).movePointLeft(6).divide(new BigDecimal("0.001"));
        BigInteger sol=rpc.balance(owner),usdc=BigInteger.ZERO;
        JSONObject result=(JSONObject)rpc.call("getTokenAccountsByOwner",new JSONArray().put(owner).put(SolanaRpc.object("mint",SolanaOrder.USDC)).put(SolanaRpc.object("encoding","base64","commitment","confirmed")));
        JSONArray accounts=result.getJSONArray("value");if(accounts.length()>100)throw new IllegalStateException("Quá nhiều tài khoản token");
        for(int i=0;i<accounts.length();i++){JSONObject a=accounts.getJSONObject(i).getJSONObject("account");SolanaAudit.token(a,owner,SolanaOrder.USDC,false);usdc=usdc.add(SolanaAudit.le(SolanaAudit.data(a),64,8));}
        SolanaPortfolio p=new SolanaPortfolio(sol,usdc,price,started);p.fresh();return p;
    }
    String report(){return "Danh mục SOL/USDC: "+SolanaWire.display(sol,9)+" SOL + "+SolanaWire.display(usdc,6)+" USDC\nGiá trị tham chiếu: "+SolanaWire.display(BigInteger.valueOf(equity()),6)+" USDC. Không phải P&L đã chốt; không bao gồm tài sản khác.";}
}
