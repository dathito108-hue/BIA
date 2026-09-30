package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class DuyenWeaveV148Test extends InstrumentationTestCase {
 protected void setUp(){MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testOverlappingDuyenShapesReply(){
  say("Nguồn a: mưa gây ra độ ẩm.");
  say("Nguồn b: độ ẩm gây ra đường trơn.");
  say("Nguồn c: mưa gây ra bùn.");
  say("Nguồn d: bùn gây ra đường trơn.");
  String answer=say("Mưa có gây ra đường trơn không?");
  assertTrue(answer,answer.contains("nhánh Duyên"));
 }
}
