package com.bia.mobile;
import android.graphics.Color;
import org.json.*;
import java.nio.*;
import java.nio.charset.StandardCharsets;
import java.io.*;
import java.util.*;

/** glTF 2.0 GLB: embedded geometry, materials and rigid TRS animation. */
public final class GlbWriter {
    final ByteArrayOutputStream bin=new ByteArrayOutputStream();final JSONArray views=new JSONArray(),accessors=new JSONArray();
    int floats(float[] values,int width,String type,boolean bounds,int target)throws Exception{
        int offset=bin.size();ByteBuffer b=ByteBuffer.allocate(values.length*4).order(ByteOrder.LITTLE_ENDIAN);double[] min=new double[width],max=new double[width];Arrays.fill(min,Double.POSITIVE_INFINITY);Arrays.fill(max,-Double.MAX_VALUE);for(int i=0;i<values.length;i++){if(!Float.isFinite(values[i]))throw new IllegalArgumentException("Giá trị GLB không hữu hạn");b.putFloat(values[i]);min[i%width]=Math.min(min[i%width],values[i]);max[i%width]=Math.max(max[i%width],values[i]);}bin.write(b.array());JSONObject view=new JSONObject().put("buffer",0).put("byteOffset",offset).put("byteLength",b.capacity());if(target!=0)view.put("target",target);int v=views.length();views.put(view);JSONObject accessor=new JSONObject().put("bufferView",v).put("componentType",5126).put("count",values.length/width).put("type",type);if(bounds)accessor.put("min",new JSONArray(min)).put("max",new JSONArray(max));int a=accessors.length();accessors.put(accessor);return a;
    }
    static double[] quaternion(double rx,double ry,double rz){double x=Math.toRadians(rx)/2,y=Math.toRadians(ry)/2,z=Math.toRadians(rz)/2;double sx=Math.sin(x),cx=Math.cos(x),sy=Math.sin(y),cy=Math.cos(y),sz=Math.sin(z),cz=Math.cos(z);return new double[]{sx*cy*cz-cx*sy*sz,cx*sy*cz+sx*cy*sz,cx*cy*sz-sx*sy*cz,cx*cy*cz+sx*sy*sz};}
    static double linear(int c){double v=c/255.0;return v<=0.04045?v/12.92:Math.pow((v+0.055)/1.055,2.4);}
    public static byte[] write(MotionProject p)throws Exception{return new GlbWriter().build(p);}
    byte[] build(MotionProject p)throws Exception{
        JSONArray nodes=new JSONArray(),meshes=new JSONArray(),materials=new JSONArray(),roots=new JSONArray(),channels=new JSONArray(),samplers=new JSONArray();
        for(int i=0;i<p.base.items.size();i++){
            SceneEngine.Item item=p.base.items.get(i),initial=MotionProject.pose(item,p.sample(i,0));CreativeEngine.Mesh mesh=SceneEngine.primitive(item);double[][] normals=new double[mesh.vertices.size()][3];for(int[] f:mesh.faces){double[] n=SceneEngine.normal(mesh.vertices.get(f[0]),mesh.vertices.get(f[1]),mesh.vertices.get(f[2]));for(int v:f)for(int k=0;k<3;k++)normals[v][k]+=n[k];}for(int j=0;j<normals.length;j++)normals[j]=SceneEngine.unit(normals[j]);
            float[] positions=new float[mesh.faces.size()*9],ns=new float[positions.length];int cursor=0;for(int[] f:mesh.faces){double[] flat=SceneEngine.unit(SceneEngine.normal(mesh.vertices.get(f[0]),mesh.vertices.get(f[1]),mesh.vertices.get(f[2])));for(int v:f){double[] n=item.smooth&&!item.kind.equals("box")?normals[v]:flat;for(int k=0;k<3;k++){positions[cursor]=(float)mesh.vertices.get(v)[k];ns[cursor]=(float)n[k];cursor++;}}}
            int pos=floats(positions,3,"VEC3",true,34962),normal=floats(ns,3,"VEC3",false,34962);meshes.put(new JSONObject().put("primitives",new JSONArray().put(new JSONObject().put("attributes",new JSONObject().put("POSITION",pos).put("NORMAL",normal)).put("material",i).put("mode",4))));
            materials.put(new JSONObject().put("name","material_"+i).put("pbrMetallicRoughness",new JSONObject().put("baseColorFactor",new JSONArray(new double[]{linear(Color.red(item.color)),linear(Color.green(item.color)),linear(Color.blue(item.color)),1})).put("metallicFactor",0).put("roughnessFactor",Math.max(0.08,1-item.gloss))));
            nodes.put(new JSONObject().put("name","object_"+i).put("mesh",i).put("translation",new JSONArray(new double[]{initial.x,initial.y,initial.z})).put("rotation",new JSONArray(quaternion(initial.rx,initial.ry,initial.rz))).put("scale",new JSONArray(new double[]{initial.sx,initial.sy,initial.sz})));roots.put(i);
            List<MotionProject.Key> keys=p.tracks.get(i);if(keys.size()<2)continue;float[] times=new float[keys.size()],translation=new float[keys.size()*3],scales=new float[translation.length],rotations=new float[keys.size()*4];double[] last=null;for(int j=0;j<keys.size();j++){MotionProject.Key key=keys.get(j);SceneEngine.Item at=MotionProject.pose(item,key);times[j]=(float)key.time;translation[j*3]=(float)at.x;translation[j*3+1]=(float)at.y;translation[j*3+2]=(float)at.z;scales[j*3]=(float)at.sx;scales[j*3+1]=(float)at.sy;scales[j*3+2]=(float)at.sz;double[] q=quaternion(at.rx,at.ry,at.rz);if(last!=null&&q[0]*last[0]+q[1]*last[1]+q[2]*last[2]+q[3]*last[3]<0)for(int k=0;k<4;k++)q[k]=-q[k];for(int k=0;k<4;k++)rotations[j*4+k]=(float)q[k];last=q;}
            int input=floats(times,1,"SCALAR",true,0);int[] outputs={floats(translation,3,"VEC3",false,0),floats(rotations,4,"VEC4",false,0),floats(scales,3,"VEC3",false,0)};String[] paths={"translation","rotation","scale"};for(int k=0;k<3;k++){int sampler=samplers.length();samplers.put(new JSONObject().put("input",input).put("output",outputs[k]).put("interpolation","LINEAR"));channels.put(new JSONObject().put("sampler",sampler).put("target",new JSONObject().put("node",i).put("path",paths[k])));}
        }
        JSONObject root=new JSONObject().put("asset",new JSONObject().put("version","2.0").put("generator","BIA V142 native sculpt and motion")).put("scene",0).put("scenes",new JSONArray().put(new JSONObject().put("nodes",roots))).put("nodes",nodes).put("meshes",meshes).put("materials",materials).put("accessors",accessors).put("bufferViews",views).put("buffers",new JSONArray().put(new JSONObject().put("byteLength",bin.size())));
        if(channels.length()>0)root.put("animations",new JSONArray().put(new JSONObject().put("name","BIA motion").put("channels",channels).put("samplers",samplers)));
        byte[] json=root.toString().getBytes(StandardCharsets.UTF_8),binary=bin.toByteArray();int jlen=(json.length+3)&~3,blen=(binary.length+3)&~3;ByteBuffer out=ByteBuffer.allocate(12+8+jlen+8+blen).order(ByteOrder.LITTLE_ENDIAN);out.putInt(0x46546c67).putInt(2).putInt(out.capacity());out.putInt(jlen).putInt(0x4e4f534a).put(json);while(out.position()<20+jlen)out.put((byte)32);out.putInt(blen).putInt(0x004e4942).put(binary);return out.array();
    }
}
