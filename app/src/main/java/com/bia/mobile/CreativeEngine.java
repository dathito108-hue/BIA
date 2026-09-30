package com.bia.mobile;

import android.graphics.*;
import org.json.*;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;
import java.util.zip.*;

/** Bounded procedural graphics skill. No trained image model, network or imported code. */
public final class CreativeEngine {
    public static final class Spec {
        public final String kind; public final long seed; public final int detail,color; public final double height;
        public Spec(String k,long s,int d,int c,double h){
            if(!Arrays.asList("mandala","landscape","box","sphere","vase").contains(k)||d<8||d>48||!Double.isFinite(h)||h<0.5||h>3)throw new IllegalArgumentException("Thông số ngoài giới hạn");
            kind=k;seed=s;detail=d;color=c|0xff000000;height=h;
        }
        public boolean mesh(){return !kind.equals("mandala")&&!kind.equals("landscape");}
        public String json()throws JSONException{return new JSONObject().put("schema","BIA_CREATIVE_1").put("kind",kind).put("seed",Long.toString(seed)).put("detail",detail).put("color",color).put("height",height).toString(2);}
        public static Spec parse(String text)throws JSONException{if(text.length()>4096)throw new IllegalArgumentException("Tệp quá lớn");JSONObject j=new JSONObject(text);if(!"BIA_CREATIVE_1".equals(j.getString("schema")))throw new IllegalArgumentException("Sai phiên bản");return new Spec(j.getString("kind"),Long.parseLong(j.getString("seed")),j.getInt("detail"),j.getInt("color"),j.getDouble("height"));}
    }
    public static final class Mesh {
        public final ArrayList<double[]> vertices=new ArrayList<>(); public final ArrayList<int[]> faces=new ArrayList<>();
        int v(double x,double y,double z){vertices.add(new double[]{x,y,z});return vertices.size()-1;}
        void f(int a,int b,int c){faces.add(new int[]{a,b,c});}
        public String obj(){StringBuilder b=new StringBuilder("# BIA V140 procedural mesh; units arbitrary; Y up\no BIA\n");for(double[] v:vertices)b.append(String.format(Locale.ROOT,"v %.7f %.7f %.7f\n",v[0],v[1],v[2]));for(int[] f:faces)b.append("f ").append(f[0]+1).append(' ').append(f[1]+1).append(' ').append(f[2]+1).append('\n');return b.toString();}
    }
    public static Mesh mesh(Spec s){
        if(!s.mesh())throw new IllegalArgumentException("Không phải mô hình 3D");Mesh m=new Mesh();
        if(s.kind.equals("box")){
            for(int y=0;y<2;y++)for(int z=0;z<2;z++)for(int x=0;x<2;x++)m.v(x*2-1,(y*2-1)*s.height,z*2-1);
            int[][] fs={{0,1,3},{0,3,2},{4,6,7},{4,7,5},{0,4,5},{0,5,1},{2,3,7},{2,7,6},{0,2,6},{0,6,4},{1,5,7},{1,7,3}};for(int[] f:fs)m.f(f[0],f[1],f[2]);return m;
        }
        int n=s.detail,rings=s.kind.equals("sphere")?n-1:n+1;
        for(int j=0;j<rings;j++){
            double t=s.kind.equals("sphere")?(j+1.0)/n:j/(double)n;
            double y=s.kind.equals("sphere")?-Math.cos(Math.PI*t)*s.height:(2*t-1)*s.height;
            double r=s.kind.equals("sphere")?Math.sin(Math.PI*t):0.55+0.25*Math.sin(t*Math.PI*2)+0.10*Math.cos(t*Math.PI*4+(s.seed%360)*Math.PI/180);
            for(int i=0;i<n;i++){double a=i*2*Math.PI/n;m.v(r*Math.cos(a),y,r*Math.sin(a));}
        }
        for(int j=0;j<rings-1;j++)for(int i=0;i<n;i++){int a=j*n+i,b=j*n+(i+1)%n,c=b+n,d=a+n;m.f(a,d,c);m.f(a,c,b);}
        int bottom=m.v(0,-s.height,0),top=m.v(0,s.height,0);
        for(int i=0;i<n;i++){int a=i,b=(i+1)%n;m.f(bottom,a,b);int c=(rings-1)*n+i,d=(rings-1)*n+(i+1)%n;m.f(top,d,c);}return m;
    }
    static final class Shape {final int color;final float[] xy;Shape(int c,float...p){color=c;xy=p;}}
    static ArrayList<Shape> shapes(Spec s){
        ArrayList<Shape> out=new ArrayList<>();Random r=new Random(s.seed);
        if(s.kind.equals("mandala")){
            for(int layer=4;layer>=1;layer--)for(int i=0;i<s.detail;i++){
                double a=2*Math.PI*i/s.detail+layer*0.13+(s.seed%360)*Math.PI/180;float rad=layer*95;float[] p=new float[8];
                double[] aa={a,a+Math.PI/s.detail,a,a-Math.PI/s.detail};float[] rr={rad+45,rad-15,rad-65,rad-15};
                for(int k=0;k<4;k++){p[k*2]=512+(float)Math.cos(aa[k])*rr[k];p[k*2+1]=512+(float)Math.sin(aa[k])*rr[k];}out.add(new Shape(tint(s.color,0.45+layer*0.12),p));
            }
        }else{
            float cx=150+r.nextInt(700),cy=100+r.nextInt(180);float[] sun=new float[64];for(int i=0;i<32;i++){sun[i*2]=cx+(float)Math.cos(i*Math.PI/16)*65;sun[i*2+1]=cy+(float)Math.sin(i*Math.PI/16)*65;}out.add(new Shape(0xffffcc73,sun));
            for(int layer=0;layer<5;layer++){float[] p=new float[(s.detail+3)*2];p[0]=0;p[1]=1024;for(int i=0;i<=s.detail;i++){p[(i+1)*2]=1024f*i/s.detail;p[(i+1)*2+1]=320+layer*115+r.nextInt(170);}p[p.length-2]=1024;p[p.length-1]=1024;out.add(new Shape(tint(s.color,0.55+layer*0.12),p));}
        }return out;
    }
    static int tint(int c,double f){return Color.rgb(Math.min(255,(int)(Color.red(c)*f)),Math.min(255,(int)(Color.green(c)*f)),Math.min(255,(int)(Color.blue(c)*f)));}
    public static String svg(Spec s){if(s.mesh())throw new IllegalArgumentException("SVG chỉ dành cho ảnh 2D");StringBuilder b=new StringBuilder("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1024\" height=\"1024\" viewBox=\"0 0 1024 1024\"><rect width=\"1024\" height=\"1024\" fill=\"#101c30\"/>");for(Shape sh:shapes(s)){b.append(String.format(Locale.ROOT,"<polygon fill=\"#%06x\" points=\"",sh.color&0xffffff));for(int i=0;i<sh.xy.length;i+=2)b.append(String.format(Locale.ROOT,"%.3f,%.3f ",sh.xy[i],sh.xy[i+1]));b.append("\"/>");}return b.append("</svg>").toString();}
    public static Bitmap render(Spec s,int size,double yaw,double pitch){if(size<64||size>2048||!Double.isFinite(yaw)||!Double.isFinite(pitch))throw new IllegalArgumentException("Kích thước/góc không hợp lệ");Bitmap b=Bitmap.createBitmap(size,size,Bitmap.Config.ARGB_8888);Canvas c=new Canvas(b);draw(c,size,size,s,yaw,pitch);return b;}
    public static void draw(Canvas c,int w,int h,Spec s,double yaw,double pitch){c.drawColor(0xff101c30);Paint p=new Paint(Paint.ANTI_ALIAS_FLAG);c.save();c.translate((w-Math.min(w,h))/2f,(h-Math.min(w,h))/2f);c.scale(Math.min(w,h)/1024f,Math.min(w,h)/1024f);
        if(!s.mesh()){for(Shape sh:shapes(s)){p.setColor(sh.color);polygon(c,p,sh.xy);}}
        else{Mesh m=mesh(s);ArrayList<double[]> projected=new ArrayList<>();for(double[] v:m.vertices){double x=v[0]*Math.cos(yaw)+v[2]*Math.sin(yaw),z=-v[0]*Math.sin(yaw)+v[2]*Math.cos(yaw);double y=v[1]*Math.cos(pitch)-z*Math.sin(pitch);z=v[1]*Math.sin(pitch)+z*Math.cos(pitch);projected.add(new double[]{512+x*190/Math.max(1,s.height),512-y*190/Math.max(1,s.height),z});}
            ArrayList<int[]> ordered=new ArrayList<>(m.faces);ordered.sort(Comparator.comparingDouble(f->projected.get(f[0])[2]+projected.get(f[1])[2]+projected.get(f[2])[2]));
            for(int[] f:ordered){double[] a=projected.get(f[0]),b=projected.get(f[1]),d=projected.get(f[2]);double area=(b[0]-a[0])*(d[1]-a[1])-(b[1]-a[1])*(d[0]-a[0]);if(area>=0)continue;p.setColor(tint(s.color,0.5+0.45*Math.min(1,Math.abs(area)/7000)));polygon(c,p,new float[]{(float)a[0],(float)a[1],(float)b[0],(float)b[1],(float)d[0],(float)d[1]});}
        }c.restore();
    }
    static void polygon(Canvas c,Paint p,float[] xy){Path path=new Path();path.moveTo(xy[0],xy[1]);for(int i=2;i<xy.length;i+=2)path.lineTo(xy[i],xy[i+1]);path.close();c.drawPath(path,p);}
    public static void zip(Spec s,OutputStream output)throws Exception{
        try(ZipOutputStream z=new ZipOutputStream(output)){entry(z,"project.bia-art.json",s.json().getBytes(StandardCharsets.UTF_8));entry(z,s.mesh()?"model.obj":"image.svg",(s.mesh()?mesh(s).obj():svg(s)).getBytes(StandardCharsets.UTF_8));Bitmap b=render(s,1024,0.65,0.35);try{z.putNextEntry(new ZipEntry("preview.png"));if(!b.compress(Bitmap.CompressFormat.PNG,100,z))throw new IOException("PNG thất bại");z.closeEntry();}finally{b.recycle();}entry(z,"README.txt",("BIA V140 — đồ họa thủ tục offline.\nPNG 1024×1024; SVG vector hoặc OBJ tam giác, Y hướng lên, đơn vị tùy chọn.\nOBJ chỉ có hình học, không texture/rig/animation. Bình là khối kín trang trí, không có lòng rỗng.\nproject.bia-art.json lưu tham số để mở lại trong BIA.\nKhông phải mô hình tạo ảnh học máy hoặc tái dựng 3D từ ảnh. Kiểm tra trước khi sản xuất/in 3D.\n").getBytes(StandardCharsets.UTF_8));}
    }
    static void entry(ZipOutputStream z,String name,byte[] bytes)throws IOException{z.putNextEntry(new ZipEntry(name));z.write(bytes);z.closeEntry();}
}
