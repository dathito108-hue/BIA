package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class ConversationalImplicatureV159Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testImplicatureAndResponseEconomy(){
  say("Nguồn v159a: mưa gây ra độ ẩm.");
  say("Nguồn v159b: độ ẩm gây ra đường trơn.");
  say("Mưa có gây ra đường trơn không?");
  String ack=say("Hiểu rồi");
  assertTrue(ack,ack.contains("không lặp lại"));
  String doubt=say("Có chắc không?");
  assertTrue(doubt,doubt.contains("không tăng độ chắc"));
  assertTrue(doubt,doubt.contains("Đường suy luận:"));
  String none=say("Có gì mới?");
  assertTrue(none,none.contains("Chưa có bằng chứng mới"));
  say("Nguồn v159c: mưa gây ra bùn.");
  String delta=say("Chỉ nói phần mới");
  assertTrue(delta,delta.contains("nguồn thêm"));
  assertTrue(delta,delta.contains("v159c"));
 }
}
