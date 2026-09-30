package com.bia.mobile;
import android.test.InstrumentationTestCase;
import android.content.*;
import android.graphics.*;
import java.io.*;
import java.util.*;
import java.util.zip.*;

public final class CreativeStudioTest extends InstrumentationTestCase {
    CreativeEngine.Spec spec(String k,int n){return new CreativeEngine.Spec(k,42,n,0xff46bdaa,1.2);}
    public void testClosedMeshTopologyAndFiniteGeometry()throws Exception{
        for(String kind:new String[]{"box","sphere","vase"})for(int n:new int[]{8,24,48}){
            CreativeEngine.Mesh m=CreativeEngine.mesh(spec(kind,n));HashMap<String,Integer> counts=new HashMap<>(),directions=new HashMap<>();double volume=0;
            for(double[] v:m.vertices)for(double x:v)assertTrue(Double.isFinite(x));
            for(int[] f:m.faces){for(int id:f)assertTrue(id>=0&&id<m.vertices.size());double[] a=m.vertices.get(f[0]),b=m.vertices.get(f[1]),c=m.vertices.get(f[2]);double nx=(b[1]-a[1])*(c[2]-a[2])-(b[2]-a[2])*(c[1]-a[1]),ny=(b[2]-a[2])*(c[0]-a[0])-(b[0]-a[0])*(c[2]-a[2]),nz=(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);assertTrue(nx*nx+ny*ny+nz*nz>1e-15);volume+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);for(int i=0;i<3;i++){int u=f[i],v=f[(i+1)%3];String key=Math.min(u,v)+":"+Math.max(u,v);counts.put(key,counts.getOrDefault(key,0)+1);directions.put(key,directions.getOrDefault(key,0)+(u<v?1:-1));}}
            for(String key:counts.keySet()){assertEquals(kind,2,(int)counts.get(key));assertEquals(kind,0,(int)directions.get(key));}assertTrue("Outward winding "+kind,volume>0);assertEquals(2,m.vertices.size()-counts.size()+m.faces.size());assertTrue(m.obj().contains("\nf "));
            Bitmap image=CreativeEngine.render(spec(kind,n),256,0.65,0.35);assertTrue("Model visible "+kind,image.getPixel(128,128)!=0xff101c30);image.recycle();
        }
        try{new CreativeEngine.Spec("sphere",0,49,0,1);fail();}catch(IllegalArgumentException expected){}
        try{new CreativeEngine.Spec("sphere",0,24,0,Double.NaN);fail();}catch(IllegalArgumentException expected){}
    }
    public void testImagesRoundTripAndUsableExports()throws Exception{
        Context c=getInstrumentation().getTargetContext();
        for(String kind:new String[]{"mandala","landscape","vase"}){
            CreativeEngine.Spec s=spec(kind,24);assertEquals(s.json(),CreativeEngine.Spec.parse(s.json()).json());
            if(!s.mesh()){assertEquals(CreativeEngine.svg(s),CreativeEngine.svg(s));org.xmlpull.v1.XmlPullParser parser=android.util.Xml.newPullParser();parser.setInput(new StringReader(CreativeEngine.svg(s)));while(parser.next()!=org.xmlpull.v1.XmlPullParser.END_DOCUMENT){}Bitmap a=CreativeEngine.render(s,256,0,0),b=CreativeEngine.render(s,256,0,0);assertTrue(a.sameAs(b));a.recycle();b.recycle();}
            ByteArrayOutputStream out=new ByteArrayOutputStream();CreativeEngine.zip(s,out);HashMap<String,byte[]> entries=new HashMap<>();try(ZipInputStream z=new ZipInputStream(new ByteArrayInputStream(out.toByteArray()))){for(ZipEntry e;(e=z.getNextEntry())!=null;){ByteArrayOutputStream body=new ByteArrayOutputStream();byte[] chunk=new byte[4096];for(int n;(n=z.read(chunk))!=-1;)body.write(chunk,0,n);entries.put(e.getName(),body.toByteArray());}}assertEquals(4,entries.size());byte[] png=entries.get("preview.png");Bitmap image=BitmapFactory.decodeByteArray(png,0,png.length);assertNotNull(image);assertEquals(1024,image.getWidth());assertEquals(1024,image.getHeight());image.recycle();assertEquals(s.json(),new String(entries.get("project.bia-art.json"),"UTF-8"));assertTrue(entries.containsKey(s.mesh()?"model.obj":"image.svg"));
            try(FileOutputStream f=new FileOutputStream(new File(c.getExternalFilesDir(null),"creative-"+kind+".zip"))){f.write(out.toByteArray());}
        }
        try{CreativeEngine.Spec.parse("{\"schema\":\"untrusted\"}");fail();}catch(IllegalArgumentException expected){}
    }
    public void testDocumentImportExportAndOversizeRejection()throws Exception{
        Context c=getInstrumentation().getTargetContext();CreativeActivity a=(CreativeActivity)getInstrumentation().startActivitySync(new Intent(c,CreativeActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));File input=new File(c.getCacheDir(),"art-import.json"),output=new File(c.getCacheDir(),"art-export.zip");
        try{
            try(FileOutputStream f=new FileOutputStream(input)){f.write(spec("landscape",32).json().getBytes("UTF-8"));}
            getInstrumentation().runOnMainSync(()->a.onActivityResult(2,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(input))));waitWork(a);assertEquals("landscape",a.current.kind);assertEquals(32,a.current.detail);
            getInstrumentation().runOnMainSync(()->{a.pending=a.current;a.onActivityResult(1,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(output)));});waitWork(a);assertTrue(a.status.getText().toString(),a.status.getText().toString().contains("Đã xuất"));try(ZipFile z=new ZipFile(output)){assertNotNull(z.getEntry("image.svg"));}
            try(FileOutputStream f=new FileOutputStream(input)){f.write(new byte[4097]);}
            getInstrumentation().runOnMainSync(()->a.onActivityResult(2,android.app.Activity.RESULT_OK,new Intent().setData(android.net.Uri.fromFile(input))));waitWork(a);assertTrue(a.status.getText().toString().contains("Không hoàn thành"));assertEquals("landscape",a.current.kind);
        }finally{input.delete();output.delete();getInstrumentation().runOnMainSync(a::finish);}
    }
    void waitWork(CreativeActivity a){long end=android.os.SystemClock.elapsedRealtime()+10000;while(a.busy&&android.os.SystemClock.elapsedRealtime()<end)android.os.SystemClock.sleep(50);getInstrumentation().waitForIdleSync();assertFalse("Creative IO timeout",a.busy);}
    public void testStudioCreatesRejectsInvalidAndRestores()throws Exception{
        Context c=getInstrumentation().getTargetContext();CreativeActivity a=(CreativeActivity)getInstrumentation().startActivitySync(new Intent(c,CreativeActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
        getInstrumentation().runOnMainSync(()->{a.kind.setSelection(4);a.seed.setText("12345");a.detail.setText("24");a.height.setText("1.8");a.color.setText("#8A77DB");a.generate.performClick();assertEquals("vase",a.current.kind);assertEquals(12345L,a.current.seed);a.detail.setText("999999");a.generate.performClick();assertEquals(24,a.current.detail);assertTrue(a.status.getText().toString().contains("Không tạo"));a.detail.setText("24");a.finish();});
        getInstrumentation().waitForIdleSync();CreativeActivity b=(CreativeActivity)getInstrumentation().startActivitySync(new Intent(c,CreativeActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));try{assertEquals("vase",b.current.kind);assertEquals(12345L,b.current.seed);getInstrumentation().runOnMainSync(()->{Bitmap proof=CreativeEngine.render(b.current,512,0.65,0.35);try(FileOutputStream f=new FileOutputStream(new File(c.getExternalFilesDir(null),"creative-proof.png"))){proof.compress(Bitmap.CompressFormat.PNG,100,f);}catch(Exception e){throw new RuntimeException(e);}finally{proof.recycle();}});}finally{getInstrumentation().runOnMainSync(b::finish);}
    }
}
