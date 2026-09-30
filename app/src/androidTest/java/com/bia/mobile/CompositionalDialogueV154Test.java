package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class CompositionalDialogueV154Test extends InstrumentationTestCase {
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testExplainCompareAndSummarize(){
  say("Nguồn a: mưa gây ra đường ướt.");
  say("Nguồn b: gió gây ra sóng.");
  say("Mưa có gây ra đường ướt không?");
  say("Gió có gây ra sóng không?");
  String r=say("Giải thích rõ, so sánh với trường hợp trước rồi tóm tắt ngắn gọn");
  assertTrue(r,r.contains("Giải thích hiện tại:"));
  assertTrue(r,r.contains("Đối chiếu:"));
  assertTrue(r,r.contains("Tóm tắt:"));
 }
}
