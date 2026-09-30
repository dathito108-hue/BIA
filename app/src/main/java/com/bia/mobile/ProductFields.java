package com.bia.mobile;
import java.util.*;

/** Stable field identities are preserved through edits and immutable versions. */
final class ProductFields {
    static final class Field {
        String id,label,kind,options;boolean required;
        Field(String id,String label,String kind,boolean required,String options){this.id=id;this.label=label;this.kind=kind;this.required=required;this.options=options;}
        String wire(){return ProductBundle.hex(id)+"\t"+ProductBundle.hex(label)+"\t"+kind+"\t"+(required?"1":"0")+"\t"+ProductBundle.hex(options);}
    }
    static String append(String base,List<Field> fields){StringBuilder s=new StringBuilder(base.replaceFirst("BIA_PRODUCT_1","BIA_PRODUCT_2"));s.append('\n').append(fields.size());for(Field f:fields)s.append('\n').append(f.wire());return s.toString();}
    static ArrayList<Field> parse(String source){String[] lines=source.split("\n");if(lines.length<9)throw new IllegalArgumentException("Thiếu đặc tả");ArrayList<Field> fields=new ArrayList<>();if(lines[0].equals("BIA_PRODUCT_1")){if(lines.length!=9)throw new IllegalArgumentException("Đặc tả V1 hỏng");return fields;}if(!lines[0].equals("BIA_PRODUCT_2")||lines.length<10)throw new IllegalArgumentException("Sai phiên bản đặc tả");int count=Integer.parseInt(lines[9]);if(count<0||count>12||lines.length!=10+count)throw new IllegalArgumentException("Sai số trường");HashSet<String> ids=new HashSet<>();for(int i=10;i<lines.length;i++){String[] a=lines[i].split("\t",-1);if(a.length!=5||!Arrays.asList("text","number","date","choice").contains(a[2])||!Arrays.asList("0","1").contains(a[3]))throw new IllegalArgumentException("Trường hỏng");String id=ProductBundle.unhex(a[0]);if(!id.matches("f-[a-zA-Z0-9-]{1,62}")||!ids.add(id))throw new IllegalArgumentException("ID trường hỏng");fields.add(new Field(id,ProductBundle.unhex(a[1]),a[2],a[3].equals("1"),ProductBundle.unhex(a[4])));}return fields;}
}
