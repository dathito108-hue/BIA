package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class ConversationLanguageTest extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,200000,0.9f,0.1f,0.1f,2048);}
 public void testNaturalDialogueUsesCoreEvidence(){
  assertTrue(say("Ghi nhớ rằng đèn sáng gây ra phòng sáng.").contains("ghi nguồn"));
  assertTrue(say("Hãy nhớ rằng phòng sáng gây ra dễ đọc.").contains("ghi nguồn"));
  String answer=say("BIA ơi, đèn sáng có dẫn tới dễ đọc không vậy?");
  assertTrue(answer,answer.contains("2 mắt xích"));assertTrue(answer,answer.contains("den sang → phong sang → de doc"));
  assertTrue(say("Giải thích thêm").contains("2 mắt xích"));
  assertTrue(say("Dựa vào đâu?").contains("hoithoai"));
  assertFalse(say("Nói ngắn gọn").contains("Đường suy luận"));
 }
}
