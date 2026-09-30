package com.bia.mobile;
import android.test.InstrumentationTestCase;
public final class GeneralTrainingTest extends InstrumentationTestCase {
 public void testTrainingThroughCanonicalChat(){
  String result=MainActivity.nativeChat("huấn luyện tổng quát",100000,0.9f,0.1f,0.1f,2048);
  assertTrue(result,result.contains("qualified_for_this_curriculum=true"));
  String answer=MainActivity.nativeChat("Hỏi: hoc144tepvao có dẫn tới hoc144tepra không?",100001,0.9f,0.1f,0.1f,2048);
  assertTrue(answer,answer.contains("2 mắt xích"));
  String repeated=MainActivity.nativeChat("huấn luyện tổng quát",100002,0.9f,0.1f,0.1f,2048);
  assertTrue(repeated,repeated.contains("Không thay đổi trí nhớ"));
 }
}
