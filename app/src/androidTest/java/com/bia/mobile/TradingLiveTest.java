package com.bia.mobile;
import android.content.Intent;
import android.test.InstrumentationTestCase;
import android.os.SystemClock;
import org.json.*;

public final class TradingLiveTest extends InstrumentationTestCase {
    public void testRealPublicMarketStreamAndStop()throws Exception {
        TradingActivity a=(TradingActivity)getInstrumentation().startActivitySync(new Intent(getInstrumentation().getTargetContext(),TradingActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK|Intent.FLAG_ACTIVITY_CLEAR_TOP));
        getInstrumentation().runOnMainSync(()->{a.symbols.setText("BTCUSDT");a.connect.performClick();});
        MarketFeed f=a.feed;assertNotNull(f);
        long until=SystemClock.elapsedRealtime()+60000;boolean ready=false;
        while(SystemClock.elapsedRealtime()<until){
            synchronized(f){MarketFeed.Instrument i=f.instruments.get("BTCUSDT");ready=f.connected && i.eventMs>0 && i.candles.size()>=21 && System.currentTimeMillis()-i.eventMs<=15000;if(ready)break;}
            Thread.sleep(200);
        }
        try {
            assertTrue("Actual public WebSocket + actual REST candles required: "+f.snapshot(),ready);
            synchronized(f){MarketFeed.Instrument i=f.instruments.get("BTCUSDT");String evidence="BIA_LIVE_EVIDENCE source=Binance symbol=BTCUSDT eventMs="+i.eventMs+" price="+i.price+" candles="+i.candles.size()+" synthetic=false orders=disabled";System.out.println(evidence);try(java.io.FileOutputStream out=new java.io.FileOutputStream(new java.io.File(getInstrumentation().getTargetContext().getExternalFilesDir(null),"market-evidence.txt"))){out.write(evidence.getBytes(java.nio.charset.StandardCharsets.UTF_8));}}
            assertFalse("Live JNI must leave warmup state",f.snapshot().contains("đang chờ nến từ nguồn"));
            getInstrumentation().runOnMainSync(()->a.stop.performClick());
            assertFalse(f.running);assertFalse(f.connected);assertNull(a.feed);
            getInstrumentation().runOnMainSync(()->a.finish());
        }finally{f.stop();}
    }
    public void testDecodersRejectReorderingAndFutureCandles()throws Exception {
        // Isolated protocol fixtures only. Production has no fixture/demo fallback.
        MarketFeed f=new MarketFeed(false,"BTCUSDT","",()->{});f.running=true;
        long now=1800000000000L;
        f.accept("{\"e\":\"aggTrade\",\"s\":\"BTCUSDT\",\"p\":\"100\",\"T\":"+now+"}",now,1);
        f.accept("{\"e\":\"aggTrade\",\"s\":\"BTCUSDT\",\"p\":\"90\",\"T\":"+(now-1)+"}",now,2);
        assertEquals(100.0,f.instruments.get("BTCUSDT").price,0.0);
        JSONArray rows=new JSONArray();rows.put(new JSONArray("[0,100,101,99,100,1,"+(now+60000)+"]"));
        f.loadSeed("BTCUSDT",rows.toString(),now);assertEquals(0,f.instruments.get("BTCUSDT").candles.size());f.stop();
        MarketFeed td=new MarketFeed(true,"EUR/USD,AAPL","fixturekey",()->{});td.running=true;td.lastRefreshMinute=now/60000;
        td.accept("{\"event\":\"price\",\"symbol\":\"EUR/USD\",\"price\":1.1,\"timestamp\":1800000000}",now,1);
        assertEquals(1.1,td.instruments.get("EUR/USD").price,0.0);assertEquals(now,td.instruments.get("EUR/USD").eventMs);
        td.stop();assertFalse(td.connected);
    }
}
