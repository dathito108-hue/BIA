package com.bia.mobile;
final class GameNative {
    static { System.loadLibrary("bia_core"); }
    static native void reset();
    static native float[] observe(int[] pixels, int width, int height, float[] profile, long timestamp);
}
