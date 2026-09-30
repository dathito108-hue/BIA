package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class ContextualPragmaticsV158Test extends InstrumentationTestCase {
 protected void setUp() throws Exception {super.setUp();MainActivity.nativeResetDialogueForTests();}
 String say(String s){return MainActivity.nativeChat(s,300000,0.9f,0.1f,0.1f,2048);}
 public void testSafeEllipsisAndTopicShift(){
  say("Nguồn v158a: mưa gây ra độ ẩm.");
  say("Nguồn v158b: mưa gây ra bùn.");
  say("Mưa có gây ra độ ẩm không?");
  String object=say("Còn bùn?");
  assertTrue(object,object.startsWith("Tiếp theo mạch về"));
  assertTrue(object,object.contains("bun"));

  MainActivity.nativeResetDialogueForTests();
  say("Nguồn v158c: mưa gây ra bùn.");
  say("Nguồn v158d: bùn gây ra đường trơn.");
  say("Mưa có gây ra đường trơn không?");
  String ambiguous=say("Thế còn bùn thì sao?");
  assertTrue(ambiguous,ambiguous.contains("nguyên nhân hay kết quả"));

  String shifted=say("Chuyển chủ đề sang âm nhạc");
  assertTrue(shifted,shifted.contains("am nhac"));
  String why=say("Tại sao?");
  assertTrue(why,why.contains("câu hỏi nào"));
 }
}
