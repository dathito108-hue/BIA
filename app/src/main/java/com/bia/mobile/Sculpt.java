package com.bia.mobile;
import java.util.*;
import org.json.*;

/** Local-space, replayable mesh edits. Topology is preserved except explicit subdivision. */
public final class Sculpt {
    public static final class Stroke {
        public final String mode;public final double x,y,z,radius,strength,dx,dy,dz;public final boolean mirror;
        public Stroke(String mode,double x,double y,double z,double radius,double strength,double dx,double dy,double dz,boolean mirror){
            if(!Arrays.asList("push","grab","smooth").contains(mode))throw new IllegalArgumentException("Cọ không hợp lệ");for(double v:new double[]{x,y,z})SceneEngine.range(v,-10,10);SceneEngine.range(radius,0.05,3);SceneEngine.range(strength,-0.3,0.3);for(double v:new double[]{dx,dy,dz})SceneEngine.range(v,-1,1);this.mode=mode;this.x=x;this.y=y;this.z=z;this.radius=radius;this.strength=strength;this.dx=dx;this.dy=dy;this.dz=dz;this.mirror=mirror;
        }
        JSONObject json()throws JSONException{return new JSONObject().put("mode",mode).put("x",x).put("y",y).put("z",z).put("radius",radius).put("strength",strength).put("dx",dx).put("dy",dy).put("dz",dz).put("mirror",mirror);}
        static Stroke parse(JSONObject j)throws JSONException{return new Stroke(j.getString("mode"),j.getDouble("x"),j.getDouble("y"),j.getDouble("z"),j.getDouble("radius"),j.getDouble("strength"),j.getDouble("dx"),j.getDouble("dy"),j.getDouble("dz"),j.getBoolean("mirror"));}
    }
    static CreativeEngine.Mesh subdivide(CreativeEngine.Mesh input){CreativeEngine.Mesh m=new CreativeEngine.Mesh();for(double[] v:input.vertices)m.v(v[0],v[1],v[2]);HashMap<Long,Integer> cache=new HashMap<>();for(int[] f:input.faces){int a=mid(m,cache,f[0],f[1]),b=mid(m,cache,f[1],f[2]),c=mid(m,cache,f[2],f[0]);m.f(f[0],a,c);m.f(a,f[1],b);m.f(c,b,f[2]);m.f(a,b,c);}return m;}
    static int mid(CreativeEngine.Mesh m,HashMap<Long,Integer> cache,int a,int b){long key=((long)Math.min(a,b)<<32)|Math.max(a,b);Integer known=cache.get(key);if(known!=null)return known;double[] x=m.vertices.get(a),y=m.vertices.get(b);int i=m.v((x[0]+y[0])/2,(x[1]+y[1])/2,(x[2]+y[2])/2);cache.put(key,i);return i;}
    static CreativeEngine.Mesh apply(CreativeEngine.Mesh m,List<Stroke> strokes,int divisions){return CoreSkills.local("sculpt.apply",()->applyCore(m,strokes,divisions));}
    static CreativeEngine.Mesh applyCore(CreativeEngine.Mesh m,List<Stroke> strokes,int divisions){for(int i=0;i<divisions;i++)m=subdivide(m);if(strokes.isEmpty())return m;ArrayList<HashSet<Integer>> adjacent=new ArrayList<>();for(int i=0;i<m.vertices.size();i++)adjacent.add(new HashSet<>());for(int[] f:m.faces)for(int i=0;i<3;i++){adjacent.get(f[i]).add(f[(i+1)%3]);adjacent.get(f[i]).add(f[(i+2)%3]);}
        for(Stroke s:strokes){double[][] normals=new double[m.vertices.size()][3];for(int[] f:m.faces){double[] n=SceneEngine.normal(m.vertices.get(f[0]),m.vertices.get(f[1]),m.vertices.get(f[2]));for(int index:f)for(int k=0;k<3;k++)normals[index][k]+=n[k];}ArrayList<double[]> next=new ArrayList<>();for(int i=0;i<m.vertices.size();i++){double[] v=m.vertices.get(i);double direct=weight(v,s.x,s.y,s.z,s.radius),mirrored=s.mirror?weight(v,-s.x,s.y,s.z,s.radius):0,w=Math.max(direct,mirrored);double[] delta=SceneEngine.unit(normals[i]);if(s.mode.equals("grab"))delta=new double[]{mirrored>direct?-s.dx:s.dx,s.dy,s.dz};else if(s.mode.equals("smooth")){delta=new double[3];for(int n:adjacent.get(i))for(int k=0;k<3;k++)delta[k]+=m.vertices.get(n)[k];if(!adjacent.get(i).isEmpty())for(int k=0;k<3;k++)delta[k]=delta[k]/adjacent.get(i).size()-v[k];}double amount=s.mode.equals("smooth")?Math.abs(s.strength):s.strength;next.add(new double[]{v[0]+delta[0]*amount*w,v[1]+delta[1]*amount*w,v[2]+delta[2]*amount*w});}m.vertices.clear();m.vertices.addAll(next);}return m;
    }
    static double weight(double[] v,double x,double y,double z,double radius){double d=Math.sqrt((v[0]-x)*(v[0]-x)+(v[1]-y)*(v[1]-y)+(v[2]-z)*(v[2]-z))/radius;return d>=1?0:(1-d*d)*(1-d*d);}
}
