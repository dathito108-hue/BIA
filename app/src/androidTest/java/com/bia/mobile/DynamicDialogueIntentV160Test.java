package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class DynamicDialogueIntentV160Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testComposedIntentPlanning(){
  say("Nguồn v160a: mưa gây ra đường trơn.");
  say("Mưa có gây ra đường trơn không?");
  say("Nguồn v160b: mưa gây ra độ ẩm.");
  say("Nguồn v160c: độ ẩm gây ra đường trơn.");
  String delta=say("Tôi hiểu rồi, nhưng có chắc không, nếu có gì mới thì chỉ nói phần mới thôi");
  assertTrue(delta,delta.contains("Phần mới"));
  assertTrue(delta,delta.contains("nguồn thêm"));
  assertFalse(delta,delta.contains("Đường suy luận:"));

  String expand=say("Đúng chứ, nói thêm đi");
  assertTrue(expand,expand.contains("Mở rộng thêm"));
  assertTrue(expand,expand.contains("Đường suy luận:"));
 }
}
