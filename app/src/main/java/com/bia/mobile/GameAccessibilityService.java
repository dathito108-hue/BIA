package com.bia.mobile;
import android.accessibilityservice.*;
import android.app.KeyguardManager;
import android.content.*;
import android.graphics.*;
import android.os.*;
import android.util.DisplayMetrics;
import android.view.*;
import android.view.accessibility.*;
import android.widget.*;

public final class GameAccessibilityService extends AccessibilityService {
    static volatile GameAccessibilityService instance;
    GameCaptureService capture;
    WindowManager wm;
    LinearLayout hud;
    TextView status;
    View calibration;
    boolean armed,busy;
    long expires,lastReceipt,issuedAt,lastObservedFrame;
    int remaining,generation,gestureSerial,frames,completed,cancelled,maxPointers;
    boolean unresolved;
    final Handler handler=new Handler(Looper.getMainLooper());
    @Override protected void onServiceConnected(){instance=this;wm=getSystemService(WindowManager.class);}
    @Override public void onAccessibilityEvent(AccessibilityEvent e){
        if(armed && e.getEventType()==AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED)pause("Đã đổi cửa sổ; cần bật lại");
    }
    @Override public void onInterrupt(){pause("Dịch vụ bị ngắt");}
    @Override public void onDestroy(){if(capture!=null)capture.stopSelf();endSession();instance=null;super.onDestroy();}
    boolean targetForeground(){
        if(capture==null || capture.profile==null)return false;
        if(capture.profile.targetPackage.equals(getPackageName()) && !GameTrainingActivity.visible)return false;
        AccessibilityNodeInfo root=getRootInActiveWindow();
        if(root==null)return false;
        boolean yes=root.getPackageName()!=null && capture.profile.targetPackage.contentEquals(root.getPackageName());root.recycle();return yes;
    }
    void showSession(GameCaptureService s){
        endSession();capture=s;frames=completed=cancelled=maxPointers=0;
        hud=new LinearLayout(this);hud.setOrientation(LinearLayout.VERTICAL);hud.setBackgroundColor(0xee11251d);
        status=new TextView(this);status.setTextColor(Color.WHITE);status.setTextSize(11);hud.addView(status);
        LinearLayout row=new LinearLayout(this);hud.addView(row);
        addButton(row,"Hiệu chỉnh",()->calibrate());
        addButton(row,"Bật 5 phút",()->arm());
        addButton(row,"Tạm dừng",()->pause("Tạm dừng"));
        addButton(row,"DỪNG",()->{if(capture!=null)capture.stopSelf();});
        WindowManager.LayoutParams p=layout(WindowManager.LayoutParams.WRAP_CONTENT,WindowManager.LayoutParams.WRAP_CONTENT);
        p.gravity=Gravity.TOP|Gravity.LEFT;p.y=4;wm.addView(hud,p);message("Chưa bật — mở đúng game, hiệu chỉnh rồi Bật. Giới hạn 1200 cử chỉ / 5 phút.");
    }
    private void addButton(LinearLayout row,String label,Runnable click){
        Button b=new Button(this);b.setText(label);b.setTextSize(10);b.setMinWidth(0);b.setMinimumWidth(0);b.setPadding(5,0,5,0);b.setOnClickListener(v->click.run());row.addView(b);
    }
    private WindowManager.LayoutParams layout(int w,int h){
        WindowManager.LayoutParams p=new WindowManager.LayoutParams(w,h,WindowManager.LayoutParams.TYPE_ACCESSIBILITY_OVERLAY,
                WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE|WindowManager.LayoutParams.FLAG_LAYOUT_IN_SCREEN,PixelFormat.TRANSLUCENT);
        if(Build.VERSION.SDK_INT>=28)p.layoutInDisplayCutoutMode=WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
        return p;
    }
    void arm(){
        if(unresolved){message("Kết quả cử chỉ chưa rõ: DỪNG rồi mở phiên mới sau khi kiểm tra.");return;}
        if(capture==null || !capture.profile.calibrated || calibration!=null || busy || unresolved || !targetForeground()){
            message("Chưa bật: cần hiệu chỉnh và mở đúng game.");return;
        }
        if(capture.latest==null || SystemClock.elapsedRealtime()-capture.latest.time>400){message("Chưa có ảnh mới.");return;}
        armed=true;generation++;issuedAt=SystemClock.elapsedRealtime();expires=issuedAt+300000;remaining=1200;lastObservedFrame=SystemClock.elapsedRealtime();GameNative.reset();message("ĐÃ BẬT — chỉ game đã chọn. DỪNG luôn thu hồi phiên.");
    }
    void pause(String reason){armed=false;generation++;GameNative.reset();message(reason);}
    void message(String text){if(status!=null)status.setText(text+" | ảnh "+frames+" / cử chỉ "+completed+" / hủy "+cancelled);}
    void endSession(){
        armed=false;generation++;gestureSerial++;unresolved=false;handler.removeCallbacksAndMessages(null);
        if(calibration!=null){try{wm.removeView(calibration);}catch(Exception ignored){}calibration=null;}
        if(hud!=null){try{wm.removeView(hud);}catch(Exception ignored){}hud=null;}
        status=null;capture=null;busy=false;
    }
    void onFrame(GameCaptureService.Frame frame){
        if(capture==null || frame==null)return;frames++;
        if(armed && lastObservedFrame>0 && frame.time-lastObservedFrame>800){pause("Luồng ảnh bị gián đoạn; cần bật lại");return;}
        lastObservedFrame=frame.time;
        if(!armed || busy)return;
        long now=SystemClock.elapsedRealtime();
        DisplayMetrics metrics=new DisplayMetrics();wm.getDefaultDisplay().getRealMetrics(metrics);
        if(metrics.widthPixels!=capture.profile.screenWidth || metrics.heightPixels!=capture.profile.screenHeight){capture.stopSelf();return;}
        if(now<issuedAt || now>=expires || remaining<=0 || now-frame.time>400 || !targetForeground()
                || getSystemService(KeyguardManager.class).isKeyguardLocked()) {pause("Hết hạn, ảnh cũ hoặc rời game");return;}
        PowerManager power=getSystemService(PowerManager.class);
        if(Build.VERSION.SDK_INT>=29 && power.getCurrentThermalStatus()>=PowerManager.THERMAL_STATUS_SEVERE){pause("Máy quá nóng");return;}
        float[] d=GameNative.observe(frame.pixels,frame.width,frame.height,capture.profile.nativeParams(),frame.time);
        if(d==null || d.length!=9 || d[8]<.5f){message("Đang tìm dấu mục tiêu đã hiệu chỉnh");return;}
        GestureDescription.Builder gesture=new GestureDescription.Builder();int fingers=0;GameProfile p=capture.profile;
        if(Math.hypot(d[2],d[3])>.1){stroke(gesture,p.joystickX,p.joystickY,p.joystickX+d[2]*.055f,p.joystickY+d[3]*.055f,80);fingers++;}
        if(Math.hypot(d[4],d[5])>.003){stroke(gesture,p.aimX,p.aimY,p.aimX+d[4],p.aimY+d[5],80);fingers++;}
        if(d[6]>.5){stroke(gesture,p.attackX,p.attackY,p.attackX,p.attackY,70);fingers++;}
        if(d[7]>.5){stroke(gesture,p.skillX,p.skillY,p.skillX,p.skillY,60);fingers++;}
        if(fingers==0)return;
        remaining--;busy=true;int token=generation;int serial=++gestureSerial;maxPointers=Math.max(maxPointers,fingers);
        boolean accepted=dispatchGesture(gesture.build(),new GestureResultCallback(){
            @Override public void onCompleted(GestureDescription g){if(serial!=gestureSerial)return;busy=false;completed++;lastReceipt=SystemClock.elapsedRealtime();if(token==generation)message("Đang chạy, còn "+remaining+" cử chỉ");}
            @Override public void onCancelled(GestureDescription g){if(serial!=gestureSerial)return;busy=false;cancelled++;if(token==generation)pause("Cử chỉ bị hủy; cần bật lại");}
        },handler);
        if(!accepted){busy=false;pause("Android từ chối cử chỉ");return;}
        handler.postDelayed(()->{if(busy && serial==gestureSerial){busy=false;unresolved=true;gestureSerial++;pause("Chưa có kết quả cử chỉ; không phát lại");}},800);
    }
    void stroke(GestureDescription.Builder b,float x,float y,float ex,float ey,long duration){
        GameProfile p=capture.profile;Path path=new Path();
        path.moveTo(clamp(x)*p.screenWidth,clamp(y)*p.screenHeight);
        if(x!=ex || y!=ey)path.lineTo(clamp(ex)*p.screenWidth,clamp(ey)*p.screenHeight);
        b.addStroke(new GestureDescription.StrokeDescription(path,0,duration));
    }
    static float clamp(float x){return Math.max(.02f,Math.min(.98f,x));}
    void calibrate(){
        if(capture==null || capture.latest==null || !targetForeground() || busy)return;
        pause("Hiệu chỉnh");
        if(calibration!=null)return;
        capture.profile.calibrated=false;
        calibration=new Calibration(this,capture.latest,capture.profile);
        wm.addView(calibration,layout(WindowManager.LayoutParams.MATCH_PARENT,WindowManager.LayoutParams.MATCH_PARENT));
    }
    final class Calibration extends View {
        final Bitmap image;final GameProfile p;int step;final Paint brush=new Paint(3);
        final String[] labels={"Chạm góc TRÊN TRÁI vùng tìm mục tiêu","Chạm góc DƯỚI PHẢI vùng tìm mục tiêu","Chạm dấu MÀU mục tiêu (không chọn nền)","Chạm tâm cần DI CHUYỂN","Chạm nút BẮN / ĐÁNH","Chạm nút KỸ NĂNG","Chạm vùng kéo NGẮM (tránh nút)"};
        Calibration(Context c,GameCaptureService.Frame f,GameProfile profile){super(c);p=profile;image=Bitmap.createBitmap(f.pixels,f.width,f.height,Bitmap.Config.ARGB_8888);}
        @Override protected void onDraw(Canvas c){
            c.drawBitmap(image,null,new Rect(0,0,getWidth(),getHeight()),brush);
            brush.setColor(0xee000000);c.drawRect(0,0,getWidth(),100,brush);
            brush.setColor(Color.WHITE);brush.setTextSize(22);c.drawText((step+1)+"/7 "+labels[step],12,36,brush);c.drawText("Chạm góc phải thanh này để HỦY; hoàn tất chưa tự chạy.",12,74,brush);
        }
        @Override public boolean onTouchEvent(MotionEvent e){
            if(e.getAction()!=MotionEvent.ACTION_UP)return true;
            if(e.getY()<100){
                if(e.getX()>getWidth()*.8f){wm.removeView(this);calibration=null;image.recycle();message("Đã hủy hiệu chỉnh; chưa cấp quyền chạy.");}
                return true;
            }
            float x=clamp(e.getX()/getWidth()),y=clamp(e.getY()/getHeight());
            switch(step){
                case 0:p.left=x;p.top=y;break;
                case 1:if(x-p.left<.05f || y-p.top<.05f)return true;p.right=x;p.bottom=y;break;
                case 2:p.color=image.getPixel(Math.min(image.getWidth()-1,(int)(x*image.getWidth())),Math.min(image.getHeight()-1,(int)(y*image.getHeight())));break;
                case 3:p.joystickX=x;p.joystickY=y;break;
                case 4:p.attackX=x;p.attackY=y;break;
                case 5:p.skillX=x;p.skillY=y;break;
                case 6:p.aimX=x;p.aimY=y;break;
            }
            step++;
            if(step==7){p.calibrated=true;p.save(GameAccessibilityService.this);wm.removeView(this);calibration=null;image.recycle();message("Đã hiệu chỉnh. Bấm Bật để cấp phiên 5 phút / 1200 cử chỉ.");}else invalidate();
            return true;
        }
    }
}
