package com.bia.mobile;
import android.app.*;
import android.content.*;
import android.content.pm.ServiceInfo;
import android.graphics.PixelFormat;
import android.hardware.display.*;
import android.media.*;
import android.media.projection.*;
import android.os.*;
import android.util.DisplayMetrics;
import android.view.WindowManager;
import java.nio.ByteBuffer;
import java.util.concurrent.atomic.AtomicBoolean;

public final class GameCaptureService extends Service {
    static volatile GameCaptureService instance;
    static final String STOP="com.bia.mobile.GAME_STOP";
    GameProfile profile;
    volatile Frame latest;
    MediaProjection projection;
    VirtualDisplay display;
    ImageReader reader;
    HandlerThread worker;
    final Handler main=new Handler(Looper.getMainLooper());
    final AtomicBoolean queued=new AtomicBoolean();
    long lastFrame;
    boolean closed;
    static final class Frame {
        final int[] pixels;final int width,height;final long time;
        Frame(int[] p,int w,int h,long t){pixels=p;width=w;height=h;time=t;}
    }
    @Override public IBinder onBind(Intent i){return null;}
    @Override public int onStartCommand(Intent i,int flags,int id){
        if(i==null || STOP.equals(i.getAction())){stopSelf();return START_NOT_STICKY;}
        if(instance!=null || GameAccessibilityService.instance==null){stopSelf();return START_NOT_STICKY;}
        instance=this;
        try {
            NotificationManager nm=getSystemService(NotificationManager.class);
            nm.createNotificationChannel(new NotificationChannel("bia_game","BIA Game — đang đọc màn hình",NotificationManager.IMPORTANCE_LOW));
            Intent stop=new Intent(this,GameCaptureService.class).setAction(STOP);
            PendingIntent pi=PendingIntent.getService(this,134,stop,PendingIntent.FLAG_UPDATE_CURRENT|PendingIntent.FLAG_IMMUTABLE);
            Notification note=new Notification.Builder(this,"bia_game").setSmallIcon(android.R.drawable.ic_media_play)
                    .setContentTitle("BIA Game: xử lý màn hình cục bộ").setContentText("Chạm Dừng để thu hồi phiên. Không lưu/gửi ảnh.")
                    .setOngoing(true).addAction(new Notification.Action.Builder(null,"Dừng",pi).build()).build();
            if(Build.VERSION.SDK_INT>=29)startForeground(134,note,ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION);else startForeground(134,note);
            DisplayMetrics metrics=new DisplayMetrics();getSystemService(WindowManager.class).getDefaultDisplay().getRealMetrics(metrics);
            String target=i.getStringExtra("target");if(target==null)throw new IllegalArgumentException("Missing target");
            profile=new GameProfile(target,i.getBooleanExtra("moba",false),metrics.widthPixels,metrics.heightPixels);
            profile.load(this);
            if(target.equals(getPackageName()))profile.calibrated=true;
            int w=Math.max(8,Math.round(metrics.widthPixels*480f/Math.max(metrics.widthPixels,metrics.heightPixels)));
            int h=Math.max(8,Math.round(metrics.heightPixels*480f/Math.max(metrics.widthPixels,metrics.heightPixels)));
            Intent data=i.getParcelableExtra("consent");
            projection=getSystemService(MediaProjectionManager.class).getMediaProjection(Activity.RESULT_OK,data);
            if(projection==null)throw new IllegalStateException("Projection denied");
            projection.registerCallback(new MediaProjection.Callback(){
                @Override public void onStop(){main.post(()->stopSelf());}
                @Override public void onCapturedContentResize(int width,int height){
                    if(width!=profile.screenWidth || height!=profile.screenHeight)main.post(()->stopSelf());
                }
            },main);
            worker=new HandlerThread("bia-game-capture");worker.start();
            reader=ImageReader.newInstance(w,h,PixelFormat.RGBA_8888,2);
            reader.setOnImageAvailableListener(source->{
                try(Image image=source.acquireLatestImage()){
                    if(image==null || closed)return;
                    long now=SystemClock.elapsedRealtime();if(now-lastFrame<100)return;lastFrame=now;
                    Image.Plane plane=image.getPlanes()[0];ByteBuffer buffer=plane.getBuffer();int stride=plane.getRowStride(),pixel=plane.getPixelStride();
                    int[] colors=new int[w*h];
                    for(int y=0;y<h;y++)for(int x=0;x<w;x++){
                        int n=y*stride+x*pixel;
                        colors[y*w+x]=0xff000000|((buffer.get(n)&255)<<16)|((buffer.get(n+1)&255)<<8)|(buffer.get(n+2)&255);
                    }
                    latest=new Frame(colors,w,h,now);
                    if(queued.compareAndSet(false,true))main.post(()->{
                        queued.set(false);
                        GameAccessibilityService a=GameAccessibilityService.instance;
                        if(!closed && a!=null)a.onFrame(latest);
                    });
                }catch(Exception e){main.post(()->stopSelf());}
            },new Handler(worker.getLooper()));
            display=projection.createVirtualDisplay("BIA-local-game",w,h,metrics.densityDpi,DisplayManager.VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR,reader.getSurface(),null,null);
            GameNative.reset();GameAccessibilityService.instance.showSession(this);
        }catch(Exception e){android.util.Log.e("BIA_GAME","Capture startup failed",e);android.widget.Toast.makeText(this,"Không mở phiên game: "+e.getClass().getSimpleName(),android.widget.Toast.LENGTH_LONG).show();stopSelf();}
        return START_NOT_STICKY;
    }
    @Override public void onDestroy(){
        closed=true;
        if(instance==this)instance=null;
        if(GameAccessibilityService.instance!=null)GameAccessibilityService.instance.endSession();
        if(display!=null)display.release();if(reader!=null)reader.close();if(projection!=null)projection.stop();if(worker!=null)worker.quitSafely();
        latest=null;stopForeground(true);super.onDestroy();
    }
}
