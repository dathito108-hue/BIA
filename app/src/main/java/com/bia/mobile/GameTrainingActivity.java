package com.bia.mobile;
import android.app.Activity;
import android.content.Context;
import android.graphics.*;
import android.os.Bundle;
import android.view.*;
import java.util.HashMap;

/** Local fixture measures delivered touch events, never claimed as a commercial game benchmark. */
public final class GameTrainingActivity extends Activity {
    static volatile boolean visible;
    static volatile int hits,maxPointers;
    @Override public void onCreate(Bundle b){
        super.onCreate(b);hits=maxPointers=0;
        getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_FULLSCREEN|View.SYSTEM_UI_FLAG_HIDE_NAVIGATION|View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY|View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN|View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION|View.SYSTEM_UI_FLAG_LAYOUT_STABLE);
        if(android.os.Build.VERSION.SDK_INT>=28)getWindow().getAttributes().layoutInDisplayCutoutMode=WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
        setContentView(new Arena(this));
    }
    @Override protected void onResume(){super.onResume();visible=true;}
    @Override protected void onPause(){visible=false;super.onPause();}
    final class Arena extends View {
        float enemyX=.62f,enemyY=.38f;
        final Paint paint=new Paint(3);
        final HashMap<Integer,PointF> previous=new HashMap<>();
        Arena(Context c){super(c);setContentDescription("BIA sân tập: mục tiêu đỏ, cần trái, đánh và kỹ năng bên phải");}
        @Override protected void onDraw(Canvas c){
            c.drawColor(Color.rgb(12,22,30));paint.setColor(Color.WHITE);paint.setTextSize(24);
            c.drawText("SÂN TẬP BIA — trúng "+hits+" | chạm đồng thời "+maxPointers,15,getHeight()*.15f,paint);
            circle(c,enemyX,enemyY,.024f,0xffff3030);circle(c,.5f,.5f,.009f,Color.WHITE);
            circle(c,.14f,.79f,.055f,0xff507080);circle(c,.88f,.8f,.04f,0xff40b0a0);circle(c,.74f,.8f,.035f,0xff8070d0);
            paint.setTextSize(18);paint.setColor(Color.WHITE);c.drawText("DI CHUYỂN",getWidth()*.06f,getHeight()*.91f,paint);c.drawText("CHIÊU      ĐÁNH",getWidth()*.69f,getHeight()*.91f,paint);
            c.drawText("Nhịp ảnh "+(android.os.SystemClock.elapsedRealtime()/100),15,getHeight()*.98f,paint);
            postInvalidateDelayed(80);
        }
        void circle(Canvas c,float x,float y,float radius,int color){paint.setColor(color);c.drawCircle(x*getWidth(),y*getHeight(),radius*Math.min(getWidth(),getHeight()),paint);}
        @Override public boolean onTouchEvent(MotionEvent e){
            maxPointers=Math.max(maxPointers,e.getPointerCount());int action=e.getActionMasked();
            for(int i=0;i<e.getPointerCount();i++){
                float x=e.getX(i)/getWidth(),y=e.getY(i)/getHeight();int id=e.getPointerId(i);PointF old=previous.get(id);
                if(action==MotionEvent.ACTION_MOVE && old!=null){
                    if(x>.35f && x<.9f && y>.2f && y<.68f){enemyX-=x-old.x;enemyY-=y-old.y;}
                    if(x<.24f && y>.68f){enemyX-=(x-.14f)*.3f;enemyY-=(y-.79f)*.3f;}
                }
                if((action==MotionEvent.ACTION_DOWN || action==MotionEvent.ACTION_POINTER_DOWN) && i==e.getActionIndex()){
                    if(y>.72f && y<.87f && x>.69f && x<.94f && Math.hypot(enemyX-.5f,enemyY-.5f)<.22f){hits++;enemyX=.62f;enemyY=.38f;}
                }
                previous.put(id,new PointF(x,y));
            }
            if(action==MotionEvent.ACTION_UP || action==MotionEvent.ACTION_CANCEL)previous.clear();
            enemyX=Math.max(.27f,Math.min(.73f,enemyX));enemyY=Math.max(.2f,Math.min(.62f,enemyY));invalidate();return true;
        }
    }
}
