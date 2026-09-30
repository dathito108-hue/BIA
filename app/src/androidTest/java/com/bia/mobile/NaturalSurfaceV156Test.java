package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class NaturalSurfaceV156Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testNaturalGroundedFlow(){
  say("Nguồn v156a: mưa gây ra độ ẩm.");
  say("Nguồn v156b: độ ẩm gây ra đường trơn.");
  say("Nguồn v156c: mưa gây ra bùn.");
  say("Nguồn v156d: bùn gây ra đường trơn.");
  String r=say("Mưa có gây ra đường trơn không?");
  assertTrue(r,r.startsWith("Nếu nhìn theo các Duyên đang chồng lên nhau"));
  assertTrue(r,r.contains("nhánh Duyên"));
  String brief=say("Nói ngắn gọn");
  assertTrue(brief,brief.contains("ủng hộ"));
  assertFalse(brief,brief.contains("Đường suy luận:"));
 }
}
