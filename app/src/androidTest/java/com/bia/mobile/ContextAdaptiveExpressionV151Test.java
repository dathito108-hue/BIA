package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class ContextAdaptiveExpressionV151Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testAdaptiveDepthAndExplicitOverride(){
  say("Nguồn a: mưa gây ra độ ẩm.");
  say("Nguồn b: độ ẩm gây ra đường trơn.");
  say("Nguồn c: mưa gây ra bùn.");
  say("Nguồn d: bùn gây ra đường trơn.");
  String adaptive=say("Mưa có gây ra đường trơn không?");
  assertTrue(adaptive,adaptive.contains("nhánh Duyên"));
  String brief=say("Nói ngắn gọn");
  assertTrue(brief,brief.contains("ủng hộ"));
  assertFalse(brief,brief.contains("Đường suy luận:"));
 }
}
