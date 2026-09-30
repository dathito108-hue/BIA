package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class NaturalDialogueV147Test extends InstrumentationTestCase {
 protected void setUp(){MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testNaturalCausalDialoguePreservesMeaning(){
  say("Ghi nhớ rằng mưa gây ra đường ướt.");
  String direct=say("BIA ơi, cho mình hỏi, mưa có phải là nguyên nhân của đường ướt không nhỉ?");
  assertTrue(direct,direct.contains("ủng hộ"));
  String inverse=say("Theo bạn, đường ướt có phải là do mưa không?");
  assertTrue(inverse,inverse.contains("ủng hộ"));
  say("Gió có gây ra sóng không?");
  String ambiguous=say("Thế còn bão thì sao?");
  assertTrue(ambiguous,ambiguous.contains("nguyên nhân hay kết quả"));
 }
}
