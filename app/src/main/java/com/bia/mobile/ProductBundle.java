package com.bia.mobile;

import java.util.*;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.io.*;
import java.util.zip.*;
import org.json.*;

/** Bounded, checksummed source bundle. Never executes imported source or a shell. */
final class ProductBundle {
    final TreeMap<String,String> files=new TreeMap<>();
    static final Set<String> NAMES=new HashSet<>(Arrays.asList("index.html","style.css","core.js","app.js","config.js","README.md","SALES-DRAFT.md","SPEC.bia","LICENSE","NOTICE","QA.json"));
    static String hex(byte[] bytes){StringBuilder s=new StringBuilder();for(byte b:bytes)s.append(String.format(java.util.Locale.ROOT,"%02x",b&255));return s.toString();}
    static String hex(String text){return hex(text.getBytes(StandardCharsets.UTF_8));}
    static String unhex(String text){if(text.length()%2!=0 || !text.matches("[0-9a-f]*"))throw new IllegalArgumentException("Hex lỗi");byte[] b=new byte[text.length()/2];for(int i=0;i<b.length;i++)b[i]=(byte)Integer.parseInt(text.substring(2*i,2*i+2),16);return new String(b,StandardCharsets.UTF_8);}
    static String hash(String text)throws Exception{return hex(MessageDigest.getInstance("SHA-256").digest(text.getBytes(StandardCharsets.UTF_8)));}
    static ProductBundle compile(String spec)throws Exception{
        String wire=ProductNative.compile(spec);if(wire==null || wire.length()>500000)throw new IllegalStateException("Bộ sinh không trả gói hợp lệ");if(wire.startsWith("ERROR\t"))throw new IllegalArgumentException(unhex(wire.substring(6)));
        String[] lines=wire.split("\n");if(!lines[0].equals("BIA_BUNDLE_1"))throw new IllegalStateException("Sai phiên bản gói");ProductBundle b=new ProductBundle();for(int i=1;i<lines.length;i++){String[] p=lines[i].split("\t",-1);if(p.length!=2 || !NAMES.contains(p[0]) || b.files.put(p[0],unhex(p[1]))!=null)throw new IllegalStateException("Tệp gói không hợp lệ");}b.validate();return b;
    }
    void validate(){if(!files.keySet().equals(NAMES))throw new IllegalStateException("Thiếu tệp bàn giao");int total=0;for(String s:files.values())total+=s.getBytes(StandardCharsets.UTF_8).length;if(total>200000)throw new IllegalStateException("Gói quá lớn");}
    String json()throws Exception{JSONObject o=new JSONObject();for(Map.Entry<String,String> e:files.entrySet())o.put(e.getKey(),e.getValue());return o.toString();}
    static ProductBundle restore(String json)throws Exception{if(json.length()>400000)throw new IllegalArgumentException("Gói quá lớn");JSONObject o=new JSONObject(json);ProductBundle b=new ProductBundle();Iterator<String> keys=o.keys();while(keys.hasNext()){String k=keys.next();if(!NAMES.contains(k))throw new IllegalArgumentException("Tệp không được hỗ trợ");b.files.put(k,o.getString(k));}b.validate();return b;}
    String manifest()throws Exception{JSONObject m=new JSONObject(),h=new JSONObject();for(Map.Entry<String,String> e:files.entrySet())h.put(e.getKey(),hash(e.getValue()));m.put("schema","BIA_PRODUCT_MANIFEST_1");m.put("generator","V139");m.put("sha256",h);return m.toString(2);}
    void zip(OutputStream out)throws Exception{validate();try(ZipOutputStream z=new ZipOutputStream(out)){TreeMap<String,String> all=new TreeMap<>(files);all.put("MANIFEST.json",manifest());for(Map.Entry<String,String> e:all.entrySet()){ZipEntry entry=new ZipEntry(e.getKey());entry.setTime(0);z.putNextEntry(entry);z.write(e.getValue().getBytes(StandardCharsets.UTF_8));z.closeEntry();}}}
}
