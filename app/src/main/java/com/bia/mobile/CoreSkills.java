package com.bia.mobile;
/** All executors enter the same Rust runtime. Tickets confer no wallet/OS permission. */
final class CoreSkills {
    static {System.loadLibrary("bia_core");}
    static native long begin(String skill);
    static native boolean finish(long id,boolean success);
    static native String report();
    interface Work<T>{T run() throws Exception;}
    interface Local<T>{T run();}
    static <T>T call(String skill,Work<T> work)throws Exception{
        long id=begin(skill);if(id<=0)throw new IllegalStateException("Lõi từ chối kỹ năng: "+skill);
        boolean ok=false;try{T result=work.run();ok=true;return result;}finally{finish(id,ok);}
    }
    static <T>T local(String skill,Local<T> work){
        long id=begin(skill);if(id<=0)throw new IllegalStateException("Lõi từ chối kỹ năng: "+skill);
        boolean ok=false;try{T result=work.run();ok=true;return result;}finally{finish(id,ok);}
    }
}
