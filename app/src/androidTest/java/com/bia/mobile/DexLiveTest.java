package com.bia.mobile;
import android.test.InstrumentationTestCase;
import java.math.*;

public final class DexLiveTest extends InstrumentationTestCase {
    public void testExactQuoteAndUnsignedCalldata()throws Exception {
        BigInteger q=DexMath.out(BigInteger.valueOf(1000),BigInteger.valueOf(1000000),BigInteger.valueOf(1000000),30);
        assertEquals(BigInteger.valueOf(996),q);assertEquals(BigInteger.valueOf(991),DexMath.minimum(q,50));
        assertEquals(new BigInteger("1000000000000000001"),DexMath.units("1.000000000000000001",18));
        boolean rejected=false;try{DexMath.units("1.0000001",6);}catch(ArithmeticException e){rejected=true;}assertTrue(rejected);
        rejected=false;try{DexMath.minimum(q,101);}catch(IllegalArgumentException e){rejected=true;}assertTrue(rejected);
        String a="0x1111111111111111111111111111111111111111",b="0x2222222222222222222222222222222222222222";
        String data=DexTransaction.calldata(BigInteger.valueOf(1000),q,a,b,a,1800000120L);
        assertEquals(10+8*64,data.length());assertTrue(data.startsWith("0x38ed1739"));
        assertEquals(BigInteger.valueOf(160),new BigInteger(data.substring(10+2*64,10+3*64),16));
        assertEquals(BigInteger.valueOf(2),new BigInteger(data.substring(10+5*64,10+6*64),16));
    }
    public void testRealEthereumAndBnbPools()throws Exception {
        StringBuilder evidence=new StringBuilder();
        for(int network=0;network<2;network++){
            String pool="0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
            if(network==1){DexFeed lookup=new DexFeed(1,pool);lookup.running=true;try{pool=DexFeed.address(lookup.call(DexFeed.FACTORY[1],"0xe6a43905"+DexFeed.arg("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")+DexFeed.arg("0x55d398326f99059ff775485246999027b3197955"),"latest"));}finally{lookup.stop();}}
            DexFeed f=new DexFeed(network,pool);f.running=true;
            try{f.refresh();assertNotNull("Real chain/pool checks required: "+f.status,f.latest);DexFeed.Snapshot s=f.latest;assertTrue(s.blockMs>0);assertTrue(s.r0.signum()>0);assertTrue(s.r1.signum()>0);
                evidence.append("BIA_DEX_LIVE_EVIDENCE chain="+DexFeed.CHAINS[network]+" pool="+pool+" block="+s.block+" hash="+s.hash+" token0="+s.token0+" token1="+s.token1+" synthetic=false signed=false\n");
                BigInteger input=BigInteger.TEN.pow(s.decimals0);
                String query="0xd06ca61f"+DexTransaction.word(input)+DexTransaction.word(BigInteger.valueOf(64))+DexTransaction.word(BigInteger.valueOf(2))+DexFeed.arg(s.token0)+DexFeed.arg(s.token1);
                BigInteger routerQuote=DexFeed.word(f.call(DexFeed.ROUTERS[network],query,s.block),3);
                assertEquals("Exact AMM math must equal actual router at same block",routerQuote,DexMath.out(input,s.r0,s.r1,DexFeed.FEES[network]));
                evidence.append("router_quote_verified=true rawOut="+routerQuote+"\n");
                boolean blocked=false;try{DexTransaction.prepare(f,"not-a-wallet","1",false,50);}catch(IllegalArgumentException e){blocked=true;}assertTrue(blocked);
                getInstrumentation().runOnMainSync(f::stop);assertNull(f.latest);assertFalse(f.running);
            }finally{f.stop();}
        }
        try(java.io.FileOutputStream out=new java.io.FileOutputStream(new java.io.File(getInstrumentation().getTargetContext().getExternalFilesDir(null),"dex-evidence.txt"))){out.write(evidence.toString().getBytes(java.nio.charset.StandardCharsets.UTF_8));}
    }
}
