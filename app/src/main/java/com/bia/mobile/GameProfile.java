package com.bia.mobile;
import android.content.Context;
import android.content.SharedPreferences;
final class GameProfile {
    final String targetPackage;
    final boolean moba;
    final int screenWidth, screenHeight;
    float left=.25f, top=.18f, right=.75f, bottom=.64f;
    int color=0xffff3030;
    float joystickX=.14f, joystickY=.79f, attackX=.88f, attackY=.80f, skillX=.74f, skillY=.80f, aimX=.62f, aimY=.5f;
    boolean calibrated;
    GameProfile(String target, boolean mode, int w, int h) { targetPackage=target;moba=mode;screenWidth=w;screenHeight=h; }
    float[] nativeParams() { return new float[]{left,top,right,bottom,(color>>16)&255,(color>>8)&255,color&255,35,moba?1:0}; }
    private String key(){return targetPackage+":"+moba+":"+screenWidth+"x"+screenHeight;}
    void save(Context c) {
        String value=left+","+top+","+right+","+bottom+","+color+","+joystickX+","+joystickY+","+attackX+","+attackY+","+skillX+","+skillY+","+aimX+","+aimY;
        c.getSharedPreferences("bia_game_profiles",0).edit().putString(key(),value).apply();
    }
    void load(Context c) {
        String raw=c.getSharedPreferences("bia_game_profiles",0).getString(key(),"");
        try {
            String[] p=raw.split(",");if(p.length!=13)return;
            float[] v=new float[13];for(int i=0;i<13;i++)v[i]=Float.parseFloat(p[i]);
            for(int i=0;i<13;i++)if(i!=4 && (!Float.isFinite(v[i])||v[i]<0||v[i]>1))return;
            left=v[0];top=v[1];right=v[2];bottom=v[3];color=Integer.parseInt(p[4]);joystickX=v[5];joystickY=v[6];attackX=v[7];attackY=v[8];skillX=v[9];skillY=v[10];aimX=v[11];aimY=v[12];
            calibrated=right-left>=.05f && bottom-top>=.05f;
        }catch(Exception ignored){calibrated=false;}
    }
}
