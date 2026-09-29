package com.bia.mobile;

import android.test.InstrumentationTestCase;
import android.content.Intent;
import android.os.SystemClock;
import org.json.*;
import java.math.*;
import java.util.*;
import java.io.*;

public final class SolanaLiveTest extends InstrumentationTestCase {
    public void testWireAndQuoteRejectChangedIntent()throws Exception{
        String owner=SolanaOrder.USDC;assertEquals(owner,SolanaWire.base58(SolanaWire.address(owner)));
        assertEquals("11111111111111111111111111111111",SolanaWire.base58(SolanaWire.address("11111111111111111111111111111111")));
        byte[] tx=new byte[135];tx[0]=1;tx[65]=1;tx[68]=1;System.arraycopy(SolanaWire.address(owner),0,tx,69,32);tx[134]=0;
        SolanaWire.Transaction t=new SolanaWire.Transaction(tx,owner);t.unsigned();
        byte[] signed=tx.clone();signed[1]=7;try{t.signed(signed,owner);fail("Invalid signature accepted");}catch(IllegalArgumentException expected){}
        signed[100]^=1;try{t.signed(signed,owner);fail("Changed message accepted");}catch(IllegalArgumentException expected){}
        try{new SolanaWire.Transaction(tx,SolanaOrder.SOL);fail("Changed payer accepted");}catch(IllegalArgumentException expected){}
        try{SolanaWire.units("0.0000001",6);fail("Rounded input accepted");}catch(IllegalArgumentException expected){}
        JSONObject q=new JSONObject("{\"inputMint\":\""+SolanaOrder.SOL+"\",\"outputMint\":\""+SolanaOrder.USDC+"\",\"inAmount\":\"1000000\",\"outAmount\":\"100000\",\"otherAmountThreshold\":\"99500\",\"swapMode\":\"ExactIn\",\"slippageBps\":50,\"router\":\"metis\",\"priceImpactPct\":\"0.001\",\"signatureFeeLamports\":0,\"prioritizationFeeLamports\":0,\"rentFeeLamports\":0,\"feeBps\":2,\"requestId\":\"unit-fixture-only\"}");
        BigInteger n=BigInteger.valueOf(1000000);assertNotNull(new SolanaOrder(q,"",true,n,50,5000000,SystemClock.elapsedRealtime()));
        q.put("otherAmountThreshold","1");try{new SolanaOrder(q,"",true,n,50,5000000,SystemClock.elapsedRealtime());fail("Weak minOut accepted");}catch(IllegalStateException expected){}
        q.put("otherAmountThreshold","99500");q.put("priceImpactPct","NaN");try{new SolanaOrder(q,"",true,n,50,5000000,SystemClock.elapsedRealtime());fail("NaN accepted");}catch(IllegalStateException expected){}
        q.put("priceImpactPct","0.001");try{new SolanaOrder(q,"",true,n,50,5000000,SystemClock.elapsedRealtime()-61000);fail("Stale quote accepted");}catch(IllegalStateException expected){}
    }
    public void testRealSolanaMainnetAndJupiterQuote()throws Exception{
        SolanaRpc rpc=new SolanaRpc();try{long slot=rpc.verifyMainnet();assertTrue(slot>0);
            SolanaOrder q=SolanaOrder.fetch(rpc,"","",true,"0.001",50,5000000);assertNull(q.transaction);assertTrue(q.minOut.signum()>0);
            String evidence="BIA_SOLANA_MAINNET slot="+slot+" genesis="+SolanaRpc.GENESIS+"\nJupiter request="+q.requestId+" input="+q.input+" output="+q.output+" inRaw="+q.inAmount+" outRaw="+q.outAmount+" minRaw="+q.minOut+" synthetic=false signed=false broadcast=false\n";
            try(FileOutputStream f=new FileOutputStream(new File(getInstrumentation().getTargetContext().getExternalFilesDir(null),"solana-evidence.txt"))){f.write(evidence.getBytes(java.nio.charset.StandardCharsets.UTF_8));}
            try{rpc.call("sendTransaction",new JSONArray());fail("Unapproved RPC write accepted");}catch(IllegalArgumentException expected){}
        }finally{getInstrumentation().runOnMainSync(rpc::close);}
    }
    public void testSolanaUiNoWalletDoesNotSubmit()throws Exception{
        Intent i=new Intent(getInstrumentation().getTargetContext(),SolanaActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK|Intent.FLAG_ACTIVITY_CLEAR_TOP);
        SolanaActivity a=(SolanaActivity)getInstrumentation().startActivitySync(i);
        getInstrumentation().runOnMainSync(()->{assertFalse(a.sign.isEnabled());a.connect.performClick();});
        getInstrumentation().waitForIdleSync();
        getInstrumentation().runOnMainSync(()->{assertTrue(a.wallet.owner().isEmpty());assertFalse(a.sign.isEnabled());assertTrue(a.pending.isEmpty());a.stop();a.finish();});
    }
}
