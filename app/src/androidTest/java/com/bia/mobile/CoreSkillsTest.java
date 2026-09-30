package com.bia.mobile;
import android.test.InstrumentationTestCase;
import android.graphics.Bitmap;
public final class CoreSkillsTest extends InstrumentationTestCase {
 public void testActualRenderAndFailureReachCore()throws Exception{
  Bitmap b=SceneEngine.render(SceneEngine.sample(),64);assertEquals(64,b.getWidth());b.recycle();
  assertTrue(CoreSkills.report().contains("scene.render 1"));
  try{CoreSkills.local("image.render",()->{throw new IllegalArgumentException("expected");});fail();}catch(IllegalArgumentException expected){}
  assertTrue(CoreSkills.report().contains("image.render 2"));
 }
 public void testNoWalletAuthorityAndNoDuplicateReceipt(){
  assertEquals(0L,CoreSkills.begin("solana.sign"));assertEquals(0L,CoreSkills.begin("dex.broadcast"));
  long id=CoreSkills.begin("image.render");assertTrue(id>0);assertTrue(CoreSkills.finish(id,true));assertFalse(CoreSkills.finish(id,true));
 }
}
