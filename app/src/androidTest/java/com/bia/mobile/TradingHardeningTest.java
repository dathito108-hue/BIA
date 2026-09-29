package com.bia.mobile;
import android.test.InstrumentationTestCase;
import android.os.SystemClock;
import android.app.job.*;
import org.json.*;
import java.math.*;
import java.io.*;
import java.util.*;
import org.bouncycastle.math.ec.rfc8032.Ed25519;

public final class TradingHardeningTest extends InstrumentationTestCase {
    TradeBook book;String dbName;String owner;byte[] seed;
    @Override protected void setUp()throws Exception{super.setUp();dbName="v137-test-"+System.nanoTime()+".db";book=new TradeBook(getInstrumentation().getTargetContext(),dbName);seed=new byte[32];Arrays.fill(seed,(byte)5);byte[] pub=new byte[32];Ed25519.generatePublicKey(seed,0,pub,0);owner=SolanaWire.base58(pub);}
    @Override protected void tearDown()throws Exception{book.close();getInstrumentation().getTargetContext().deleteDatabase(dbName);super.tearDown();}
    interface Action{void run()throws Exception;}
    void rejected(Action action)throws Exception{try{action.run();fail("Expected fail-closed rejection");}catch(IllegalStateException|IllegalArgumentException expected){}}
    byte[] wire(){byte[] tx=new byte[135];tx[0]=1;tx[65]=1;tx[68]=1;System.arraycopy(SolanaWire.address(owner),0,tx,69,32);return tx;}
    SolanaOrder quote(String id,boolean sell)throws Exception{JSONObject q=SolanaRpc.object("inputMint",sell?SolanaOrder.SOL:SolanaOrder.USDC,"outputMint",sell?SolanaOrder.USDC:SolanaOrder.SOL,"inAmount",sell?"1000000":"1000000","outAmount",sell?"100000":"10000000","otherAmountThreshold",sell?"99500":"9950000","swapMode","ExactIn","slippageBps",50,"router","metis","priceImpactPct","0.001","signatureFeeLamports",5000,"prioritizationFeeLamports",0,"rentFeeLamports",0,"feeBps",2,"requestId",id,"taker",owner,"transaction",android.util.Base64.encodeToString(wire(),android.util.Base64.NO_WRAP));return new SolanaOrder(q,owner,sell,BigInteger.valueOf(1000000),50,5000000,SystemClock.elapsedRealtime());}
    SolanaPortfolio portfolio(String price){return new SolanaPortfolio(BigInteger.valueOf(1000000000),BigInteger.valueOf(100000000),new BigDecimal(price),SystemClock.elapsedRealtime());}
    void configure(SolanaPortfolio p)throws Exception{book.configure(owner,1000000,2000000,10000000,500,10000,5,p);}
    String signature(){byte[] b=new byte[64];Arrays.fill(b,(byte)7);return SolanaWire.base58(b);}
    static void integer(ByteArrayOutputStream out,long n,int size){for(int i=0;i<size;i++)out.write((byte)(n>>(8*i)));}
    public void testCryptographicSignatureAndRouteBytes()throws Exception{
        byte[] tx=wire();SolanaWire.Transaction t=new SolanaWire.Transaction(tx,owner);Ed25519.sign(seed,0,t.message,0,t.message.length,tx,1);assertNotNull(t.signed(tx,owner));tx[20]^=1;rejected(()->t.signed(tx,owner));
        SolanaOrder q=quote("route-guard",true);ByteArrayOutputStream out=new ByteArrayOutputStream();out.write(Arrays.copyOf(SolanaAudit.hash("global:route".getBytes("UTF-8")),8));integer(out,1,4);out.write(7);out.write(100);out.write(0);out.write(1);integer(out,1000000,8);integer(out,100000,8);integer(out,50,2);out.write(2);byte[] route=out.toByteArray();SolanaAudit.routeArgs(route,q,false);byte[] changed=route.clone();changed[16]=0;rejected(()->SolanaAudit.routeArgs(changed,q,false));byte[] appended=Arrays.copyOf(route,route.length+1);rejected(()->SolanaAudit.routeArgs(appended,q,false));
        byte[] unknown=route.clone();unknown[12]=(byte)255;rejected(()->SolanaAudit.routeArgs(unknown,q,false));
        byte[] oversized={(byte)128,0};rejected(()->new SolanaAudit.Reader(oversized).compact());
        String source=SolanaAudit.ata(owner,SolanaOrder.SOL);assertEquals(32,SolanaWire.address(source).length);assertFalse(SolanaAudit.onCurve(SolanaWire.address(source)));assertFalse(source.equals(SolanaAudit.ata(owner,SolanaOrder.USDC)));
    }
    public void testReservationsSurviveProcessRestartWithoutReplay()throws Exception{
        SolanaPortfolio p=portfolio("100");configure(p);SolanaOrder q=quote("a",true);String id=book.reserve(q,p);assertTrue(book.hasOpen());rejected(()->book.reserve(quote("duplicate",true),p));book.signed(id,signature());assertTrue(book.pending().isEmpty());book.recheck(id,q,p);book.close();book=new TradeBook(getInstrumentation().getTargetContext(),dbName);book.recover();assertFalse(book.hasOpen());
        SolanaOrder q2=quote("b",true);String id2=book.reserve(q2,p);byte[] bytes=new byte[64];Arrays.fill(bytes,(byte)9);book.signed(id2,SolanaWire.base58(bytes));book.dispatch(id2);rejected(()->book.dispatch(id2));book.close();book=new TradeBook(getInstrumentation().getTargetContext(),dbName);book.recover();assertTrue(book.hasOpen());book.cancel(id2);assertTrue(book.hasOpen());assertTrue(book.export().contains("UNKNOWN"));rejected(()->book.reserve(quote("c",true),p));
    }
    public void testRiskCapsDrawdownExposureAndExternalChanges()throws Exception{
        SolanaPortfolio p=portfolio("100");SolanaOrder q=quote("risk",true);rejected(()->book.check(owner,q,p));
        book.configure(owner,50000,2000000,10000000,500,10000,5,p);rejected(()->book.check(owner,q,p));
        book.configure(owner,2000000,2000000,10000000,500,5000,5,p);rejected(()->book.check(owner,quote("buy",false),p));
        configure(p);rejected(()->book.check(owner,q,portfolio("80")));rejected(()->book.check(owner,q,p));
        configure(p);SolanaPortfolio altered=new SolanaPortfolio(p.sol.add(BigInteger.ONE),p.usdc,p.price,SystemClock.elapsedRealtime());rejected(()->book.check(owner,q,altered));configure(p);book.halt("test emergency");rejected(()->book.check(owner,q,p));
    }
    public void testFinalizedReceiptIsAppliedOnceAndFailedReceiptStaysLocked()throws Exception{
        SolanaPortfolio p=portfolio("100");configure(p);String id=book.reserve(quote("receipt",true),p),sig=signature();book.signed(id,sig);book.dispatch(id);book.mark(sig,"CONFIRMED","");assertTrue(book.hasOpen());rejected(()->book.mark(sig,"FINALIZED",""));
        String receipt=SolanaRpc.object("owner",owner,"signature",sig,"preSol",p.sol.toString(),"solDelta","-1005000","usdcDelta","100000","failed",false).toString();book.mark(sig,"FINALIZED",receipt);assertFalse(book.hasOpen());String first=book.export();book.mark(sig,"FINALIZED",receipt);JSONObject before=new JSONObject(first),after=new JSONObject(book.export());assertEquals(before.getJSONArray("policy").toString(),after.getJSONArray("policy").toString());
        assertEquals("998995000",after.getJSONArray("policy").getJSONObject(0).getString("sol"));assertEquals("100100000",after.getJSONArray("policy").getJSONObject(0).getString("usdc"));
    }
    public void testRecoveryJobAndNativeQualityBoundaries()throws Exception{
        TradeRecoveryService.schedule(getInstrumentation().getTargetContext());JobScheduler s=getInstrumentation().getTargetContext().getSystemService(JobScheduler.class);JobInfo j=s.getPendingJob(137);assertNotNull(j);assertTrue(j.isPersisted());assertEquals("com.bia.mobile.TradeRecoveryService",j.getService().getClassName());s.cancel(137);
        long now=System.currentTimeMillis();double[] points=new double[60];for(int i=0;i<30;i++){points[i*2]=now-(29-i)*15000;points[i*2+1]=100*Math.pow(1.001,i);}String text=TradingNative.quality(points,now,10);assertTrue(text.contains("không phải khớp lệnh/P&L"));assertTrue(TradingNative.quality(points,now+60000,10).contains("CHƯA ĐỦ"));
    }
}
