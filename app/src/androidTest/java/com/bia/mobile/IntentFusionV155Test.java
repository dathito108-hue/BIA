package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class IntentFusionV155Test extends InstrumentationTestCase {
 protected void setUp(){MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testFocusExcludeAndPreviousConclusion(){
  say("Nguồn a: mưa gây ra đường trơn.");
  say("Nguồn b: gió gây ra sóng.");
  say("Mưa có gây ra đường trơn không?");
  say("Gió có gây ra sóng không?");
  String r=say("Giải thích nhưng tập trung vào sóng, bỏ phần so sánh và kết luận theo trường hợp trước");
  assertTrue(r,r.contains("Trọng tâm:"));
  assertTrue(r,r.contains("Giải thích:"));
  assertTrue(r,r.contains("Kết luận theo trường hợp trước:"));
  assertFalse(r,r.contains("Đối chiếu:"));
 }
}
