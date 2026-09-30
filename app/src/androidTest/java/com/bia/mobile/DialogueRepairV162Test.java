package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class DialogueRepairV162Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testRepairAndResume(){
  say("Nguồn v162a: mưa gây ra đường trơn.");
  say("Mưa có gây ra đường trơn không?");
  String r=say("Bạn hiểu sai rồi");
  assertTrue(r,r.contains("điểm lệch"));
  String q=say("Ý tôi là hỏi nguyên nhân của đường trơn");
  assertTrue(q,q.contains("sửa mạch hiểu"));
  String next=say("Mưa có gây ra đường trơn không?");
  assertTrue(next,next.contains("đường trơn")||next.contains("bằng chứng"));
 }
}
