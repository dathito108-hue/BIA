package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class CorrectionEllipsisV153Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testCorrectionAndEllipsis(){
  say("Nguồn a: mưa gây ra đường ướt.");
  say("Nguồn b: mưa gây ra đường trơn.");
  String first=say("Mưa có gây ra đường ướt không?");
  assertTrue(first,first.contains("ủng hộ"));
  String corrected=say("Ý tôi là đường trơn chứ không phải đường ướt");
  assertTrue(corrected,corrected.contains("duong tron"));
  String clarify=say("Bỏ duyên thứ hai thì sao?");
  assertTrue(clarify,clarify.contains("nêu tên duyên"));
 }
}
