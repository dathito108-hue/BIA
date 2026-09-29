package com.bia.mobile;
import android.app.*;
import android.content.*;
import android.graphics.Bitmap;
import android.os.SystemClock;
import android.test.InstrumentationTestCase;
import android.view.accessibility.*;
import java.io.FileOutputStream;
import java.util.List;

/** Runs on a disposable emulator; all input is limited to BIA's local arena and OS consent dialog. */
public final class GameLoopTest extends InstrumentationTestCase {
    UiAutomation ui;
    public void testProjectionNativeVisionAndRealMultitouch() throws Exception {
        ui=getInstrumentation().getUiAutomation(UiAutomation.FLAG_DONT_SUPPRESS_ACCESSIBILITY_SERVICES);
        android.accessibilityservice.AccessibilityServiceInfo info=ui.getServiceInfo();
        info.flags|=android.accessibilityservice.AccessibilityServiceInfo.FLAG_RETRIEVE_INTERACTIVE_WINDOWS;ui.setServiceInfo(info);
        Intent setup=new Intent(getInstrumentation().getTargetContext(),GameSetupActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        GameSetupActivity activity=(GameSetupActivity)getInstrumentation().startActivitySync(setup);
        // am instrument force-stops the target process; bind only after that restart.
        shell("settings delete secure enabled_accessibility_services");
        shell("settings put secure enabled_accessibility_services com.bia.mobile/.GameAccessibilityService");
        shell("settings put secure accessibility_enabled 1");
        waitFor(()->GameAccessibilityService.instance!=null,20000,"accessibility service");
        getInstrumentation().runOnMainSync(()->activity.modes.setSelection(1));
        click("2. Đồng ý đọc màn hình và mở game",5000);
        long end=SystemClock.elapsedRealtime()+15000;
        while(SystemClock.elapsedRealtime()<end && GameCaptureService.instance==null){
            AccessibilityNodeInfo root=ui.getRootInActiveWindow();
            if(root!=null && root.getPackageName()!=null && root.getPackageName().toString().contains("systemui")) {
                for(String name:new String[]{"Start now","Start recording","Start","Share screen"}) if(clickOnce(name)) break;
                List<AccessibilityNodeInfo> buttons=root.findAccessibilityNodeInfosByViewId("android:id/button1");
                for(AccessibilityNodeInfo button:buttons) if(button.isEnabled())button.performAction(AccessibilityNodeInfo.ACTION_CLICK);
            }
            Thread.sleep(150);
        }
        waitFor(()->GameCaptureService.instance!=null && GameCaptureService.instance.latest!=null && GameTrainingActivity.visible,10000,"projection frames in arena");
        // First immersive launch presents an Android-owned tutorial window.
        clickOnce("Got it");
        waitFor(()->{clickOnce("Got it");return GameAccessibilityService.instance.targetForeground();},10000,"arena is the active window");
        waitFor(()->GameCaptureService.instance.latest!=null && SystemClock.elapsedRealtime()-GameCaptureService.instance.latest.time<350,5000,"fresh arena frame");
        click("Bật 5 phút",5000);
        waitFor(()->GameTrainingActivity.hits>=3 && GameTrainingActivity.maxPointers>=2,15000,"arena hits and delivered multitouch");
        GameAccessibilityService service=GameAccessibilityService.instance;
        assertTrue("native decisions dispatched",service.completed>=1);
        assertTrue("multi-pointer MotionEvent arrived",GameTrainingActivity.maxPointers>=2);
        System.out.println("BIA_GAME_EVIDENCE frames="+service.frames+" completed="+service.completed+" arenaHits="+GameTrainingActivity.hits+" actualMaxPointers="+GameTrainingActivity.maxPointers);
        Bitmap shot=ui.takeScreenshot();if(shot!=null){try(FileOutputStream out=new FileOutputStream(getInstrumentation().getTargetContext().getExternalFilesDir(null)+"/game-proof.png")){shot.compress(Bitmap.CompressFormat.PNG,100,out);}shot.recycle();}
        getInstrumentation().getTargetContext().startActivity(new Intent(getInstrumentation().getTargetContext(),MainActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
        waitFor(()->!service.armed,5000,"pause outside arena");Thread.sleep(200);int stopped=service.completed;Thread.sleep(600);
        assertEquals("no gestures after leaving target",stopped,service.completed);
        click("DỪNG",5000);waitFor(()->GameCaptureService.instance==null,5000,"session stop");
        assertFalse("permission not armed after stop",service.armed);
    }
    void shell(String command)throws Exception {
        try(java.io.InputStream in=new android.os.ParcelFileDescriptor.AutoCloseInputStream(ui.executeShellCommand(command))){
            byte[] buffer=new byte[1024];while(in.read(buffer)!=-1){}
        }
    }
    interface Check {boolean yes();}
    void waitFor(Check check,long timeout,String label)throws Exception{
        long until=SystemClock.elapsedRealtime()+timeout;while(SystemClock.elapsedRealtime()<until){if(check.yes())return;Thread.sleep(100);}fail("Timed out: "+label+"; UI="+dump());
    }
    void click(String text,long timeout)throws Exception{waitFor(()->clickOnce(text),timeout,"button "+text);}
    boolean clickOnce(String text){
        for(AccessibilityWindowInfo window:ui.getWindows()) if(clickNode(window.getRoot(),text,0))return true;
        return false;
    }
    boolean clickNode(AccessibilityNodeInfo node,String text,int depth){
        if(node==null || depth>16)return false;
        if(node.getText()!=null && node.getText().toString().equalsIgnoreCase(text) && node.isEnabled()){
            if(!node.isVisibleToUser()){node.performAction(AccessibilityNodeInfo.AccessibilityAction.ACTION_SHOW_ON_SCREEN.getId());return false;}
            AccessibilityNodeInfo p=node;while(p!=null && !p.isClickable())p=p.getParent();
            if(p!=null && p.performAction(AccessibilityNodeInfo.ACTION_CLICK))return true;
        }
        for(int i=0;i<node.getChildCount();i++)if(clickNode(node.getChild(i),text,depth+1))return true;
        return false;
    }
    String dump(){StringBuilder s=new StringBuilder();for(AccessibilityWindowInfo w:ui.getWindows())walk(w.getRoot(),s,0);return s.toString();}
    void walk(AccessibilityNodeInfo n,StringBuilder s,int depth){if(n==null || depth>12)return;s.append("[").append(n.getPackageName()).append(":").append(n.getText()).append(":").append(n.getViewIdResourceName()).append("]");for(int i=0;i<n.getChildCount();i++)walk(n.getChild(i),s,depth+1);}
    @Override protected void tearDown()throws Exception{getInstrumentation().getTargetContext().stopService(new Intent(getInstrumentation().getTargetContext(),GameCaptureService.class));super.tearDown();}
}
