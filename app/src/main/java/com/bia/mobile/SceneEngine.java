package com.bia.mobile;

import android.graphics.Bitmap;
import android.graphics.Color;
import org.json.*;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;
import java.util.zip.*;

/** Bounded offline scene compiler and orthographic depth-buffer renderer. */
public final class SceneEngine {
    public static final String[] KINDS={"box","sphere","vase","cylinder","cone","torus"};
    public static final int MAX_OBJECTS=12, MAX_TRIANGLES=30000, MAX_JSON=65536;
    public static final class Item {
        public final String kind; public final int detail,color; public final long seed;
        public final List<Sculpt.Stroke> strokes;public final int subdivisions;
        public final double x,y,z,sx,sy,sz,rx,ry,rz,gloss; public final boolean smooth;
        public Item(String kind,int detail,int color,long seed,double x,double y,double z,double sx,double sy,double sz,double rx,double ry,double rz,double gloss,boolean smooth){
            this(kind,detail,color,seed,x,y,z,sx,sy,sz,rx,ry,rz,gloss,smooth,Collections.emptyList(),0);
        }
        private Item(String kind,int detail,int color,long seed,double x,double y,double z,double sx,double sy,double sz,double rx,double ry,double rz,double gloss,boolean smooth,List<Sculpt.Stroke> strokes,int subdivisions){
            if(strokes==null||strokes.size()>24||subdivisions<0||subdivisions>2||strokes.contains(null))throw new IllegalArgumentException("Tối đa 24 nét cọ và 2 lần chia lưới");this.strokes=Collections.unmodifiableList(new ArrayList<>(strokes));this.subdivisions=subdivisions;
            if(!Arrays.asList(KINDS).contains(kind)||detail<8||detail>48)throw new IllegalArgumentException("Hình hoặc độ chi tiết không hợp lệ");
            for(double v:new double[]{x,y,z})range(v,-20,20);for(double v:new double[]{sx,sy,sz})range(v,0.05,5);for(double v:new double[]{rx,ry,rz})range(v,-360,360);range(gloss,0,1);
            this.kind=kind;this.detail=detail;this.color=color|0xff000000;this.seed=seed;this.x=x;this.y=y;this.z=z;this.sx=sx;this.sy=sy;this.sz=sz;this.rx=rx;this.ry=ry;this.rz=rz;this.gloss=gloss;this.smooth=smooth;
        }
        public Item geometry(List<Sculpt.Stroke> strokes,int divisions){return new Item(kind,detail,color,seed,x,y,z,sx,sy,sz,rx,ry,rz,gloss,smooth,strokes,divisions);}
        public int triangles(){return baseTriangles()*(1<<(subdivisions*2));}
        private int baseTriangles(){switch(kind){case "box":return 12;case "cylinder":return 4*detail;case "cone":return 2*detail;case "torus":return 2*detail*detail;case "sphere":return 2*detail*(detail-1);default:return 2*detail*(detail+1);}}
        JSONObject json()throws JSONException{JSONArray edits=new JSONArray();for(Sculpt.Stroke stroke:strokes)edits.put(stroke.json());return new JSONObject().put("strokes",edits).put("subdivisions",subdivisions).put("kind",kind).put("detail",detail).put("color",color).put("seed",Long.toString(seed)).put("position",new JSONArray(new double[]{x,y,z})).put("scale",new JSONArray(new double[]{sx,sy,sz})).put("rotation",new JSONArray(new double[]{rx,ry,rz})).put("gloss",gloss).put("smooth",smooth);}
        static Item parse(JSONObject j)throws JSONException{JSONArray p=j.getJSONArray("position"),s=j.getJSONArray("scale"),r=j.getJSONArray("rotation");if(p.length()!=3||s.length()!=3||r.length()!=3)throw new IllegalArgumentException("Cần ba trục");JSONArray edits=j.optJSONArray("strokes");ArrayList<Sculpt.Stroke> strokes=new ArrayList<>();if(edits!=null){if(edits.length()>24)throw new IllegalArgumentException("Quá nhiều nét cọ");for(int i=0;i<edits.length();i++)strokes.add(Sculpt.Stroke.parse(edits.getJSONObject(i)));}return new Item(j.getString("kind"),integer(j,"detail"),integer(j,"color"),Long.parseLong(j.getString("seed")),p.getDouble(0),p.getDouble(1),p.getDouble(2),s.getDouble(0),s.getDouble(1),s.getDouble(2),r.getDouble(0),r.getDouble(1),r.getDouble(2),j.getDouble("gloss"),j.getBoolean("smooth"),strokes,j.has("subdivisions")?integer(j,"subdivisions"):0);}
    }
    public static final class Scene {
        public final List<Item> items; public final double yaw,pitch,zoom,light;public final int size;public final boolean transparent;
        public Scene(List<Item> items,double yaw,double pitch,double zoom,double light,int size,boolean transparent){
            if(items==null||items.isEmpty()||items.size()>MAX_OBJECTS)throw new IllegalArgumentException("Cảnh cần 1–12 vật thể");int faces=0,edits=0;for(Item i:items){if(i==null)throw new IllegalArgumentException("Vật thể trống");faces+=i.triangles();edits+=i.strokes.size();}if(edits>48)throw new IllegalArgumentException("Tối đa 48 nét cọ trong một cảnh");if(faces>MAX_TRIANGLES)throw new IllegalArgumentException("Cảnh vượt 30.000 tam giác; giảm độ chi tiết");
            range(yaw,-Math.PI,Math.PI);range(pitch,-1.5,1.5);range(zoom,0.25,3);range(light,-Math.PI,Math.PI);if(size!=512&&size!=1024&&size!=2048)throw new IllegalArgumentException("Ảnh cần 512, 1024 hoặc 2048 px");
            this.items=Collections.unmodifiableList(new ArrayList<>(items));this.yaw=yaw;this.pitch=pitch;this.zoom=zoom;this.light=light;this.size=size;this.transparent=transparent;
        }
        public Scene camera(double yaw,double pitch,double zoom){return new Scene(items,yaw,pitch,zoom,light,size,transparent);}
        public String json()throws JSONException{JSONArray a=new JSONArray();for(Item i:items)a.put(i.json());return new JSONObject().put("schema","BIA_SCENE_2").put("objects",a).put("yaw",yaw).put("pitch",pitch).put("zoom",zoom).put("light",light).put("size",size).put("transparent",transparent).toString(2);}
        public static Scene parse(String text)throws JSONException{
            if(text.length()>MAX_JSON)throw new IllegalArgumentException("Tệp cảnh quá lớn");JSONObject j=new JSONObject(text);String schema=j.getString("schema");
            if(schema.equals("BIA_CREATIVE_1")){CreativeEngine.Spec old=CreativeEngine.Spec.parse(text);if(!old.mesh())throw new IllegalArgumentException("Mở ảnh 2D trong Xưởng ảnh");return new Scene(Collections.singletonList(new Item(old.kind,old.detail,old.color,old.seed,0,0,0,1,old.height,1,0,0,0,0.3,true)),0.65,0.35,1,-0.7,1024,false);}
            if(!schema.equals("BIA_SCENE_1")&&!schema.equals("BIA_SCENE_2"))throw new IllegalArgumentException("Sai phiên bản cảnh");JSONArray a=j.getJSONArray("objects");if(a.length()>MAX_OBJECTS)throw new IllegalArgumentException("Quá nhiều vật thể");ArrayList<Item> items=new ArrayList<>();for(int i=0;i<a.length();i++)items.add(Item.parse(a.getJSONObject(i)));return new Scene(items,j.getDouble("yaw"),j.getDouble("pitch"),j.getDouble("zoom"),j.getDouble("light"),integer(j,"size"),j.getBoolean("transparent"));
        }
    }
    static void range(double v,double lo,double hi){if(!Double.isFinite(v)||v<lo||v>hi)throw new IllegalArgumentException("Thông số phải nằm trong "+lo+"…"+hi);}
    static int integer(JSONObject j,String k)throws JSONException{double d=j.getDouble(k);if(!Double.isFinite(d)||d!=Math.rint(d)||d<Integer.MIN_VALUE||d>Integer.MAX_VALUE)throw new IllegalArgumentException("Cần số nguyên: "+k);return (int)d;}
    public static Item item(String kind,int color,double x,double y,double z,double scale){return new Item(kind,24,color,42,x,y,z,scale,scale,scale,0,0,0,0.45,true);}
    public static Scene sample(){return new Scene(Arrays.asList(new Item("cylinder",32,0xff244c65,0,0,-1.2,0,2.4,0.18,2.4,0,0,0,0.2,false),item("vase",0xff38b9a0,-1,0,0,0.85),new Item("torus",32,0xffefb955,0,1,0,0,0.85,0.85,0.85,65,20,0,0.8,true),item("sphere",0xffa184e8,0,1,0,0.48)),0.6,0.3,1,-0.8,1024,false);}
    public static CreativeEngine.Mesh primitive(Item item){return Sculpt.apply(rawPrimitive(item),item.strokes,item.subdivisions);}
    private static CreativeEngine.Mesh rawPrimitive(Item item){
        int n=item.detail;String kind=item.kind;
        if(Arrays.asList("box","sphere","vase").contains(kind))return CreativeEngine.mesh(new CreativeEngine.Spec(kind,item.seed,n,item.color,1));
        CreativeEngine.Mesh m=new CreativeEngine.Mesh();
        if(kind.equals("torus")){
            for(int i=0;i<n;i++)for(int j=0;j<n;j++){double u=i*2*Math.PI/n,v=j*2*Math.PI/n,r=0.78+0.28*Math.cos(v);m.v(r*Math.cos(u),0.28*Math.sin(v),r*Math.sin(u));}
            for(int i=0;i<n;i++)for(int j=0;j<n;j++){int a=i*n+j,b=((i+1)%n)*n+j,c=((i+1)%n)*n+(j+1)%n,d=i*n+(j+1)%n;m.f(a,d,c);m.f(a,c,b);}return m;
        }
        for(int i=0;i<n;i++){double a=2*Math.PI*i/n;m.v(Math.cos(a),-1,Math.sin(a));}
        if(kind.equals("cylinder")){for(int i=0;i<n;i++){double a=2*Math.PI*i/n;m.v(Math.cos(a),1,Math.sin(a));}for(int i=0;i<n;i++){int j=(i+1)%n;m.f(i,i+n,j+n);m.f(i,j+n,j);}}
        int bottom=m.v(0,-1,0),top=m.v(0,1,0);for(int i=0;i<n;i++){int j=(i+1)%n;m.f(bottom,i,j);if(kind.equals("cylinder"))m.f(top,j+n,i+n);else m.f(top,j,i);}return m;
    }
    static double[] rotate(double[] v,double rx,double ry,double rz){double x=v[0],y=v[1]*Math.cos(rx)-v[2]*Math.sin(rx),z=v[1]*Math.sin(rx)+v[2]*Math.cos(rx);double xx=x*Math.cos(ry)+z*Math.sin(ry);z=-x*Math.sin(ry)+z*Math.cos(ry);x=xx;return new double[]{x*Math.cos(rz)-y*Math.sin(rz),x*Math.sin(rz)+y*Math.cos(rz),z};}
    public static CreativeEngine.Mesh world(Item item){CreativeEngine.Mesh m=primitive(item);for(int i=0;i<m.vertices.size();i++){double[] v=m.vertices.get(i),t=rotate(new double[]{v[0]*item.sx,v[1]*item.sy,v[2]*item.sz},Math.toRadians(item.rx),Math.toRadians(item.ry),Math.toRadians(item.rz));m.vertices.set(i,new double[]{t[0]+item.x,t[1]+item.y,t[2]+item.z});}return m;}
    static double[] normal(double[] a,double[] b,double[] c){double x=b[0]-a[0],y=b[1]-a[1],z=b[2]-a[2],u=c[0]-a[0],v=c[1]-a[1],w=c[2]-a[2];return new double[]{y*w-z*v,z*u-x*w,x*v-y*u};}
    static double[] unit(double[] a){double l=Math.sqrt(a[0]*a[0]+a[1]*a[1]+a[2]*a[2]);return l>1e-20?new double[]{a[0]/l,a[1]/l,a[2]/l}:new double[]{0,1,0};}
    static final class Part {final Item item;final CreativeEngine.Mesh mesh;final double[][] normals;Part(Item i){item=i;mesh=world(i);normals=new double[mesh.vertices.size()][3];for(int[] f:mesh.faces){double[] n=normal(mesh.vertices.get(f[0]),mesh.vertices.get(f[1]),mesh.vertices.get(f[2]));for(int v:f)for(int k=0;k<3;k++)normals[v][k]+=n[k];}for(int j=0;j<normals.length;j++)normals[j]=unit(normals[j]);}}
    static double[] camera(double[] v,Scene s){double[] y=rotate(v,0,s.yaw,0);return rotate(y,s.pitch,0,0);}
    static double lighting(double[] n,double[] light,double gloss){n=unit(n);double diffuse=Math.max(0,n[0]*light[0]+n[1]*light[1]+n[2]*light[2]);double[] half=unit(new double[]{light[0],light[1],light[2]+1});double spec=Math.pow(Math.max(0,n[0]*half[0]+n[1]*half[1]+n[2]*half[2]),8+gloss*88)*gloss;return Math.min(1.35,0.2+0.8*diffuse+0.5*spec);}
    static int shade(int color,double intensity){return Color.rgb(Math.min(255,(int)(Color.red(color)*intensity)),Math.min(255,(int)(Color.green(color)*intensity)),Math.min(255,(int)(Color.blue(color)*intensity)));}
    public static Bitmap render(Scene scene,int size)throws InterruptedIOException{return render(scene,size,null);}
    public static Bitmap render(Scene scene,int size,Scene framing)throws InterruptedIOException{
        if(size<64||size>2048)throw new IllegalArgumentException("Ảnh tối đa 2048px");ArrayList<Part> parts=new ArrayList<>();double[] lo={Double.POSITIVE_INFINITY,Double.POSITIVE_INFINITY,Double.POSITIVE_INFINITY},hi={-Double.MAX_VALUE,-Double.MAX_VALUE,-Double.MAX_VALUE};
        for(Item i:scene.items){Part p=new Part(i);parts.add(p);for(double[] v:p.mesh.vertices)for(int k=0;k<3;k++){lo[k]=Math.min(lo[k],v[k]);hi[k]=Math.max(hi[k],v[k]);}}
        ArrayList<Part> bounds=parts;if(framing!=null){bounds=new ArrayList<>();Arrays.fill(lo,Double.POSITIVE_INFINITY);Arrays.fill(hi,-Double.MAX_VALUE);for(Item i:framing.items){Part p=new Part(i);bounds.add(p);for(double[] v:p.mesh.vertices)for(int k=0;k<3;k++){lo[k]=Math.min(lo[k],v[k]);hi[k]=Math.max(hi[k],v[k]);}}}
        double[] center={(lo[0]+hi[0])/2,(lo[1]+hi[1])/2,(lo[2]+hi[2])/2};double radius=0;for(Part p:bounds)for(double[] v:p.mesh.vertices){double dx=v[0]-center[0],dy=v[1]-center[1],dz=v[2]-center[2];radius=Math.max(radius,Math.sqrt(dx*dx+dy*dy+dz*dz));}double scale=size*0.43*scene.zoom/Math.max(radius,0.001);
        int[] pixels=new int[size*size];if(!scene.transparent)Arrays.fill(pixels,0xff101c30);float[] depth=new float[pixels.length];Arrays.fill(depth,Float.NEGATIVE_INFINITY);double[] light=unit(new double[]{Math.sin(scene.light),0.75,Math.cos(scene.light)});
        for(Part p:parts){double[][] verts=new double[p.mesh.vertices.size()][3];double[] intensities=new double[verts.length];for(int i=0;i<verts.length;i++){double[] v=p.mesh.vertices.get(i),r=camera(new double[]{v[0]-center[0],v[1]-center[1],v[2]-center[2]},scene);verts[i]=new double[]{size*0.5+r[0]*scale,size*0.5-r[1]*scale,r[2]};intensities[i]=lighting(camera(p.normals[i],scene),light,p.item.gloss);}
            for(int[] f:p.mesh.faces){if(Thread.currentThread().isInterrupted())throw new InterruptedIOException("Đã dừng dựng ảnh");double[] a=verts[f[0]],b=verts[f[1]],c=verts[f[2]];double area=(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);if(area>=-1e-10)continue;int minX=Math.max(0,(int)Math.floor(Math.min(a[0],Math.min(b[0],c[0])))),maxX=Math.min(size-1,(int)Math.ceil(Math.max(a[0],Math.max(b[0],c[0])))),minY=Math.max(0,(int)Math.floor(Math.min(a[1],Math.min(b[1],c[1])))),maxY=Math.min(size-1,(int)Math.ceil(Math.max(a[1],Math.max(b[1],c[1]))));
                double flat=lighting(camera(normal(p.mesh.vertices.get(f[0]),p.mesh.vertices.get(f[1]),p.mesh.vertices.get(f[2])),scene),light,p.item.gloss);
                for(int y=minY;y<=maxY;y++)for(int x=minX;x<=maxX;x++){double px=x+0.5,py=y+0.5;double w0=((b[0]-px)*(c[1]-py)-(b[1]-py)*(c[0]-px))/area,w1=((c[0]-px)*(a[1]-py)-(c[1]-py)*(a[0]-px))/area,w2=1-w0-w1;if(w0<-1e-9||w1<-1e-9||w2<-1e-9)continue;double z=w0*a[2]+w1*b[2]+w2*c[2];int index=y*size+x;if(z<=depth[index])continue;depth[index]=(float)z;double value=p.item.smooth&&!p.item.kind.equals("box")?w0*intensities[f[0]]+w1*intensities[f[1]]+w2*intensities[f[2]]:flat;pixels[index]=shade(p.item.color,value);}
            }
        }return Bitmap.createBitmap(pixels,size,size,Bitmap.Config.ARGB_8888);
    }
    public static void zip(Scene scene,OutputStream output)throws Exception{
        try(ZipOutputStream zip=new ZipOutputStream(output)){
            text(zip,"scene.bia-scene.json",scene.json());StringBuilder obj=new StringBuilder("# BIA V141; Y up; units arbitrary\nmtllib materials.mtl\n"),mtl=new StringBuilder(),stl=new StringBuilder("solid BIA\n");int vertexOffset=0,normalOffset=0,index=0;
            for(Item item:scene.items){Part part=new Part(item);CreativeEngine.Mesh m=part.mesh;String name="object_"+(++index);obj.append("o ").append(name).append("\nusemtl ").append(name).append('\n');mtl.append(String.format(Locale.ROOT,"newmtl %s\nKd %.6f %.6f %.6f\nKs %.4f %.4f %.4f\nNs %.3f\nd 1\nillum 2\n\n",name,Color.red(item.color)/255.0,Color.green(item.color)/255.0,Color.blue(item.color)/255.0,item.gloss,item.gloss,item.gloss,8+88*item.gloss));
                for(double[] v:m.vertices)obj.append(String.format(Locale.ROOT,"v %.8f %.8f %.8f\n",v[0],v[1],v[2]));boolean smooth=item.smooth&&!item.kind.equals("box");if(smooth)for(double[] n:part.normals)obj.append(String.format(Locale.ROOT,"vn %.8f %.8f %.8f\n",n[0],n[1],n[2]));else for(int[] f:m.faces){double[] n=unit(normal(m.vertices.get(f[0]),m.vertices.get(f[1]),m.vertices.get(f[2])));obj.append(String.format(Locale.ROOT,"vn %.8f %.8f %.8f\n",n[0],n[1],n[2]));}
                int faceIndex=0;for(int[] f:m.faces){obj.append("f");for(int v:f)obj.append(' ').append(vertexOffset+v+1).append("//").append(normalOffset+(smooth?v:faceIndex)+1);obj.append('\n');double[] n=unit(normal(m.vertices.get(f[0]),m.vertices.get(f[1]),m.vertices.get(f[2])));stl.append(String.format(Locale.ROOT,"facet normal %.8f %.8f %.8f\nouter loop\n",n[0],n[1],n[2]));for(int v:f){double[] p=m.vertices.get(v);stl.append(String.format(Locale.ROOT,"vertex %.8f %.8f %.8f\n",p[0],p[1],p[2]));}stl.append("endloop\nendfacet\n");faceIndex++;}vertexOffset+=m.vertices.size();normalOffset+=smooth?m.vertices.size():m.faces.size();
            }
            stl.append("endsolid BIA\n");text(zip,"scene.obj",obj.toString());text(zip,"materials.mtl",mtl.toString());text(zip,"scene.stl",stl.toString());Bitmap image=render(scene,scene.size);try{zip.putNextEntry(new ZipEntry("render.png"));if(!image.compress(Bitmap.CompressFormat.PNG,100,zip))throw new IOException("Không ghi được PNG");zip.closeEntry();}finally{image.recycle();}
            text(zip,"README.txt","BIA V141 — cảnh 3D thủ tục offline.\nMở scene.obj cùng materials.mtl để giữ màu và vật liệu cơ bản. STL chỉ có hình học. Y hướng lên; đơn vị tùy chọn. PNG dùng góc nhìn và nền đã lưu trong scene.bia-scene.json.\nCác vật thể là các vỏ riêng, có thể giao nhau; không hợp nhất Boolean, không tự bảo đảm in 3D được. Bình trang trí là khối kín.\nÁnh sáng hướng và độ bóng minh họa; không PBR, không bóng đổ, không texture/UV/rig/animation.\nMở scene.bia-scene.json trong Xưởng cảnh để chỉnh tiếp.\n");
        }
    }
    static void text(ZipOutputStream z,String name,String text)throws IOException{CreativeEngine.entry(z,name,text.getBytes(StandardCharsets.UTF_8));}
}
