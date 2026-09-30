package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class ConversationContinuityV152Test extends InstrumentationTestCase {
 protected void setUp(){MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testPreviousTurnAndOrdinalReference(){
  say("Nguồn a: mưa gây ra đường ướt.");
  String first=say("Mưa có gây ra đường ướt không?");
  assertTrue(first,first.contains("ủng hộ"));
  String repeat=say("Ý vừa rồi");
  assertTrue(repeat,repeat.contains("ủng hộ"));
  String second=say("Cái thứ hai");
  assertTrue(second,second.contains("duong uot"));
  String ambiguous=say("Cái thứ hai thì sao?");
  assertTrue(ambiguous,ambiguous.contains("nguyên nhân hay kết quả"));
 }
}
