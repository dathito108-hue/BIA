package com.bia.mobile;
import java.util.*;
import org.json.*;

/** Rigid object animation; no skeletal or deforming-mesh animation is claimed. */
public final class MotionProject {
    public static final int MAX_JSON=131072;
    public static final class Key {
        public final double time,x,y,z,yaw,scale;
        public Key(double time,double x,double y,double z,double yaw,double scale){SceneEngine.range(time,0,20);for(double v:new double[]{x,y,z})SceneEngine.range(v,-10,10);SceneEngine.range(yaw,-180,180);SceneEngine.range(scale,0.2,2);this.time=Math.rint(time*1000)/1000;this.x=x;this.y=y;this.z=z;this.yaw=yaw;this.scale=scale;}
        JSONObject json()throws JSONException{return new JSONObject().put("time",time).put("x",x).put("y",y).put("z",z).put("yaw",yaw).put("scale",scale);}
        static Key parse(JSONObject j)throws JSONException{return new Key(j.getDouble("time"),j.getDouble("x"),j.getDouble("y"),j.getDouble("z"),j.getDouble("yaw"),j.getDouble("scale"));}
    }
    public final SceneEngine.Scene base;public final List<List<Key>> tracks;
    public MotionProject(SceneEngine.Scene base,List<List<Key>> tracks){if(tracks.size()!=base.items.size())throw new IllegalArgumentException("Sai số track");ArrayList<List<Key>> copy=new ArrayList<>();for(int i=0;i<tracks.size();i++){List<Key> track=tracks.get(i);if(track.size()>8)throw new IllegalArgumentException("Tối đa 8 keyframe/vật thể");double last=-1;for(Key k:track){if(k==null||k.time<=last)throw new IllegalArgumentException("Keyframe phải tăng thời gian");last=k.time;pose(base.items.get(i),k);}copy.add(Collections.unmodifiableList(new ArrayList<>(track)));}this.base=base;this.tracks=Collections.unmodifiableList(copy);}
    public static MotionProject from(SceneEngine.Scene scene){ArrayList<List<Key>> tracks=new ArrayList<>();for(int i=0;i<scene.items.size();i++)tracks.add(Collections.emptyList());return new MotionProject(scene,tracks);}
    public MotionProject withBase(SceneEngine.Scene scene){return new MotionProject(scene,tracks);}
    public MotionProject key(int object,Key key){ArrayList<List<Key>> copy=new ArrayList<>(tracks);ArrayList<Key> list=new ArrayList<>(copy.get(object));list.removeIf(k->Math.abs(k.time-key.time)<0.000001);list.add(key);list.sort(Comparator.comparingDouble(k->k.time));copy.set(object,list);return new MotionProject(base,copy);}
    public MotionProject clear(int object){ArrayList<List<Key>> copy=new ArrayList<>(tracks);copy.set(object,Collections.emptyList());return new MotionProject(base,copy);}
    public double duration(){double end=0;for(List<Key> t:tracks)if(!t.isEmpty())end=Math.max(end,t.get(t.size()-1).time);return end;}
    public Key sample(int object,double time){SceneEngine.range(time,0,20);List<Key> keys=tracks.get(object);if(keys.isEmpty())return new Key(time,0,0,0,0,1);if(time<=keys.get(0).time)return keys.get(0);for(int i=1;i<keys.size();i++)if(time<=keys.get(i).time){Key a=keys.get(i-1),b=keys.get(i);double t=(time-a.time)/(b.time-a.time),angle=Math.toDegrees(Math.atan2(Math.sin(Math.toRadians(b.yaw-a.yaw)),Math.cos(Math.toRadians(b.yaw-a.yaw))));return new Key(time,a.x+(b.x-a.x)*t,a.y+(b.y-a.y)*t,a.z+(b.z-a.z)*t,wrap(a.yaw+angle*t),a.scale+(b.scale-a.scale)*t);}return keys.get(keys.size()-1);}
    static double wrap(double angle){return Math.toDegrees(Math.atan2(Math.sin(Math.toRadians(angle)),Math.cos(Math.toRadians(angle))));}
    public static SceneEngine.Item pose(SceneEngine.Item a,Key k){return new SceneEngine.Item(a.kind,a.detail,a.color,a.seed,a.x+k.x,a.y+k.y,a.z+k.z,a.sx*k.scale,a.sy*k.scale,a.sz*k.scale,a.rx,wrap(a.ry+k.yaw),a.rz,a.gloss,a.smooth).geometry(a.strokes,a.subdivisions);}
    public SceneEngine.Scene at(double time){ArrayList<SceneEngine.Item> items=new ArrayList<>();for(int i=0;i<base.items.size();i++)items.add(pose(base.items.get(i),sample(i,time)));return new SceneEngine.Scene(items,base.yaw,base.pitch,base.zoom,base.light,base.size,base.transparent);}
    public String json()throws JSONException{JSONArray t=new JSONArray();for(List<Key> track:tracks){JSONArray a=new JSONArray();for(Key k:track)a.put(k.json());t.put(a);}return new JSONObject().put("schema","BIA_MOTION_1").put("scene",new JSONObject(base.json())).put("tracks",t).toString(2);}
    public static MotionProject parse(String text)throws JSONException{if(text.length()>MAX_JSON)throw new IllegalArgumentException("Dự án quá lớn");JSONObject j=new JSONObject(text);if(!"BIA_MOTION_1".equals(j.getString("schema")))return from(SceneEngine.Scene.parse(text));SceneEngine.Scene scene=SceneEngine.Scene.parse(j.getJSONObject("scene").toString());JSONArray t=j.getJSONArray("tracks");if(t.length()!=scene.items.size())throw new IllegalArgumentException("Sai track");ArrayList<List<Key>> tracks=new ArrayList<>();for(int i=0;i<t.length();i++){JSONArray a=t.getJSONArray(i);if(a.length()>8)throw new IllegalArgumentException("Quá nhiều keyframe");ArrayList<Key> keys=new ArrayList<>();for(int k=0;k<a.length();k++)keys.add(Key.parse(a.getJSONObject(k)));tracks.add(keys);}return new MotionProject(scene,tracks);}
    public MotionProject sculpt(int object,Sculpt.Stroke stroke,int divisions){ArrayList<SceneEngine.Item> items=new ArrayList<>(base.items);SceneEngine.Item old=items.get(object);ArrayList<Sculpt.Stroke> strokes=new ArrayList<>(old.strokes);if(stroke!=null)strokes.add(stroke);items.set(object,old.geometry(strokes,divisions));return withBase(new SceneEngine.Scene(items,base.yaw,base.pitch,base.zoom,base.light,base.size,base.transparent));}
}
