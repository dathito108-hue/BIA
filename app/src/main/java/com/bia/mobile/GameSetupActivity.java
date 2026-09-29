package com.bia.mobile;
import android.app.*;
import android.content.*;
import android.content.pm.*;
import android.media.projection.*;
import android.os.*;
import android.provider.Settings;
import android.widget.*;
import java.util.*;

public final class GameSetupActivity extends Activity {
    final ArrayList<String> packages=new ArrayList<>();
    Spinner games,modes;
    @Override public void onCreate(Bundle state){
        super.onCreate(state);
        LinearLayout box=new LinearLayout(this);box.setOrientation(LinearLayout.VERTICAL);box.setPadding(20,20,20,20);
        TextView help=new TextView(this);help.setText("BIA Game V134 — quan sát và đa chạm\n\nChỉ hoạt động trên game bạn chọn. Ảnh xử lý cục bộ, không lưu/gửi. Bật Trợ năng và đồng ý chia sẻ toàn màn hình. Sau khi mở game, Hiệu chỉnh rồi bấm Bật trên thanh nổi.\n\nĐiều khiển theo dấu màu đã chọn, chưa hiểu chiến thuật hoặc mọi đối tượng trong game. Dùng sân tập trước. Dừng khi đổi cửa sổ, xoay màn hình, khóa máy, hết 5 phút / 1200 cử chỉ. Không khôi phục tự chạy sau gián đoạn.");box.addView(help);
        games=new Spinner(this);ArrayList<String> names=new ArrayList<>();packages.add(getPackageName());names.add("BIA — sân tập đa chạm");
        Intent launcher=new Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER);
        for(ResolveInfo r:getPackageManager().queryIntentActivities(launcher,0)){
            String pkg=r.activityInfo.packageName;if(packages.contains(pkg))continue;packages.add(pkg);names.add(r.loadLabel(getPackageManager())+" — "+pkg);
        }
        games.setAdapter(new ArrayAdapter<>(this,android.R.layout.simple_spinner_dropdown_item,names));box.addView(games);
        modes=new Spinner(this);modes.setAdapter(new ArrayAdapter<>(this,android.R.layout.simple_spinner_dropdown_item,new String[]{"FPS — kéo ngắm và bắn khi gần tâm","MOBA — tiến tới dấu mục tiêu, đánh và chiêu"}));box.addView(modes);
        Button accessibility=new Button(this);accessibility.setText("1. Bật BIA Game trong Trợ năng");accessibility.setOnClickListener(v->startActivity(new Intent(Settings.ACTION_ACCESSIBILITY_SETTINGS)));box.addView(accessibility);
        Button begin=new Button(this);begin.setText("2. Đồng ý đọc màn hình và mở game");begin.setOnClickListener(v->{
            if(GameAccessibilityService.instance==null){Toast.makeText(this,"Hãy bật BIA Game trong Trợ năng trước",Toast.LENGTH_LONG).show();return;}
            if(GameCaptureService.instance!=null){Toast.makeText(this,"Dừng phiên hiện tại trước",Toast.LENGTH_LONG).show();return;}
            MediaProjectionManager manager=getSystemService(MediaProjectionManager.class);
            Intent consent=Build.VERSION.SDK_INT>=34?manager.createScreenCaptureIntent(MediaProjectionConfig.createConfigForDefaultDisplay()):manager.createScreenCaptureIntent();
            startActivityForResult(consent,134);
        });box.addView(begin);
        Button arena=new Button(this);arena.setText("Mở sân tập bằng tay");arena.setOnClickListener(v->startActivity(new Intent(this,GameTrainingActivity.class)));box.addView(arena);
        ScrollView scroll=new ScrollView(this);scroll.addView(box);setContentView(scroll);
    }
    @Override protected void onActivityResult(int request,int result,Intent data){
        super.onActivityResult(request,result,data);if(request!=134 || result!=RESULT_OK || data==null)return;
        String target=packages.get(games.getSelectedItemPosition());boolean moba=modes.getSelectedItemPosition()==1;
        Intent service=new Intent(this,GameCaptureService.class).putExtra("target",target).putExtra("moba",moba).putExtra("consent",data);
        startForegroundService(service);
        Intent launch=target.equals(getPackageName())?new Intent(this,GameTrainingActivity.class).putExtra("moba",moba):getPackageManager().getLaunchIntentForPackage(target);
        if(launch!=null)startActivity(launch);else{stopService(service);Toast.makeText(this,"Không mở được game đã chọn",Toast.LENGTH_LONG).show();}
    }
}
