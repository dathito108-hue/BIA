package com.bia.mobile;

import android.test.InstrumentationTestCase;
import android.content.*;
import android.graphics.*;
import java.io.*;
import java.util.*;
import java.util.zip.*;

public final class SceneStudioTest extends InstrumentationTestCase {
    public void testPrimitivesTransformsAndResourceBounds()throws Exception{
        for(String kind:SceneEngine.KINDS)for(int detail:new int[]{8,48}){
            SceneEngine.Item item=new SceneEngine.Item(kind,detail,0xff52bea8,42,2,3,4,0.5,1.2,0.8,22,37,49,0.4,true);CreativeEngine.Mesh m=SceneEngine.world(item);assertEquals(item.triangles(),m.faces.size());HashMap<String,Integer> count=new HashMap<>(),direction=new HashMap<>();double volume=0;
            for(double[] v:m.vertices)for(double x:v)assertTrue(Double.isFinite(x));
            for(int[] f:m.faces){double[] a=m.vertices.get(f[0]),b=m.vertices.get(f[1]),c=m.vertices.get(f[2]),n=SceneEngine.normal(a,b,c);assertTrue(n[0]*n[0]+n[1]*n[1]+n[2]*n[2]>1e-15);volume+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);for(int i=0;i<3;i++){int u=f[i],v=f[(i+1)%3];String k=Math.min(u,v)+":"+Math.max(u,v);count.put(k,count.getOrDefault(k,0)+1);direction.put(k,direction.getOrDefault(k,0)+(u<v?1:-1));}}
            for(String k:count.keySet()){assertEquals(kind,2,(int)count.get(k));assertEquals(kind,0,(int)direction.get(k));}assertEquals(kind,kind.equals("torus")?0:2,m.vertices.size()-count.size()+m.faces.size());assertTrue(kind,volume>0);
        }
        ArrayList<SceneEngine.Item> items=new ArrayList<>();for(int i=0;i<13;i++)items.add(SceneEngine.item("box",0xffaaaaaa,0,0,0,1));try{new SceneEngine.Scene(items,0,0,1,0,512,false);fail("Object cap");}catch(IllegalArgumentException expected){}
        items.clear();for(int i=0;i<7;i++)items.add(new SceneEngine.Item("torus",48,0,0,0,0,0,1,1,1,0,0,0,0,true));try{new SceneEngine.Scene(items,0,0,1,0,512,false);fail("Triangle cap");}catch(IllegalArgumentException expected){}
        try{SceneEngine.item("sphere",0,0,0,0,-1);fail("Negative scale");}catch(IllegalArgumentException expected){}
        try{SceneEngine.sample().camera(Double.NaN,0,1);fail("NaN camera");}catch(IllegalArgumentException expected){}
        SceneEngine.Scene legacy=SceneEngine.Scene.parse(new CreativeEngine.Spec("vase",12,24,0xff112233,2).json());assertEquals(2.0,legacy.items.get(0).sy,0.0);assertEquals(legacy.json(),SceneEngine.Scene.parse(legacy.json()).json());
    }
    public void testDepthTransparencyLightingAndCamera()throws Exception{
        SceneEngine.Item front=SceneEngine.item("box",0xffff0000,0,0,1,0.8),back=SceneEngine.item("box",0xff0000ff,0,0,-1,0.8);
        SceneEngine.Scene a=new SceneEngine.Scene(Arrays.asList(front,back),0,0,1,0,512,true),b=new SceneEngine.Scene(Arrays.asList(back,front),0,0,1,0,512,true);
        Bitmap first=SceneEngine.render(a,256),second=SceneEngine.render(b,256);assertTrue("Depth independent of object order",first.sameAs(second));assertEquals(0,Color.alpha(first.getPixel(0,0)));assertTrue(Color.red(first.getPixel(128,128))>Color.blue(first.getPixel(128,128)));first.recycle();second.recycle();
        SceneEngine.Scene s=SceneEngine.sample();Bitmap base=SceneEngine.render(s,256),rotated=SceneEngine.render(s.camera(1.5,0.2,1),256),lit=SceneEngine.render(new SceneEngine.Scene(s.items,s.yaw,s.pitch,1,2.1,512,false),256);assertFalse(base.sameAs(rotated));assertFalse(base.sameAs(lit));base.recycle();rotated.recycle();lit.recycle();
    }
    public void testSceneExportsNormalsMaterialsAnd2kPng()throws Exception{
        SceneEngine.Scene source=SceneEngine.sample(),s=new SceneEngine.Scene(source.items,source.yaw,source.pitch,1,source.light,2048,true);ByteArrayOutputStream bytes=new ByteArrayOutputStream();SceneEngine.zip(s,bytes);HashMap<String,byte[]> files=unzip(bytes.toByteArray());assertEquals(6,files.size());assertEquals(s.json(),new String(files.get("scene.bia-scene.json"),"UTF-8"));String obj=new String(files.get("scene.obj"),"UTF-8"),mtl=new String(files.get("materials.mtl"),"UTF-8"),stl=new String(files.get("scene.stl"),"UTF-8");int vertices=0,normals=0,faces=0,objects=0;
        for(String line:obj.split("\n")){if(line.startsWith("v "))vertices++;if(line.startsWith("vn "))normals++;if(line.startsWith("o "))objects++;}
        for(String line:obj.split("\n"))if(line.startsWith("f ")){faces++;for(String entry:line.substring(2).split(" ")){String[] refs=entry.split("//");assertEquals(2,refs.length);assertTrue(Integer.parseInt(refs[0])>0&&Integer.parseInt(refs[0])<=vertices);assertTrue(Integer.parseInt(refs[1])>0&&Integer.parseInt(refs[1])<=normals);}}
        int expected=0;for(SceneEngine.Item item:s.items)expected+=item.triangles();assertEquals(expected,faces);assertEquals(4,objects);assertEquals(4,mtl.split("newmtl ").length-1);assertEquals(expected,stl.split("facet normal ").length-1);
        byte[] png=files.get("render.png");Bitmap image=BitmapFactory.decodeByteArray(png,0,png.length);assertNotNull(image);assertEquals(2048,image.getWidth());assertEquals(0,Color.alpha(image.getPixel(0,0)));image.recycle();
        try(FileOutputStream f=new FileOutputStream(new File(getInstrumentation().getTargetContext().getExternalFilesDir(null),"creative-scene-v141.zip"))){f.write(bytes.toByteArray());}
    }
    HashMap<String,byte[]> unzip(byte[] bytes)throws IOException{HashMap<String,byte[]> result=new HashMap<>();try(ZipInputStream z=new ZipInputStream(new ByteArrayInputStream(bytes))){for(ZipEntry e;(e=z.getNextEntry())!=null;){ByteArrayOutputStream body=new ByteArrayOutputStream();byte[] b=new byte[4096];for(int n;(n=z.read(b))!=-1;)body.write(b,0,n);result.put(e.getName(),body.toByteArray());}}return result;}
    public void testImageVariantsTransparent2kAndSvg()throws Exception{
        for(String kind:new String[]{"mandala","waves","mosaic"}){CreativeEngine.Spec s=new CreativeEngine.Spec(kind,42,16,0xff658ee8,1);String svg=CreativeEngine.svg(s,2048,true);assertTrue(svg.contains("width=\"2048\""));assertFalse(svg.contains("<rect"));org.xmlpull.v1.XmlPullParser parser=android.util.Xml.newPullParser();parser.setInput(new StringReader(svg));while(parser.next()!=org.xmlpull.v1.XmlPullParser.END_DOCUMENT){}
            Bitmap a=CreativeEngine.render(s,256,0,0,true),b=CreativeEngine.render(new CreativeEngine.Spec(kind,71,16,s.color,1),256,0,0,true);assertFalse("Seed changes "+kind,a.sameAs(b));if(kind.equals("mandala"))assertEquals(0,Color.alpha(a.getPixel(0,0)));a.recycle();b.recycle();
        }
        CreativeEngine.Spec s=new CreativeEngine.Spec("waves",42,24,0xff648dda,1);try(FileOutputStream f=new FileOutputStream(new File(getInstrumentation().getTargetContext().getExternalFilesDir(null),"creative-image-v141.zip"))){CreativeEngine.zip(s,f,2048,true,0,0);}
    }
    public void testEditorUndoRedoDurabilityAndDocumentIo()throws Exception{
        Context c=getInstrumentation().getTargetContext();SceneActivity a=(SceneActivity)getInstrumentation().startActivitySync(new Intent(c,SceneActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));File input=new File(c.getCacheDir(),"scene-import.json"),output=new File(c.getCacheDir(),"scene-export.zip");
        try{
            getInstrumentation().runOnMainSync(()->{a.change(SceneEngine.sample());a.selected=0;a.fillItem();a.position.setText("-2,0,0");a.apply.performClick();assertEquals(-2.0,a.scene.items.get(0).x,0.0);a.undo.performClick();assertEquals(0.0,a.scene.items.get(0).x,0.0);a.redo.performClick();assertEquals(-2.0,a.scene.items.get(0).x,0.0);a.duplicate.performClick();assertEquals(5,a.scene.items.size());a.remove.performClick();assertEquals(4,a.scene.items.size());a.scale.setText("-1,1,1");a.apply.performClick();assertTrue(a.status.getText().toString().contains("Không thay đổi"));a.fillItem();});
            try(FileOutputStream f=new FileOutputStream(input)){f.write(SceneEngine.sample().json().getBytes("UTF-8"));}
            getInstrumentation().runOnMainSync(()->a.onActivityResult(2,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(input))));waitWork(a);assertEquals(SceneEngine.sample().json(),a.scene.json());
            getInstrumentation().runOnMainSync(()->{a.pending=new SceneEngine.Scene(a.scene.items,0.2,0.3,1,-0.4,512,true);a.onActivityResult(1,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(output)));});waitWork(a);assertTrue(a.status.getText().toString(),a.status.getText().toString().contains("Đã xuất"));try(ZipFile zip=new ZipFile(output)){assertNotNull(zip.getEntry("scene.obj"));}
            try(FileOutputStream f=new FileOutputStream(input)){f.write(new byte[65537]);}getInstrumentation().runOnMainSync(()->a.onActivityResult(2,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(input))));waitWork(a);assertTrue(a.status.getText().toString().contains("Không hoàn thành"));assertEquals(4,a.scene.items.size());
        }finally{input.delete();output.delete();getInstrumentation().runOnMainSync(a::finish);}
        getInstrumentation().waitForIdleSync();SceneActivity b=(SceneActivity)getInstrumentation().startActivitySync(new Intent(c,SceneActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));try{assertEquals(SceneEngine.sample().json(),b.scene.json());}finally{getInstrumentation().runOnMainSync(b::finish);}
    }
    void waitWork(SceneActivity a){long until=android.os.SystemClock.elapsedRealtime()+20000;while(a.busy&&android.os.SystemClock.elapsedRealtime()<until)android.os.SystemClock.sleep(50);getInstrumentation().waitForIdleSync();assertFalse("Scene IO timeout",a.busy);}
}
