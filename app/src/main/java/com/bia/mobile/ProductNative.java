package com.bia.mobile;
final class ProductNative {
    static { System.loadLibrary("bia_core"); }
    static native String compile(String spec);
    static native String compatibility(String before,String after);
}
