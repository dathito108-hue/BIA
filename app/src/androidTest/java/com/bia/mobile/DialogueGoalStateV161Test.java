package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class DialogueGoalStateV161Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testImplicitGoalProgression(){
  say("Nguồn v161a: mưa gây ra đường trơn.");
  say("Nguồn v161b: mưa gây ra bùn.");
  say("Nguồn v161c: bùn ngăn đường trơn.");
  say("Mưa có gây ra đường trơn không?");
  String verify=say("Có chắc không?");
  assertTrue(verify,verify.contains("không tăng độ chắc"));
  String challenge=say("Có phản chứng không?");
  assertTrue(challenge,challenge.contains("v161c"));
  String conclusion=say("Vậy kết luận thế nào?");
  assertTrue(conclusion,conclusion.contains("Sau bước kiểm tra phản chứng"));

  say("Chuyển chủ đề sang âm nhạc");
  String cleared=say("Chốt lại");
  assertTrue(cleared,cleared.contains("Chưa có quan hệ hiện tại"));
 }
}
