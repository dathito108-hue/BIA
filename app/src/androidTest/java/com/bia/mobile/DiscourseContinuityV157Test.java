package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class DiscourseContinuityV157Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testContinuityAndAntiRepetition(){
  say("Nguồn v157a: mưa gây ra độ ẩm.");
  say("Nguồn v157b: độ ẩm gây ra đường trơn.");
  say("Nguồn v157c: mưa gây ra bùn.");
  String first=say("Mưa có gây ra đường trơn không?");
  assertTrue(first,first.contains("Đường suy luận:"));
  String next=say("Mưa có gây ra bùn không?");
  assertTrue(next,next.startsWith("Tiếp theo mạch về"));
  String repeated=say("Mưa có gây ra bùn không?");
  assertTrue(repeated,repeated.startsWith("Vẫn ở quan hệ này"));
  assertTrue(repeated,repeated.contains("Mạch bằng chứng không đổi"));
  assertFalse(repeated,repeated.contains("Đường suy luận:"));
 }
}
