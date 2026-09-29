package com.bia.mobile;
final class TradingNative {
    static {System.loadLibrary("bia_core");}
    static native String quality(double[] points,long nowMs,double costBps);
    static native String analyze(double[] candles,double price,long eventMs,long nowMs,boolean connected);
}
