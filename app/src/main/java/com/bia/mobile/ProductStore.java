package com.bia.mobile;
import android.content.*;
import android.database.*;
import android.database.sqlite.*;
import java.util.*;

/** Immutable builds; failed edits never overwrite a previous working version. */
final class ProductStore extends SQLiteOpenHelper {
    ProductStore(Context c){this(c,"bia-products-v1.db");}
    ProductStore(Context c,String name){super(c.getApplicationContext(),name,null,1);setWriteAheadLoggingEnabled(true);}
    public void onCreate(SQLiteDatabase db){db.execSQL("CREATE TABLE builds(id INTEGER PRIMARY KEY AUTOINCREMENT,project TEXT NOT NULL,version INTEGER NOT NULL,title TEXT NOT NULL,spec TEXT NOT NULL,bundle TEXT NOT NULL,digest TEXT NOT NULL,created INTEGER NOT NULL,UNIQUE(project,version))");}
    public void onUpgrade(SQLiteDatabase d,int a,int b){throw new IllegalStateException("Migration required");}
    synchronized long save(String project,String title,String spec,ProductBundle b)throws Exception{
        if(!project.equals(ProductBundle.unhex(spec.split("\\n")[1])))throw new IllegalArgumentException("Project/spec mismatch");
        if(!project.matches("[a-zA-Z0-9-]{8,64}") || !spec.equals(b.files.get("SPEC.bia")))throw new IllegalArgumentException("Sai định danh bản dựng");String json=b.json(),digest=ProductBundle.hash(json);SQLiteDatabase db=getWritableDatabase();db.beginTransaction();try{
            long version=1;try(Cursor c=db.rawQuery("SELECT COALESCE(MAX(version),0)+1 FROM builds WHERE project=?",new String[]{project})){c.moveToFirst();version=c.getLong(0);}try(Cursor c=db.rawQuery("SELECT COUNT(*) FROM builds",null)){c.moveToFirst();if(c.getLong(0)>=200)throw new IllegalStateException("Đã lưu 200 phiên bản; xuất và xóa phiên bản cũ trước");}
            ContentValues v=new ContentValues();v.put("project",project);v.put("version",version);v.put("title",title);v.put("spec",spec);v.put("bundle",json);v.put("digest",digest);v.put("created",System.currentTimeMillis());long id=db.insertOrThrow("builds",null,v);db.setTransactionSuccessful();return id;
        }finally{db.endTransaction();}
    }
    synchronized ProductBundle load(long id)throws Exception{try(Cursor c=getReadableDatabase().rawQuery("SELECT bundle,digest FROM builds WHERE id=?",new String[]{Long.toString(id)})){if(!c.moveToFirst())throw new IllegalStateException("Không tìm thấy phiên bản");String json=c.getString(0);if(!ProductBundle.hash(json).equals(c.getString(1)))throw new IllegalStateException("Checksum bản lưu không khớp");return ProductBundle.restore(json);}}
    synchronized ArrayList<String[]> list(){ArrayList<String[]> list=new ArrayList<>();try(Cursor c=getReadableDatabase().rawQuery("SELECT id,title,version,created FROM builds ORDER BY id DESC",null)){while(c.moveToNext())list.add(new String[]{Long.toString(c.getLong(0)),c.getString(1)+" · v"+c.getLong(2)+" · "+new Date(c.getLong(3))});}return list;}
    synchronized void delete(long id){getWritableDatabase().delete("builds","id=?",new String[]{Long.toString(id)});}
}
