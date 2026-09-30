package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class IntentFusionV155Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testFocusExcludeAndPreviousConclusion(){
  say("Nguồn a: mưa gây ra đường trơn.");
  say("Nguồn b: gió gây ra sóng.");
  say("Mưa có gây ra đường trơn không?");
  say("Gió có gây ra sóng không?");
  String r=say("Giải thích nhưng tập trung vào sóng, bỏ phần so sánh và kết luận theo trường hợp trước");
  assertTrue(r,r.contains("giữ trọng tâm"));
  assertTrue(r,r.contains("Trước hết"));
  assertTrue(r,r.contains("mốc tham chiếu"));
  assertFalse(r,r.contains("Đặt cạnh trường hợp trước"));
  assertFalse(r,r.contains("Giải thích:"));
 }
}
