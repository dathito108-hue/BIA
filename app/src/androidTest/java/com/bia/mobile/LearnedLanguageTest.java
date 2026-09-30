package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class LearnedLanguageTest extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testLearnedExpressionAndRoleFollowup(){
  assertTrue(say("Cách nói làm phát sinh: gây ra").contains("Đã học"));
  say("Ghi nhớ rằng gió mạnh làm phát sinh sóng lớn.");
  say("Ghi nhớ rằng sóng lớn gây ra thuyền lắc.");
  String answer=say("Gió mạnh có làm phát sinh sóng lớn không?");assertTrue(answer,answer.contains("ủng hộ"));
  assertTrue(say("Còn kết quả thuyền lắc thì sao?").contains("2 mắt xích"));
  assertTrue(say("Dựa vào đâu?").contains("hoithoai"));
 }
}
